// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::de::EnumAccess;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasmlib::*;

use crate::structs::*;
use crate::*;

// Defining default configuration parameters

// The default amount of players to tag an image for it to be considered processed
const DEFAULT_TAGS_REQUIRED_PER_IMAGE: i32 = 10;
// The distance required for two clusters of tags to remain separate and not merge into one
const MIN_INTER_CLUSTER_DISTANCE: f64 = 100.0;
// The percentage of players on an image that have to agree on a tag for it to be valid
const CONFIRMATION_PERCENTAGE: f32 = 0.6;

// This functions sets an owner of the smart contract, to which remaining funds can be sent to
pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.contract_creator());
}

// Smart contract ownership can be transfered
pub fn func_set_owner(_ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

// 'createGame' is used for a researcher to start a game for players to tag images, wich can then be used
// for training an Artificial Intelligence (Deep Learning). It requires the researcher to place a reward for
// the players efforts. If the players tag correctly an image, based on other plays, it gets rewarded.
// The 'createGame' function takes 2 mandatory parameters:
// - 'numberOfImages', which must be an Int32 number, and
// - 'description', which must be a String
// It can also take a non-mandatory parameter:
// - 'tagsRequiredPerImage', which must be an Int32
pub fn func_create_game(ctx: &ScFuncContext, f: &CreateGameContext) {
    // No game can be in progress in order to be able to crate a new one
    ctx.require(
        f.state.reward().value() == 0_i64,
        "Error: Game already in progress",
    );

    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = ctx.incoming();

    // Set the state variables of the game.
    f.state.processed_images().clear();
    let reward = incoming.balance(&ScColor::IOTA);
    let number_of_images = f.params.number_of_images().value();
    let description = &f.params.description().value();
    let mut tags_required_per_image = f.params.tags_required_per_image().value();

    if tags_required_per_image < 1 {
        tags_required_per_image = DEFAULT_TAGS_REQUIRED_PER_IMAGE;
    }
    f.state.reward().set_value(reward);
    f.state.number_of_images().set_value(number_of_images);
    f.state.description().set_value(description);
    f.state
        .tags_required_per_image()
        .set_value(tags_required_per_image);
    f.state.creator().set_value(&ctx.caller());

    // Reward must be at least one iota per tag
    ctx.require(
        reward >= tags_required_per_image as i64 * number_of_images as i64,
        "Error: Reward too low!",
    );

    // Now, we have to initialize the taggedimages and the playsPerImage with default values.
    let default_tagged_image = TaggedImage {
        image_id: -1,
        boost: "".to_string(),
        player: ctx.account_id(),
        x: "".to_string(),
        y: "".to_string(),
        h: "".to_string(),
        w: "".to_string(),
    };
    for i in 0..tags_required_per_image * number_of_images {
        f.state
            .tagged_images()
            .get_tagged_image(i)
            .set_value(&default_tagged_image);
    }
    for i in 0..number_of_images {
        f.state.plays_per_image().get_int32(i).set_value(0);
    }

    ctx.event(&format!(
        "game.started {0} {1} {2} {3}",
        number_of_images,
        tags_required_per_image,
        reward.to_string(),
        description
    ));
}

// This function can only be executed by the creator of the game (aka: the investigator).
// Ideally, it should be executed when all images have been tagged the required amount of times,
// although it can be executed before that. What this function does, is to arrange the tags of
// the individual images into clusters and finding outliers this way. Outliers are tags that are
// // not inside of a 'big' cluster (defined by the CONFIRMATION_PERCENTAGE constant). They represent
// a bad tag and are not considered valid. For every valid tag, it's corresponding player recieves
// a reward. This reward is the monney put into the contract by the investigator, and it gets
// split equally between valid tags. We calculate the center of the clusters and find the top best
// tags by finding the shortest distances from this tags to the center of their cluster. The
// betters that placed this tags, win the betting monney, apart from the default reward for making
// valid tags.
pub fn func_end_game(ctx: &ScFuncContext, f: &EndGameContext) {
    // The context caller has to be the game crator.
    ctx.require(
        f.state.creator().value() == ctx.caller(),
        "Error: Only the game creator can end the game",
    );

    // Also, a game has to be in progress.
    ctx.require(
        f.state.reward().value() != 0_i64,
        "Error: No game in progress",
    );

    let mut centers: Vec<Vec<TaggedImage>> = Vec::new(); // stores the center of the clusters for all images

    let number_of_images = f.state.number_of_images().value();

    // For every image, we apply Aglomerative Hierarchical Clustering
    for image in 0..number_of_images {
        let centers_in_image = find_image_centers(image, f, ctx);

        // We append the clusters coordinate to the centers vector (a vector of centers for every image)
        centers.push(centers_in_image);
    }

    // The following line, sorts the centers vector by 'image_id'
    centers.sort_by(|b, a| b[0].image_id.cmp(&a[0].image_id));

    // update the 'processed_images' state variable with the final tagging data
    for centers_in_image in centers {
        let processed_image = vec_to_tagged_image(centers_in_image, ctx);
        f.state
            .processed_images()
            .get_tagged_image(processed_image.image_id)
            .set_value(&processed_image)
    }

    // Now, we set the top players and the rewards for the correct tags
    // The betters_top vector is an ordered list of the winners, from better to worse tagger.
    let n_rewards = f.state.valid_tags().length() as i64;
    ctx.require(n_rewards > 0, "No valid tags so no rewards will be paid.");

    let individual_reward = f.state.reward().value() / n_rewards;
    let transfers: ScTransfers = ScTransfers::iotas(individual_reward);
    for i in 0..f.state.valid_tags().length() {
        let address = f
            .state
            .valid_tags()
            .get_valid_tag(i)
            .value()
            .player
            .address();
        ctx.transfer_to_address(&address, transfers);
    }

    // Now, we set the winners and the rewards for the correct tags
    // The betters_top vector is an ordered list of the winners, from better
    // to worse tagger acording to their best accuracy in the round.
    let betters_top: Vec<Better> = do_players_ranking(f, ctx);
    // Finding the total value placed in the game's bets
    let mut total_payout: i64 = 0_i64;
    for bet in &betters_top {
        total_payout += bet.amount;
    }

    // 'points' represents by how much the betting money has to be divided.
    // We have to fit the amount betted to the sum of all the prices.
    // We need "position" because if there are a number of repeated accuracies, their position must be the same.
    let mut points: i64 = 0_i64;
    let mut position = 1;
    for i in 0..betters_top.len() {
        // The prices take an exponential form, where the 'i' represents the position of the player given it's acuracy.
        // Tags that were boosted and resulted to be the players best tag, recieve a 2x or 3x boost.
        points +=
            ((position) * (position)) as i64 * betters_top[i].amount * betters_top[i].boost as i64;
        if i + 1 < betters_top.len() && betters_top[i].accuracy != betters_top[i + 1].accuracy {
            position += 1;
        }
    }

    let multiplier: f64 = total_payout as f64 / points as f64;
    position = 1;
    // here we calculate how much to betting monney to transfer to every player, and we tranfer it
    for i in 0..betters_top.len() {
        // Again, the prices take an exponential form, where 'i'
        // represents the position of the player given it's acuracy (higher is better)
        let payout = multiplier
            * ((position) * (position)) as f64
            * betters_top[i].amount as f64
            * betters_top[i].boost as f64;
        if payout < 1.0 {
            continue;
        }
        // payout may incurr in rounding errors. This errors are truncations, so no negative balance exists.

        if i + 1 < betters_top.len() && betters_top[i].accuracy != betters_top[i + 1].accuracy {
            position += 1;
        }

        let transfers: ScTransfers = ScTransfers::iotas(payout as i64);
        ctx.transfer_to_address(&betters_top[i].player.address(), transfers);
        ctx.log(&format!(
            "{ } PAID { } IOTAS TO { } Accuracy: { } Boost: { }",
            i.to_string(),
            (payout as i64).to_string(),
            betters_top[i].player.address().to_string(),
            betters_top[i].accuracy.to_string(),
            betters_top[i].boost.to_string()
        ));
    }

    // reset the players information (boost related), unless specified otherwise using reset_player_info as false
    let reset_player_info = f.params.reset_player_info();
    if !reset_player_info.exists() {
        f.state.player().clear();
    } else {
        if reset_player_info.value() == true {
            f.state.player().clear();
        }
    }

    // We clear all the state variables, so a new game can begin
    f.state.bets().clear();
    f.state.plays_per_image().clear();
    f.state.tagged_images().clear();
    f.state.reward().set_value(0_i64);
    f.state.pending_play().clear();
    f.state.valid_tags().clear();

    ctx.event(&format!("dtag.game.ended",));
}

// This function is used by players to be assigned an image and for them to place a bet on their tags.
// If an image has all the tags it requires by the 'tags_required_per_image' variable, this image can no
// longer be assigned to a player. Also, if all images have their required tags, no image can be assigned at all.
// The 'requestPlay' function takes no parameters.
pub fn func_request_play(ctx: &ScFuncContext, f: &RequestPlayContext) {
    // Defining relevant variables for the request
    let tags_required_per_image = f.state.tags_required_per_image().value();
    let number_of_images = f.state.number_of_images().value();
    let player = ctx.caller();
    let plays_per_image = f.state.plays_per_image();
    let pending_play = f.state.pending_play().get_bet(&player.address().to_string());

    // Check if the player has an open request. If it does, panic.
    if pending_play.exists() {
        panic("Error: Player already has an open request");
    }

    // Check if any images are available for the player to tag. If all are tagged the required amount of times
    // or if the ones available have been already tagged by the player, the counter will be equal to the number of images.
    let mut counter = 0;
    'image: for i in 0..number_of_images {
        if plays_per_image.get_int32(i).value() >= tags_required_per_image {
            counter += 1;
            continue;
        }
        for j in i * tags_required_per_image as i32..(i + 1) * tags_required_per_image as i32 {
            if f.state.tagged_images().get_tagged_image(j).value().image_id == -1 {
                continue;
            }
            if f.state
                .tagged_images()
                .get_tagged_image(j)
                .value()
                .player
                .address()
                == player.address()
            {
                counter += 1;
                continue 'image;
            }
        }
        break;
    }

    // If no more images are available to tag, we dont accept the request and panic.
    ctx.require(
        counter < number_of_images,
        "Error: Sorry, no more images are available to tag",
    );

    // We choose an image randomly to assign to the player for tagging.
    // This loop checks if the image has been tagged the required amount of times,
    // or if it has already been tagged by the player. If so, we choose another one.
    // Note that the loop is not infinite, as we have checked that there is at least an image available to tag.
    let mut image_id: i32;
    'outer: loop {
        image_id = ctx.random((number_of_images) as i64) as i32;
        // has the image the maximum amount of plays?
        if plays_per_image.get_int32(image_id).value() == tags_required_per_image {
            continue;
        }
        // has the image been tagged by this player before?
        for i in image_id * tags_required_per_image as i32
            ..(image_id + 1) * tags_required_per_image as i32
        {
            if f.state.tagged_images().get_tagged_image(i).value().image_id != -1 {
                if f.state
                    .tagged_images()
                    .get_tagged_image(i)
                    .value()
                    .player
                    .address()
                    == player.address()
                {
                    continue 'outer;
                }
            }
        }
        break;
    }

    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = ctx.incoming();
    let bet = Bet {
        amount: incoming.balance(&ScColor::IOTA),
        player: ctx.caller(),
        image_id: image_id,
    };

    // Append the bet data to the bets array and to the pending plays map.
    // They will automatically take care of serializing the bet struct into a bytes representation.
    let bets: ArrayOfMutableBet = f.state.bets();
    let bets_nr: i32 = bets.length();
    bets.get_bet(bets_nr).set_value(&bet);
    pending_play.set_value(&bet);

    ctx.event(&format!(
        "play.requested {0} {1} {2}",
        &bet.player.address().to_string(),
        bet.amount,
        bet.image_id,
    ));

    f.results.image_id().set_value(image_id);
}

// This function is used for a player to tag an image that has been assigned to it.
// It basically deletes the request from the request list 'pending_plays', adds the
// information of the tag to the tagged images list and adds one to the number of times
// the image has been tagged, using the 'plays_per_game' list.
// The 'sendTags' function takes 4 mandatory parameters, corresponding to the coordinates and dimentions of the tag:
// - 'x', which must be an Int64 number,
// - 'y', which must be an Int64 number,
// - 'h', which must be an Int64 number and
// - 'w', which must be an Int64 number
pub fn func_send_tags(ctx: &ScFuncContext, f: &SendTagsContext) {
    let bet = f.state.pending_play().get_bet(&ctx.caller().address().to_string());
    let tags_req_per_image = f.state.tags_required_per_image().value();
    let mut pending_play_id: i32 = 0;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Annotations {
        x: Vec<i64>,
        y: Vec<i64>,
        h: Vec<i64>,
        w: Vec<i64>,
        boost: Vec<i32>,
    }

    // convert input as strings to vectors of integers
    let input: Result<_, serde_json::Error> = serde_json::from_str(&f.params.input_json().value());
    let mut annotations_option: Option<Annotations> = None;
    match input {
        Ok(a) => annotations_option = Some(a),
        Err(e) => ctx.panic(&e.to_string()),
    }

    let annotations = annotations_option.unwrap();

    // check that boost value is allowed
    check_boost(annotations.boost.clone(), f, ctx);

    // Searching for the player's open request. If it doesn't exist, panic.
    if !bet.exists() {
        ctx.panic("Error: No plays requested for this address");
    }

    // Get the image_id and the number of times a play has been made for this image.
    let image_id = bet.value().image_id;
    let plays_per_image: i32 = f.state.plays_per_image().get_int32(image_id).value();

    // We delete the bet from the pending plays
    bet.delete();
    /*
    let mut vec_pending_plays: Vec<Bet> = Vec::new();
    for i in 0..pending_plays.length() {
        vec_pending_plays.push(f.state.pending_plays().get_bet(i).value());
    }
    f.state.pending_plays().clear();
    for i in 0..pending_play_id {
        f.state
            .pending_plays()
            .get_bet(i)
            .set_value(&vec_pending_plays[i as usize]);
    }
    for i in pending_play_id + 1..pending_plays_length {
        f.state
            .pending_plays()
            .get_bet(i - 1)
            .set_value(&vec_pending_plays[i as usize]);
    }*/

    // If the image has all it's required plays, we panic.
    // Note that the request has been removed from the pendingPlays list
    if plays_per_image >= f.state.tags_required_per_image().value() {
        ctx.panic("Error: All plays have been made for this image. Please request another one.");
    }

    // We gather all the information into this struct
    let tagged_image = TaggedImage {
        image_id: image_id,
        player: ctx.caller(),
        boost: vec32_to_str(annotations.boost.clone()),
        x: vec64_to_str(annotations.x),
        y: vec64_to_str(annotations.y),
        h: vec64_to_str(annotations.h),
        w: vec64_to_str(annotations.w),
    };

    // Add the tag data to the taggedImage array. The taggedImages array will automatically take care
    // of serializing the taggedImage struct into a bytes representation.
    f.state
        .tagged_images()
        .get_tagged_image(image_id * tags_req_per_image + plays_per_image as i32)
        .set_value(&tagged_image);

    // Add one to the number of times this image has been tagged
    let playsfor_this_image: i32 = f
        .state
        .plays_per_image()
        .get_int32(tagged_image.image_id)
        .value();
    f.state
        .plays_per_image()
        .get_int32(tagged_image.image_id)
        .set_value(playsfor_this_image + 1);

    ctx.event(&format!(
        "dtag.image.tagged {0} {1}",
        &tagged_image.player.address().to_string(),
        f.state
            .plays_per_image()
            .get_int32(tagged_image.image_id)
            .value()
            .to_string(), // nr of times the image has been tagged
    ));
}

// This function is used for a the owner of the smart contract to withdraw any
// remaining balance in the smart contract. Players can place bets that stay in
// the smart contract if it's tags are all invalid. Also, any incurring fees also
// stay in the smart contract. This function sends this funds to the owner, to pay
// for it's efforts if you will.
// The 'withdraw' function takes no parameters
pub fn func_withdraw(ctx: &ScFuncContext, f: &WithdrawContext) {
    // No game can be in progress in order to be able withdraw the remaining balance
    ctx.require(f.state.reward().value() == 0_i64, "Error: Game in progress");

    // Send any remainig funds to the owner
    let color = &ctx.balances().colors().get_color(0).value();
    let balance = ctx.balances().balance(color);

    let transfers: ScTransfers = ScTransfers::iotas(balance as i64);
    ctx.transfer_to_address(&f.state.owner().value().address(), transfers);
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_plays_per_image(ctx: &ScViewContext, f: &GetPlaysPerImageContext) {
    let image_id = f.params.image_id().value();
    let plays = f.state.plays_per_image().get_int32(image_id).value();

    ctx.log(&format!("PLAYS PER IMAGE: {0}", plays.to_string()));
    f.results.plays_per_image().set_value(plays);
}

pub fn view_get_results(ctx: &ScViewContext, f: &GetResultsContext) {
    let image_id = f.params.image_id().value();
    let tagged_image = f
        .state
        .processed_images()
        .get_tagged_image(image_id)
        .value();

    let coords = unsafe_input_tgimg_to_vecs(&tagged_image);

    let mut results: String = "".to_string();

    for i in 0..coords.len() {
        results += &format!(
            "{0} {1} {2} {3}/",
            coords[i][0], coords[i][1], coords[i][2], coords[i][3],
        );
    }

    ctx.log(&results);
    f.results.results().set_value(&results);
}

pub fn view_get_player_bets(ctx: &ScViewContext, f: &GetPlayerBetsContext) {
    struct Bet {
        player: ScAgentID,
        amount: i64,
    }

    let mut player_bets: Vec<Bet> = Vec::new();
    'bet: for i in 0..f.state.bets().length() {
        let bet = f.state.bets().get_bet(i).value();
        for player_bet in 0..player_bets.len() {
            if player_bets[player_bet].player == bet.player {
                player_bets[player_bet].amount += bet.amount;
                continue 'bet;
            }
        }
        let new_player_bet = Bet {
            player: bet.player,
            amount: bet.amount,
        };
        player_bets.push(new_player_bet);
    }

    let mut output: String = "{\n\"players\": [\n".to_string();

    let mut player_bet_strings: Vec<String> = Vec::new();
    for player_bet in player_bets {
        let mut player_bet_string: String = "".to_string();
        player_bet_string.push_str("{\n\"amount\": \"");
        player_bet_string.push_str(&player_bet.amount.to_string());
        player_bet_string.push_str("\",\n\"address\": \"");
        player_bet_string.push_str(&player_bet.player.address().to_string());
        player_bet_string.push_str("\"\n}");
        player_bet_strings.push(player_bet_string);
    }
    output.push_str(&player_bet_strings.join(",\n"));
    output.push_str("\n]\n}");

    ctx.log(&format!("{0}", output));

    f.results.player_bets().set_value(&output);
}

pub fn view_get_player_info(ctx: &ScViewContext, f: &GetPlayerInfoContext) {
    let player_address = f.params.player_address().value().to_string();
    let player = f.state.player().get_player(&player_address);
    if !player.exists() {
        ctx.panic("player not found");
    }

    let mut json = "{\n".to_string();
    json.push_str("\"n_tags\": \"");
    json.push_str(&player.value().n_tags.to_string());
    json.push_str("\",\n\"n_double_boosts\": \"");
    json.push_str(&player.value().n_double_boosts.to_string());
    json.push_str("\",\n\"n_tripple_boosts\": \"");
    json.push_str(&player.value().n_tripple_boosts.to_string());
    json.push_str("\"\n} ");

    f.results.info().set_value(&json);
    ctx.log(&json);
}

// This struct refers to a player regarding it's best accuracy play,
// the total amount betted and the boost of it's best accuracy play
#[derive(Clone)]
struct Better {
    accuracy: f64,
    player: ScAgentID,
    amount: i64,
    boost: i32,
}

impl Better {
    pub fn new(accuracy: f64, player: ScAgentID, amount: i64, boost: i32) -> Self {
        Better {
            accuracy,
            player,
            amount,
            boost,
        }
    }
}

// An internal function to check if the boost entered by a player are valid or not
fn check_boost(boost: Vec<i32>, f: &SendTagsContext, ctx: &ScFuncContext) {
    // initialize mutable variables of the player
    let mut n_double_boosts = 0;
    let mut n_tripple_boosts = 0;
    let mut n_tags = 0;
    let player_address = &ctx.caller().address().to_string();

    let player = f.state.player().get_player(player_address);
    // Fill in the variables if the player has a history
    if player.exists() {
        n_double_boosts = player.value().n_double_boosts;
        n_tripple_boosts = player.value().n_tripple_boosts;
        n_tags = player.value().n_tags;
    }

    // For every boost value, check if it's valid for the player
    for i in 0..boost.len() {
        if boost[i] == 1 {
            n_tags += 1;
        } else if boost[i] == 2 {
            if ((n_tags + 1) / 12) <= n_double_boosts {
                ctx.panic("Error: You don't have enough Drift to use a double boost");
            } else {
                n_double_boosts += 1;
                n_tags += 1;
            }
        } else if boost[i] == 3 {
            if ((n_tags + 1) / 144) <= n_tripple_boosts {
                ctx.panic("Error: You don't have enough Mana to use a tripple boost");
            } else {
                n_tripple_boosts += 1;
                n_tags += 1;
            }
        } else {
            ctx.panic("Error: invalid boost value. Must be 1, 2 or 3")
        }

        let player = Player {
            player_id: ctx.caller(),
            n_tags: n_tags,
            n_double_boosts: n_double_boosts,
            n_tripple_boosts: n_tripple_boosts,
        };

        f.state
            .player()
            .get_player(player_address)
            .set_value(&player);
    }
}

// An internal function that calculates distance between points in four dimentions (x, y, h, w)
fn euclidean_distance(a: Vec<i64>, b: Vec<i64>) -> f64 {
    (((a[0] - b[0]) * (a[0] - b[0])
        + (a[1] - b[1]) * (a[1] - b[1])
        + (a[2] - b[2]) * (a[2] - b[2])
        + (a[3] - b[3]) * (a[3] - b[3])) as f64)
        .sqrt()
}

// An internal function that takes many clusters and merges them using the Aglomerative Hierarchical Clustering
// algorithm and MIN_INTER_CLUSTER_DISTANCE as a parameter to prevent merging clusters too far apart from each other
fn clustering(mut clusters: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut min_distance = [0.0, 0.0, 0.0]; // stores [distance between two clusters, 1st cluster, 2nd cluster]

    // Here, we apply the Aglomerative Hierarchical Clustering: Merging all clusters that are the closest to each other
    // until the closest are more than MIN_INTER_CLUSTER_DISTANCE pixels⁴ or there is only one cluster left (in that
    // case, 9999999.0 will not be overwritten).
    while min_distance[0] < MIN_INTER_CLUSTER_DISTANCE {
        // Evaluate the distance matrix and store the shortest euclidean distance in 'min_distance[0]'
        min_distance[0] = 9999999.0;
        for x in 0..clusters.len() {
            for y in x + 1..clusters.len() {
                // this way we dont evaluete a pair twice, nor a cluster against itself
                let distance = euclidean_distance(clusters[x].clone(), clusters[y].clone());
                if distance < min_distance[0] {
                    min_distance = [distance, x as f64, y as f64];
                }
            }
        }
        // If the four dimentional distance is greater than MIN_INTER_CLUSTER_DISTANCE, then we dont merge the clusters.
        // Clusters that are this far apart are considered different final clusters
        if min_distance[0] < MIN_INTER_CLUSTER_DISTANCE {
            // define the indexes of the clusters one and two to be merged
            let index_1 = min_distance[1] as usize;
            let index_2 = min_distance[2] as usize;
            // the weight is equal to the number of point's that conform the cluster
            let weight_1 = (clusters[index_1].len() - 4) as i64;
            let weight_2 = (clusters[index_2].len() - 4) as i64;

            // Calculating the coordiantes of the new cluster. The more weight,
            // the more influence on the new coordinate it has. This way, the
            // coordinate represents the average of all points in the cluster
            // TODO: Divisions inside a loop are not cool. Maybe we can improve this somehow?
            let mut new_cluster = vec![
                (clusters[index_1][0] * weight_1 + clusters[index_2][0] * weight_2)
                    / (weight_1 + weight_2),
                (clusters[index_1][1] * weight_1 + clusters[index_2][1] * weight_2)
                    / (weight_1 + weight_2),
                (clusters[index_1][2] * weight_1 + clusters[index_2][2] * weight_2)
                    / (weight_1 + weight_2),
                (clusters[index_1][3] * weight_1 + clusters[index_2][3] * weight_2)
                    / (weight_1 + weight_2),
            ];
            // Push the point's inside both clusters to the new cluster
            for i in 0..weight_1 {
                new_cluster.push(clusters[index_1][i as usize + 4]);
            }
            for i in 0..weight_2 {
                new_cluster.push(clusters[index_2][i as usize + 4]);
            }

            // Remove the old clusters and replace with the new one. Note that inxex_2 > index_1.
            // When removing index_2 first, we don't alter index_1.
            clusters.remove(index_2);
            clusters.remove(index_1);
            clusters.push(new_cluster);
        }
    }

    return clusters;
}

// An internal function to calculate the average position of a tag (center) inside an image.
fn find_image_centers(image: i32, f: &EndGameContext, ctx: &ScFuncContext) -> Vec<TaggedImage> {
    let tags_req_per_image = f.state.tags_required_per_image().value();
    let mut hash_image_id: HashMap<i64, i32> = HashMap::new(); // a hashmap to retrieve an image_id from a tag_id
    let mut hash_play_tag_id: HashMap<i64, i32> = HashMap::new(); // a hashmap to retrieve an play_tag_id from a tag_id

    let mut clusters: Vec<Vec<i64>> = Vec::new(); // stores clusters with their centers and all the id's of the point's that conform it
    let mut playsfor_this_image = 0; // counts the real amount of players that tagged this image. This is because
                                     // the game could end before images are tagged with the required amount
                                     // it will be used to calculate the amount of players needed to agree for a valid tag
    for i in image * tags_req_per_image..(image + 1) * tags_req_per_image {
        // I'm forced to do this is because there are no nested arrays in schema yet
        if f.state.tagged_images().get_tagged_image(i).value().image_id == -1 {
            break;
        }
        let tagged_image = f.state.tagged_images().get_tagged_image(i).value();
        // Every 'tagged_image' starts as one cluster. The algorithm will then merge close-by clusters
        let x = input_str_to_vec64(&tagged_image.x, ctx);
        let y = input_str_to_vec64(&tagged_image.y, ctx);
        let h = input_str_to_vec64(&tagged_image.h, ctx);
        let w = input_str_to_vec64(&tagged_image.w, ctx);
        for j in 0..x.len() {
            // clusters have the following form:
            // [x, y, h, w, tag_id1. tag_id2, tag_id3, ... ],
            // where x, y, h and w are the center coordinates of the cluster
            let cluster = vec![
                x[j as usize],
                y[j as usize],
                h[j as usize],
                w[j as usize],
                clusters.len() as i64,
            ];
            hash_image_id.insert(clusters.len() as i64, i);
            hash_play_tag_id.insert(clusters.len() as i64, j as i32);
            clusters.push(cluster);
        }
        playsfor_this_image += 1;
    }

    // every tag starts as a different cluster. We merge them until they are at least
    // MIN_INTER_CLUSTER_DISTANCE pixels⁴ apart or there is only one cluster for the image.
    clusters = clustering(clusters);

    // We should be left with only one cluster (until multi-tagging is implemented).
    // The ones that have fewer points get discarted.
    // Here we also store all the players that made correct taggs. They can be stored multiple times.
    let length = clusters.len(); // clusters.len() will be changing, so we want to fix it here
    for i in 0..length {
        let id = length - i - 1; // this way it's a backwards iterator and we dont change the id's as we remove them.
        if clusters[id].len() - 4 < (playsfor_this_image as f32 * CONFIRMATION_PERCENTAGE) as usize
        {
            clusters.remove(id);
        } else {
            // here we push the players that tagged correctly to the reward-list and add the tag to valid_tags
            for j in 4..clusters[id].len() {
                let tagged_image = *hash_image_id.get(&clusters[id][j]).unwrap();
                let vaid_tag = ValidTag {
                    player: f
                        .state
                        .tagged_images()
                        .get_tagged_image(tagged_image)
                        .value()
                        .player,
                    tagged_image: tagged_image,
                    play_tag_id: *hash_play_tag_id.get(&clusters[id][j]).unwrap(),
                };
                f.state
                    .valid_tags()
                    .get_valid_tag(f.state.valid_tags().length())
                    .set_value(&vaid_tag);
            }
        }
    }

    // We want to have one cluster per image, even if it is an empty cluster. This way,
    // it's easier to find processed images based on their id. TODO: With nested arrays and nested
    // constructors, this would not be necessary.
    if clusters.len() == 0 {
        let cluster = vec![-1, -1, -1, -1];
        clusters.push(cluster);
    }

    let mut centers: Vec<TaggedImage> = Vec::new();
    for i in 0..clusters.len() {
        let center = TaggedImage {
            player: f.state.creator().value(), // the constructor requires a creator. This time it's not used tho.
            image_id: image,
            boost: 1.to_string(), // this is only the default value, same as player, and can change later
            x: clusters[i][0].to_string(),
            y: clusters[i][1].to_string(),
            h: clusters[i][2].to_string(),
            w: clusters[i][3].to_string(),
        };
        centers.push(center);
    }
    return centers;
}

// An internal function to generate a ranking of players based on their best bet.
// It uses the accuracy to calculate how good a bet is and the ranking is so that
// the best players are higher up in the returning vector.
fn do_players_ranking(f: &EndGameContext, ctx: &ScFuncContext) -> Vec<Better> {
    // 'valid_bets' stores all the bets placed, including zero value ones (with the player,
    // the accuracy of the tag and, for the moment, a total bet equal to zero)
    let mut valid_bets: Vec<Better> = Vec::new();
    // fill the 'valid_bets' with the bets. The bet amount will be filled later
    for i in 0..f.state.valid_tags().length() as usize {
        let valid_tag = f.state.valid_tags().get_valid_tag(i as i32).value();
        let player_tag_id = valid_tag.play_tag_id as usize;
        let tagged_image = f
            .state
            .tagged_images()
            .get_tagged_image(valid_tag.tagged_image)
            .value();
        let tagged_image_coords = input_tgimg_to_vecs(&tagged_image, ctx);
        let tagged_image_point = &tagged_image_coords[player_tag_id];
        let boost = input_str_to_vec32(&tagged_image.boost, ctx)[player_tag_id];
        let clusters_centers = f
            .state
            .processed_images()
            .get_tagged_image(tagged_image.image_id)
            .value();
        let cluster_center_coords = input_tgimg_to_vecs(&clusters_centers, ctx);
        let mut distance_to_cluster_center =
            euclidean_distance(tagged_image_point.clone(), cluster_center_coords[0].clone());
        for j in 1..cluster_center_coords.len() {
            let cluster_center_point = &cluster_center_coords[j];
            let distance =
                euclidean_distance(tagged_image_point.to_vec(), cluster_center_point.to_vec());
            if distance < distance_to_cluster_center {
                distance_to_cluster_center = distance;
            }
        }
        valid_bets.push(Better::new(
            distance_to_cluster_center,
            valid_tag.clone().player,
            0,
            boost,
        ));
    }

    // Next, we make a list with all the betters that made a valid tag, leaving their best one
    // and calculating how much they betted in total
    let mut betters_top: Vec<Better> = Vec::new();
    'all: for i in 0..valid_bets.len() {
        let valid_bet = valid_bets[i].clone();

        for better in 0..betters_top.len() {
            if valid_bet.player == betters_top[better].player {
                // replace the accuracy for the player's best one
                if valid_bet.accuracy < betters_top[better].accuracy {
                    betters_top[better].boost = valid_bet.boost;
                    betters_top[better].accuracy = valid_bet.accuracy;
                }

                // skip to next iteration of the outer loop to avoid adding the player to the 'betters_top' again
                continue 'all;
            }
        }
        let new_better = Better {
            accuracy: valid_bet.accuracy,
            amount: valid_bet.amount,
            boost: valid_bet.boost,
            player: valid_bet.player,
        };

        betters_top.push(new_better);
    }

    // Here we calculate the amount of iotas betted by each player in the 'betters_top' list
    'bet: for i in 0..f.state.bets().length() {
        let bet = f.state.bets().get_bet(i).value();
        for better in 0..betters_top.len() {
            if betters_top[better].player == bet.player {
                betters_top[better].amount += bet.amount;
                continue 'bet;
            }
        }
    }

    // sort the 'top_betters' by accuracy
    betters_top.sort_by(|a, b| b.accuracy.partial_cmp(&a.accuracy).unwrap());

    return betters_top;
}

// An internal function to convert inputs to the smart contract as strings that contain
// int64 variables separated by spaces into a vector of those int64 variables.
fn input_str_to_vec64(string: &String, ctx: &ScFuncContext) -> Vec<i64> {
    let iterator = string.split_whitespace();
    let mut vec64: Vec<i64> = Vec::new();
    for i in iterator {
        let input = i.parse::<i64>();
        match input {
            Ok(integer) => vec64.push(integer),
            Err(_error) => {
                ctx.panic("Error: Input couldn't be decoded correctly. Must be an int64.")
            }
        }
    }
    return vec64;
}

// An internal function to convert inputs to the smart contract as strings that contain
// int32 variables separated by spaces into a vector of those int32 variables.
fn input_str_to_vec32(string: &String, ctx: &ScFuncContext) -> Vec<i32> {
    let iterator = string.split_whitespace();
    let mut vec32: Vec<i32> = Vec::new();
    for i in iterator {
        let input = i.parse::<i32>();
        match input {
            Ok(integer) => vec32.push(integer),
            Err(_error) => {
                ctx.panic("Error: Input couldn't be decoded correctly. Must be an int32.")
            }
        }
    }
    return vec32;
}

// An internal function to convert a vector of i32 variables into a string
// containing those variables, separated by spaces
fn vec32_to_str(vec32: Vec<i32>) -> String {
    let mut string = "".to_string();
    for i in 0..vec32.len() {
        if i != 0 {
            string += " ";
        }
        string += &((vec32[i] as u8).to_string());
    }
    return string;
}

// An internal function to convert a vector of i32 variables into a string
// containing those variables, separated by spaces
fn vec64_to_str(vec64: Vec<i64>) -> String {
    let mut string = "".to_string();
    for i in 0..vec64.len() {
        if i != 0 {
            string += " ";
        }
        string += &vec64[i].to_string();
    }
    return string;
}

// An internal function to convert a vector of tagged images, each with a single point
// to one single tagged image but with many points
fn vec_to_tagged_image(vec: Vec<TaggedImage>, ctx: &ScFuncContext) -> TaggedImage {
    let mut x: Vec<i64> = Vec::new();
    let mut y: Vec<i64> = Vec::new();
    let mut h: Vec<i64> = Vec::new();
    let mut w: Vec<i64> = Vec::new();
    let mut boost: Vec<i32> = Vec::new();
    for point in &vec {
        x.push(input_str_to_vec64(&point.x, ctx)[0]);
        y.push(input_str_to_vec64(&point.y, ctx)[0]);
        h.push(input_str_to_vec64(&point.h, ctx)[0]);
        w.push(input_str_to_vec64(&point.w, ctx)[0]);
        boost.push(input_str_to_vec32(&point.boost, ctx)[0]);
    }
    let processed_image = TaggedImage {
        image_id: (&vec[0]).image_id,
        player: ctx.caller(), // field is required but not used in this case
        x: vec64_to_str(x),
        y: vec64_to_str(y),
        h: vec64_to_str(h),
        w: vec64_to_str(w),
        boost: vec32_to_str(boost),
    };

    return processed_image;
}

// An internal function to get a vector of vectors of type i64, each vector representing
// a dimention and each dimention having multiple points. This is calculated taking a
// reference to a TaggedImage as an input.
fn input_tgimg_to_vecs(tagged_image: &TaggedImage, ctx: &ScFuncContext) -> Vec<Vec<i64>> {
    let x = input_str_to_vec64(&tagged_image.x, ctx);
    let y = input_str_to_vec64(&tagged_image.y, ctx);
    let h = input_str_to_vec64(&tagged_image.h, ctx);
    let w = input_str_to_vec64(&tagged_image.w, ctx);

    let mut vectors: Vec<Vec<i64>> = Vec::new();

    for i in 0..x.len() {
        vectors.push(vec![x[i], y[i], h[i], w[i]]);
    }
    return vectors;
}

// An internal function to convert inputs to the smart contract as strings that contain
// int64 variables separated by spaces into a vector of those int64 variables.
// CAUTION: inputs MUST BE of type i64. Else, the error will not be handeled correctly.
// This function takes no ctx, so no panic can be induced.
fn unsafe_input_str_to_vec64(string: &String) -> Vec<i64> {
    let iterator = string.split_whitespace();
    let mut vec64: Vec<i64> = Vec::new();
    for i in iterator {
        let input = i.parse::<i64>();
        vec64.push(input.unwrap())
    }
    return vec64;
}

// An internal function to get a vector of vectors of type i64, each vector representing
// a dimention and each dimention having multiple points. This is calculated taking a
// reference to a TaggedImage as an input.
// CAUTION: inputs MUST BE of type i64. Else, the error will not be handeled correctly.
// This function takes no ctx, so no panic can be induced.
fn unsafe_input_tgimg_to_vecs(tagged_image: &TaggedImage) -> Vec<Vec<i64>> {
    let x = unsafe_input_str_to_vec64(&tagged_image.x);
    let y = unsafe_input_str_to_vec64(&tagged_image.y);
    let h = unsafe_input_str_to_vec64(&tagged_image.h);
    let w = unsafe_input_str_to_vec64(&tagged_image.w);

    let mut vectors: Vec<Vec<i64>> = Vec::new();

    for i in 0..x.len() {
        vectors.push(vec![x[i], y[i], h[i], w[i]]);
    }
    return vectors;
}
