// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryFrom;

use serde::de::EnumAccess;
use serde::{Deserialize, Serialize};
use wasmlib::*;

use crate::contract::*;
use crate::structs::*;
use crate::utility_funcs::*;
use crate::*;

// Defining default configuration parameters

// The default amount of players to tag an image for it to be considered processed
pub const DEFAULT_TAGS_REQUIRED_PER_IMAGE: u32 = 10;
// The distance required for two clusters of tags to remain separate and not merge into one
pub const MIN_INTER_CLUSTER_DISTANCE: f64 = 100.0;
// The percentage of players on an image that have to agree on a tag for it to be valid. It is highly recomended for it to be > 0.5
pub const CONFIRMATION_PERCENTAGE: f32 = 0.6;

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
        f.state.reward().value() == 0_u64,
        "Error: Game already in progress",
    );

    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = ctx.incoming();

    // Set the state variables of the game.
    f.state.processed_images().clear();
    let reward = incoming.balance(&ScColor::IOTA) as u64;
    let number_of_images = f.params.number_of_images().value();
    let description = &f.params.description().value();
    let mut tags_required_per_image = DEFAULT_TAGS_REQUIRED_PER_IMAGE;
    if f.params.tags_required_per_image().exists() {
        tags_required_per_image = f.params.tags_required_per_image().value();
    }
    f.state.reward().set_value(reward);
    f.state.number_of_images().set_value(number_of_images);
    f.state.description().set_value(description);
    f.state.plays_required_per_image().set_value(tags_required_per_image);
    f.state.creator().set_value(&ctx.caller());

    // Reward must be at least one iota per tag
    ctx.require(
        reward >= tags_required_per_image as u64 * number_of_images as u64,
        "Error: Reward too low!",
    );

    // Now, we have to initialize the tagged images (TgdImg) and the playsPerImage with default values.
    let default_tgd_img = TgdImg {
        image_id: -1,
        boost: "".to_string(),
        player: ctx.account_id().address(),
        x: "".to_string(),
        y: "".to_string(),
        h: "".to_string(),
        w: "".to_string(),
    };
    for _ in 0..tags_required_per_image * number_of_images {
        f.state
            .tgd_imgs()
            .append_tgd_img()
            .set_value(&default_tgd_img);
    }
    for _ in 0..number_of_images {
        f.state.plays_per_image().append_uint32().set_value(0);
    }

    f.events.game_started(
        description,
        number_of_images,
        reward,
        tags_required_per_image,
    );
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
    // unless all images are completed with it's required plays,
    // the context caller has to be the game crator.
    // As automatic endGame() is not implemented, complete_images() is never increased.
    if ! f.state.complete_images().value() == f.state.plays_required_per_image().value() {
        ctx.require(
            f.state.creator().value() == ctx.caller(),
            "Error: Only the game creator can end the game",
    );
    }

    // Also, a game has to be in progress.
    ctx.require(
        f.state.reward().value() != 0_u64,
        "Error: No game in progress",
    );

    let mut centers: Vec<Vec<TgdImg>> = Vec::new(); // stores the center of the clusters for all images

    let number_of_images = f.state.number_of_images().value();

    // For every image, we apply Aglomerative Hierarchical Clustering
    for image_id in 0..number_of_images {
        let centers_in_image = find_image_centers(image_id, f, ctx);

        // We append the clusters coordinate to the centers vector (a vector of centers for every image)
        centers.push(centers_in_image);
    }

    // The following line, sorts the centers vector by 'image_id'
    centers.sort_by(|b, a| b[0].image_id.cmp(&a[0].image_id));

    // update the 'processed_images' state variable with the final tagging data
    for image_id in 0..number_of_images as usize {
        let centers_in_image = centers[image_id].clone();
        let processed_image = vec_to_tgd_img(centers_in_image, ctx);
        f.state
            .processed_images()
            .append_tgd_img()
            .set_value(&processed_image)
    }

    // Now, we set the top players and the rewards for the correct tags
    // The betters_top vector is an ordered list of the winners, from better to worse tagger.
    let n_rewards = f.state.valid_tags().length() as u64;
    ctx.require(n_rewards > 0, "No valid tags so no rewards can be paid.");

    for i in 0..f.state.players_info().length() {
        // get player address
        let address = f
            .state
            .players_info()
            .get_string(i)
            .value();
        
        let mut player_info = f.state.player_info().get_player_info(&address).value();
        let valid_tags = f.state.reward().value() * player_info.n_valid_tags;
        let payout = (valid_tags) / n_rewards;
        if payout == 0 { continue }
        ctx.log(&format!("Rewarding {}i to {} for doing {} correct annotations.", payout, address, valid_tags));

        let d = coreaccounts::ScFuncs::deposit(ctx);
        d.params.agent_id().set_value(&player_info.player);
        d.func.transfer_iotas(payout).call();

        player_info.n_valid_tags = 0;
        f.state.player_info().get_player_info(&address.to_string()).set_value(&player_info);
    }

    // Now, we set the winners and the rewards for the correct tags
    // The betters_top vector is an ordered list of the winners, from better
    // to worse tagger acording to their best accuracy in the round.
    let betters_top: Vec<Better> = do_players_ranking(f, ctx);
    // Finding the total value placed in the game's bets
    let mut total_payout: i64 = 0_i64;
    for bet in &betters_top {
        total_payout += bet.amount as i64;
    }

    // 'points' represents by how much the betting money has to be divided.
    // We have to fit the amount betted to the sum of all the prices.
    // We need "position" because if there are a number of repeated accuracies, their position must be the same.
    let mut points: u64 = 0_u64;
    let mut position = 1;
    for i in 0..betters_top.len() {
        // The prices take an exponential form, where the 'position' represents the position of the player given it's acuracy.
        // Tags that were boosted and resulted to be the players best tag, recieve a 2x or 3x boost.
        points +=
            ((position) * (position)) as u64 * betters_top[i].amount * betters_top[i].boost as u64;
        if i + 1 < betters_top.len() && betters_top[i].accuracy != betters_top[i + 1].accuracy {
            position += 1;
        }
    }

    position = 1;
    // here we calculate how much to betting monney to transfer to every player, and we tranfer it
    for i in 0..betters_top.len() {
        // Again, the prices take an exponential form, where the position of the player is given by it's acuracy (higher is better)
        let payout = (total_payout as f64
            * ((position) * (position)) as f64
            * betters_top[i].amount as f64
            * betters_top[i].boost as f64)
            / points as f64;
        if payout < 1.0 {
            continue;
        }
        // payout may incurr in rounding errors. This errors are truncations, so no negative balance exists.

        if i + 1 < betters_top.len() && betters_top[i].accuracy != betters_top[i + 1].accuracy {
            position += 1;
        }

        let d = coreaccounts::ScFuncs::deposit(ctx);
        d.params.agent_id().set_value(&betters_top[i].player.as_agent_id());
        d.func.transfer_iotas(payout as u64).call();

        /*
        let transfers: ScTransfers = ScTransfers::iotas(payout as u64);
        let parms = ScDict::new(&[]);
        parms.set(&string_to_bytes("a"), &betters_top[i].player.as_agent_id().to_bytes());
        ctx.call(ScHname::new("accounts".as_bytes()), ScHname::new("deposit".as_bytes()), Some(parms), Some(transfers));
        */
        f.events.paid(
            &betters_top[i].accuracy.to_string(),
            payout as u64,
            betters_top[i].amount,
            betters_top[i].boost,
            &f.params.mission().value(),
            &betters_top[i].player.to_string(),
            (betters_top.len()-i) as u64,
        );
        ctx.log(&format!(
            "{ } PAID { } IOTAS TO { } Accuracy: { } Boost: { }",
            i.to_string(),
            (payout as i64).to_string(),
            betters_top[i].player.to_string(),
            betters_top[i].accuracy.to_string(),
            betters_top[i].boost.to_string()
        ));
    }

    // reset the players information (boost related), unless specified otherwise using reset_player_info as false
    let reset_player_info = f.params.reset_player_info();
    if !reset_player_info.exists() {
        clear_player(f);
    } else {
        if reset_player_info.value() == true {
            clear_player(f);
        }
    }

    // We clear necessary state variables, so a new game can begin
    f.state.bets().clear();
    f.state.plays_per_image().clear();
    f.state.tgd_imgs().clear();
    f.state.reward().set_value(0_u64);
    f.state.valid_tags().clear();
    f.state.complete_images().delete();
    clear_pending_play(f);

    f.events.game_ended(
        &f.params.mission().value(),
    );
}

// This function is used by players to be assigned an image and for them to place a bet on their tags.
// If an image has all the tags it requires by the 'tags_required_per_image' variable, this image can no
// longer be assigned to a player. Also, if all images have their required tags, no image can be assigned at all.
// The 'requestPlay' function takes no parameters.
pub fn func_request_play(ctx: &ScFuncContext, f: &RequestPlayContext) {
    // A game has to be in progress.
    ctx.require(
        f.state.reward().value() != 0_u64,
        "Error: No game in progress",
    );
    // Defining relevant variables for the request
    let plays_required_per_image = f.state.plays_required_per_image().value();
    let number_of_images = f.state.number_of_images().value();
    let player = ctx.caller();
    let plays_per_image = f.state.plays_per_image();
    let pending_play = f
        .state
        .pending_play()
        .get_bet(&player.address().to_string());

    // Check if the player has an open request. If it does, panic.
    if pending_play.exists() {
        panic("Error: Player already has an open request");
    }

    // Check if the amount betted doesn't exceed the maximum allowed, established by the ingots
    check_ingots(ctx.incoming().balance(&ScColor::IOTA), f,  ctx);

    // Check if any images are available for the player to tag. If all are tagged the required amount of times
    // or if the ones available have been already tagged by the player, the counter will be equal to the number of images.
    let mut counter = 0;
    'image: for i in 0..number_of_images {
        if plays_per_image.get_uint32(i).value() >= plays_required_per_image {
            counter += 1;
            continue;
        }
        for j in i * plays_required_per_image as u32..(i + 1) * plays_required_per_image as u32 {
            let tgd_img = f.state.tgd_imgs().get_tgd_img(j).value();
            if tgd_img.image_id == -1 {
                continue;
            }
            if tgd_img.player == player.address()
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
    let mut image_id: u32;
    'outer: loop {
        image_id = ctx.random((number_of_images) as u64) as u32;
        // has the image the maximum amount of plays?
        if plays_per_image.get_uint32(image_id).value() == plays_required_per_image {
            continue;
        }
        // has the image been tagged by this player before?
        for i in image_id * plays_required_per_image
            ..(image_id + 1) * plays_required_per_image
        {
            if f.state.tgd_imgs().get_tgd_img(i).value().image_id != -1 {
                if f.state
                    .tgd_imgs()
                    .get_tgd_img(i)
                    .value()
                    .player
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
        amount: incoming.balance(&ScColor::IOTA) as u64,
        player: ctx.caller().address(),
        image_id: image_id as u32,
    };

    add_bet(ctx, None, Some(f), image_id);

    pending_play.set_value(&bet);
    f.state
        .pending_plays()
        .append_bet()
        .set_value(&bet);

    f.events
        .play_requested(&bet.player.to_string(), bet.amount, bet.image_id);

    f.results.image_id().set_value(image_id);
}

// This function is used for a player to tag an image that has been assigned to it.
// It basically deletes the request from the request map 'pending_play', adds the
// information of the tag to the tagged images list and adds one to the number of times
// the image has been tagged, using the 'plays_per_game' list.
// The 'sendTags' function takes 1 mandatory parameter, corresponding to a json
// of x, y, h, w coordinates and boosts of the annotations in the image:
// { "x": [], "y": [], "h":[], "w":[], "boost":[]}
// Each of the lists for evey dimention has as many elements as annotations the user made in the image.
pub fn func_send_tags(ctx: &ScFuncContext, f: &SendTagsContext) {
    let bet = f
        .state
        .pending_play()
        .get_bet(&ctx.caller().address().to_string());
    let plays_req_per_image = f.state.plays_required_per_image().value();

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Annotations {
        x: Vec<f64>,
        y: Vec<f64>,
        h: Vec<f64>,
        w: Vec<f64>,
        boost: Vec<u8>,
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
    let image_id = bet.value().image_id as i32;
    let plays_per_image: u32 = f.state.plays_per_image().get_uint32(image_id as u32).value();

    // We delete the bet from the pending plays
    bet.delete();

    // If the image has all it's required plays, we panic.
    // Note that the request has been removed from the pendingPlays list
    if plays_per_image >= plays_req_per_image {
        ctx.panic("Error: All plays have been made for this image. Please request another one.");
    }


    add_bet(ctx, Some(f), None, image_id as u32);

    // We gather all the information into this struct
    let tgd_img = TgdImg {
        image_id: image_id,
        player: ctx.caller().address(),
        boost: vecu8_to_str(annotations.boost.clone()),
        x: vecf64_to_str(annotations.x),
        y: vecf64_to_str(annotations.y),
        h: vecf64_to_str(annotations.h),
        w: vecf64_to_str(annotations.w),
    };

    // Add the tag data to the tagged image (TgdImg) array. The tagged images array will automatically take care
    // of serializing the tagged image struct into a bytes representation.
    f.state
        .tgd_imgs()
        .get_tgd_img(image_id as u32 * plays_req_per_image + plays_per_image)
        .set_value(&tgd_img);

    // Get the number of times this image has been tagged and add one to it
    f.state
        .plays_per_image()
        .get_uint32(tgd_img.image_id as u32)
        .set_value(plays_per_image + 1);

    f.events.imagetagged(
        &tgd_img.player.to_string(),
        image_id as u32,
        f.state
            .plays_per_image()
            .get_uint32(tgd_img.image_id as u32)
            .value(), // nr of times the image has been tagged, plays_per_image)
    );

    // Now we just request the next image to save us time (:
    // But first, we need to check if the request is possible
    // in order not to revert the entire transaction in case it's not
    
    internal_request_play(f, ctx);
    

    /* ((IN CASE WE IMPLEMENT AUTOMATIC GAME ENDING IN THE FUTURE))
    // if this image is now played as many times as it is required, add one to the completeImages state variable
    if plays_per_image + 1 == plays_req_per_image {
        let complete_images = f.state.complete_images().value();
        ctx.log("Ending game automatically");

        // If all images have been played the required amount of times, add one to the completeImages state variable
        // the game can no longer recieve plays, so it will come to an end.
        if complete_images +1 == f.state.number_of_images().value() {
            let end_game = ScFuncs::end_game(ctx);
            end_game.params.reset_player_info().set_value(false);
            end_game.func
                .transfer_iotas(1)
                .post();
        }
        f.state.complete_images().set_value(complete_images + 1);
    }*/
}

// This function is used for a the owner of the smart contract to withdraw any
// remaining balance in the smart contract. Players can place bets that stay in
// the smart contract if it's tags are all invalid. Also, any incurring fees also
// stay in the smart contract. This function sends this funds to the owner, to pay
// for it's efforts if you will.
// The 'withdraw' function takes no parameters
pub fn func_withdraw(ctx: &ScFuncContext, f: &WithdrawContext) {
    // No game can be in progress in order to be able withdraw the remaining balance
    ctx.require(f.state.reward().value() == 0_u64, "Error: Game in progress");

    // Send any remainig funds to the owner
    let balance = ctx.balances().balance(&ScColor::IOTA);

    let transfers: ScTransfers = ScTransfers::iotas(balance);
    ctx.transfer_to_address(&f.state.owner().value().address(), transfers);
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_plays_per_image(ctx: &ScViewContext, f: &GetPlaysPerImageContext) {
    let image_id = f.params.image_id().value();
    let plays = f.state.plays_per_image().get_uint32(image_id).value();

    ctx.log(&format!("PLAYS PER IMAGE: {0}", plays.to_string()));
    f.results.plays_per_image().set_value(plays);
}

pub fn view_get_results(ctx: &ScViewContext, f: &GetResultsContext) {
    let image_id = f.params.image_id().value();
    let tgd_img = f
        .state
        .processed_images()
        .get_tgd_img(image_id)
        .value();

    let coords = unsafe_input_tgimg_to_vecs(&tgd_img);

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
        player: ScAddress,
        amount: u64,
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
        player_bet_string.push_str(&player_bet.player.to_string());
        player_bet_string.push_str("\"\n}");
        player_bet_strings.push(player_bet_string);
    }
    output.push_str(&player_bet_strings.join(",\n"));
    output.push_str("\n]\n}");

    ctx.log(&format!("{}", output));

    f.results.player_bets().set_value(&output);
}

pub fn view_get_player_info(ctx: &ScViewContext, f: &GetPlayerInfoContext) {
    let player_address = f.params.player_address().value().to_string();

    // In case player_ingot map exists for this address, get its value. Else, leave it as zero.
    let mut total_player_tags = 0;
    if f.state.
    total_player_tags().
    get_uint64(&player_address).
    exists()
    {
        total_player_tags = f.state.total_player_tags().get_uint64(&player_address).value();
    }
    
    // In case player_info map exists for this address, get its value. Else, leave it as zero.
    let mut player_info = PlayerInfo {
        player: ctx.contract_creator(), // just a default value
        n_double_boosts: 0,
        n_tripple_boosts: 0,
        n_tags: 0,
        n_valid_tags: 0
    };
    if f.state
        .player_info()
        .get_player_info(&player_address)
        .exists()
    {
        player_info = f
            .state
            .player_info()
            .get_player_info(&player_address)
            .value()
    }

    let mut json = "{\n".to_string();
    json.push_str("\"round_n_tags\": \"");
    json.push_str(&player_info.n_tags.to_string());
    json.push_str("\",\n\"n_double_boosts\": \"");
    json.push_str(&player_info.n_double_boosts.to_string());
    json.push_str("\",\n\"n_tripple_boosts\": \"");
    json.push_str(&player_info.n_tripple_boosts.to_string());
    json.push_str("\",\n\"total_n_tags\": \"");
    json.push_str(&total_player_tags.to_string());
    json.push_str("\"\n} ");

    f.results.info().set_value(&json);
    ctx.log(&json);
}