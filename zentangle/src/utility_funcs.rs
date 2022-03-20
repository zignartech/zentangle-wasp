// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
#![allow(unused_imports)]
use wasmlib::*;
use wasmlib::host::*;
use std::collections::HashMap;
use crate::*;
use crate::structs::*;


// This struct refers to a player regarding it's best accuracy play,
// the total amount betted and the boost of it's best accuracy play
#[derive(Clone)]
pub struct Better {
    pub accuracy: f64,
    pub player: ScAddress,
    pub amount: u64,
    pub boost: u8,
}

impl Better {
    pub fn new(accuracy: f64, player: ScAddress, amount: u64, boost: u8) -> Self {
        Better {
            accuracy,
            player,
            amount,
            boost,
        }
    }
}

// An internal function to check if the boost entered by a player are valid or not
pub fn check_boost(boost: Vec<u8>, f: &SendTagsContext, ctx: &ScFuncContext) {
    // initialize mutable variables of the player
    let mut total_player_tags = 0;
    let mut n_double_boosts = 0;
    let mut n_tripple_boosts = 0;
    let mut n_tags = 0;
    let player_address = &ctx.caller().address().to_string();

    let sc_total_player_tags = f.state.total_player_tags().get_uint64(player_address);
    // get the total tags made by the player if it has any
    if sc_total_player_tags.exists() {
        total_player_tags = sc_total_player_tags.value();
    }

    let player_boost = f.state.player_boost().get_player_boost(player_address);
    // Fill in the variables if the player has a history
    if player_boost.exists() {
        n_double_boosts = player_boost.value().n_double_boosts;
        n_tripple_boosts = player_boost.value().n_tripple_boosts;
        n_tags = player_boost.value().n_tags;
    }

    // For every boost value, check if it's valid for the player
    for i in 0..boost.len() {
        if boost[i] == 1 {
            n_tags += 1;
            total_player_tags += 1;
        } else if boost[i] == 2 {
            if ((n_tags + 1) / 12) <= n_double_boosts {
                ctx.panic("Error: You don't have enough Drift to use a double boost");
            } else {
                n_double_boosts += 1;
                total_player_tags += 1;
                n_tags += 1;
            }
        } else if boost[i] == 3 {
            if ((n_tags + 1) / 144) <= n_tripple_boosts {
                ctx.panic("Error: You don't have enough Mana to use a tripple boost");
            } else {
                n_tripple_boosts += 1;
                n_tags += 1;
                total_player_tags += 1;
            }
        } else {
            ctx.panic("Error: invalid boost value. Must be 1, 2 or 3")
        }

        let player = PlayerBoost {
            player: ctx.caller(),
            n_tags: n_tags,
            n_double_boosts: n_double_boosts,
            n_tripple_boosts: n_tripple_boosts,
            n_valid_tags: 0
        };

        // if the player isn't on the players list, add it
        if !f.state.player_boost().get_player_boost(player_address).exists() {
            f.state
            .players_boost()
            .get_string(f.state.players_boost().length())
            .set_value(&player.player.address().to_string());
        }
        // update player's boost data on map
        f.state
            .player_boost()
            .get_player_boost(player_address)
            .set_value(&player);
        f.state.total_player_tags().get_uint64(player_address).set_value(total_player_tags);
    }
}

// An internal function to check if the amount betted doesn't exceed the maximum allowed
pub fn check_ingots(amount_betted: i64, f: &RequestPlayContext, ctx: &ScFuncContext) {
    let miota: i64 = 1000000;
    if amount_betted <= 60*miota { return } // Everyone can bet 60 Mi or less

    let player_address = &ctx.caller().address().to_string();
    let mut total_player_tags = 0;
    if f.state.total_player_tags().get_uint64(player_address).exists() {
        total_player_tags = f.state.total_player_tags().get_uint64(player_address).value();
    }
    let ingots = total_player_tags / 576;

    if amount_betted <= 180*miota {
        ctx.require(ingots >= 1, "Error: Not enough ingots");
    }
    else if amount_betted <= 480*miota {
        ctx.require(ingots >= 2, "Error: Not enough ingots");
    }
    else if amount_betted <= 780*miota {
        ctx.require(ingots >= 4, "Error: Not enough ingots");
    }
    else if amount_betted <= 1260*miota {
        ctx.require(ingots >= 6, "Error: Not enough ingots");
    }
    else if amount_betted <= 2040*miota {
        ctx.require(ingots >= 8, "Error: Not enough ingots");
    }
    else if amount_betted <= 3300*miota {
        ctx.require(ingots >= 10, "Error: Not enough ingots");
    }
    else {
        ctx.require(ingots >= 12, "Error: Not enough ingots");
    }
}

// An internal function to clear the player map and the players list
pub fn clear_player(f: &EndGameContext) {
    for player_id in 0..f.state.players_boost().length() {
        let player_address = f
            .state
            .players_boost()
            .get_string(player_id)
            .value();
        let player = f.state.player_boost().get_player_boost(&player_address);
        if player.exists() {
            player.delete();
        }
    }
    f.state.players_boost().clear();
}

// An internal function to clear the pending_play map and the pending_plays list
pub fn clear_pending_play(f: &EndGameContext) {
    for player_id in 0..f.state.pending_plays().length() {
        let player_address = f
            .state
            .pending_plays()
            .get_bet(player_id)
            .value()
            .player
            .to_string();
        let bet = f.state.pending_play().get_bet(&player_address);
        if bet.exists() {
            bet.delete();
        }
    }
    f.state.pending_plays().clear();
}

// An internal function that calculates distance between points in four dimentions (x, y, h, w)
pub fn euclidean_distance(a: Vec<f64>, b: Vec<f64>) -> f64 {
    (((a[0] - b[0]) * (a[0] - b[0])
        + (a[1] - b[1]) * (a[1] - b[1])
        + (a[2] - b[2]) * (a[2] - b[2])
        + (a[3] - b[3]) * (a[3] - b[3])) as f64)
        .sqrt()
}

// An internal function that takes many clusters and merges them using the Aglomerative Hierarchical Clustering
// algorithm and MIN_INTER_CLUSTER_DISTANCE as a parameter to prevent merging clusters too far apart from each other
pub fn clustering(mut clusters: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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
            let weight_1 = (clusters[index_1].len() - 4) as f64;
            let weight_2 = (clusters[index_2].len() - 4) as f64;

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
            for i in 0..weight_1 as i32 {
                new_cluster.push(clusters[index_1][i as usize + 4]);
            }
            for i in 0..weight_2 as i32 {
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
pub fn find_image_centers(image: u32, f: &EndGameContext, ctx: &ScFuncContext) -> Vec<TaggedImage> {
    let tags_req_per_image = f.state.plays_required_per_image().value();
    let mut hash_image_id: HashMap<u64, u32> = HashMap::new(); // a hashmap to retrieve an image_id from a tag_id
    let mut hash_play_tag_id: HashMap<u64, u32> = HashMap::new(); // a hashmap to retrieve an play_tag_id from a tag_id

    let mut clusters: Vec<Vec<f64>> = Vec::new(); // stores clusters with their centers and all the id's of the point's that conform it
    let mut playsfor_this_image = 0; // counts the real amount of players that tagged this image. This is because
                                     // the game could end before images are tagged with the required amount
                                     // it will be used to calculate the amount of players needed to agree for a valid tag
    for i in image * tags_req_per_image..(image + 1) * tags_req_per_image {
        // I'm forced to do this is because there are no nested arrays in schema yet
        if f.state.tagged_images().get_tagged_image(i as i32).value().image_id == -1 {
            break;
        }
        let tagged_image = f.state.tagged_images().get_tagged_image(i as i32).value();
        // Every 'tagged_image' starts as one cluster. The algorithm will then merge close-by clusters
        let x = input_str_to_vecf64(&tagged_image.x, ctx);
        let y = input_str_to_vecf64(&tagged_image.y, ctx);
        let h = input_str_to_vecf64(&tagged_image.h, ctx);
        let w = input_str_to_vecf64(&tagged_image.w, ctx);
        for j in 0..x.len() {
            // clusters have the following form:
            // [x, y, h, w, tag_id1. tag_id2, tag_id3, ... ],
            // where x, y, h and w are the center coordinates of the cluster
            let cluster = vec![
                x[j],
                y[j],
                h[j],
                w[j],
                clusters.len() as f64,
            ];
            hash_image_id.insert(clusters.len() as u64, i);
            hash_play_tag_id.insert(clusters.len() as u64, j as u32);
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
                let tagged_image = *hash_image_id.get(&(clusters[id][j] as u64)).unwrap();
                let player = f
                    .state
                    .tagged_images()
                    .get_tagged_image(tagged_image as i32)
                    .value()
                    .player;
                // increment the valid tags in the player's name. This is to calculate rewards in the end-
                let mut player_boost = f.state.player_boost().get_player_boost(&player.to_string()).value();
                player_boost.n_valid_tags = player_boost.n_valid_tags + 1;
                f.state.player_boost().get_player_boost(&player.to_string()).set_value(&player_boost);

                let vaid_tag = ValidTag {
                    player: player,
                    tagged_image: tagged_image,
                    play_tag_id: *hash_play_tag_id.get(&(clusters[id][j] as u64)).unwrap(),
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
        let cluster = vec![-1.0, -1.0, -1.0, -1.0];
        clusters.push(cluster);
    }

    let mut centers: Vec<TaggedImage> = Vec::new();
    for i in 0..clusters.len() {
        let center = TaggedImage {
            player: f.state.creator().value().address(), // the constructor requires a creator. This time it's not used tho.
            image_id: image as i32,
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
pub fn do_players_ranking(f: &EndGameContext, ctx: &ScFuncContext) -> Vec<Better> {
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
            .get_tagged_image(valid_tag.tagged_image as i32)
            .value();
        let tagged_image_coords = input_tgimg_to_vecs(&tagged_image, ctx);
        let tagged_image_point = &tagged_image_coords[player_tag_id];
        let boost = input_str_to_vecu8(&tagged_image.boost, ctx)[player_tag_id];
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
pub fn input_str_to_vecf64(string: &String, ctx: &ScFuncContext) -> Vec<f64> {
    let iterator = string.split_whitespace();
    let mut vec64: Vec<f64> = Vec::new();
    for i in iterator {
        let input = i.parse::<f64>();
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
pub fn input_str_to_vecu8(string: &String, ctx: &ScFuncContext) -> Vec<u8> {
    let iterator = string.split_whitespace();
    let mut vec8: Vec<u8> = Vec::new();
    for i in iterator {
        let input = i.parse::<u8>();
        match input {
            Ok(integer) => vec8.push(integer),
            Err(_error) => {
                ctx.panic("Error: Input couldn't be decoded correctly. Must be an int32.")
            }
        }
    }
    return vec8;
}

// An internal function to convert a vector of i32 variables into a string
// containing those variables, separated by spaces
pub fn vecu8_to_str(vec8: Vec<u8>) -> String {
    let mut string = "".to_string();
    for i in 0..vec8.len() {
        if i != 0 {
            string += " ";
        }
        string += &(vec8[i].to_string());
    }
    return string;
}

// An internal function to convert a vector of i32 variables into a string
// containing those variables, separated by spaces
pub fn vecf64_to_str(vecf64: Vec<f64>) -> String {
    let mut string = "".to_string();
    for i in 0..vecf64.len() {
        if i != 0 {
            string += " ";
        }
        string += &vecf64[i].to_string();
    }
    return string;
}

// An internal function to convert a vector of tagged images, each with a single point
// to one single tagged image but with many points
pub fn vec_to_tagged_image(vec: Vec<TaggedImage>, ctx: &ScFuncContext) -> TaggedImage {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut h: Vec<f64> = Vec::new();
    let mut w: Vec<f64> = Vec::new();
    let mut boost: Vec<u8> = Vec::new();
    for point in &vec {
        x.push(input_str_to_vecf64(&point.x, ctx)[0]);
        y.push(input_str_to_vecf64(&point.y, ctx)[0]);
        h.push(input_str_to_vecf64(&point.h, ctx)[0]);
        w.push(input_str_to_vecf64(&point.w, ctx)[0]);
        boost.push(input_str_to_vecu8(&point.boost, ctx)[0]);
    }
    let processed_image = TaggedImage {
        image_id: (&vec[0]).image_id,
        player: ctx.caller().address(), // field is required but not used in this case
        x: vecf64_to_str(x),
        y: vecf64_to_str(y),
        h: vecf64_to_str(h),
        w: vecf64_to_str(w),
        boost: vecu8_to_str(boost),
    };

    return processed_image;
}

// An internal function to get a vector of vectors of type i64, each vector representing
// a dimention and each dimention having multiple points. This is calculated taking a
// reference to a TaggedImage as an input.
pub fn input_tgimg_to_vecs(tagged_image: &TaggedImage, ctx: &ScFuncContext) -> Vec<Vec<f64>> {
    let x = input_str_to_vecf64(&tagged_image.x, ctx);
    let y = input_str_to_vecf64(&tagged_image.y, ctx);
    let h = input_str_to_vecf64(&tagged_image.h, ctx);
    let w = input_str_to_vecf64(&tagged_image.w, ctx);

    let mut vectors: Vec<Vec<f64>> = Vec::new();

    for i in 0..x.len() {
        vectors.push(vec![x[i], y[i], h[i], w[i]]);
    }
    return vectors;
}

// An internal function to convert inputs to the smart contract as strings that contain
// int64 variables separated by spaces into a vector of those int64 variables.
// CAUTION: inputs MUST BE of type i64. Else, the error will not be handeled correctly.
// This function takes no ctx, so no panic can be induced.
pub fn unsafe_input_str_to_vecf64(string: &String) -> Vec<f64> {
    let iterator = string.split_whitespace();
    let mut vec64: Vec<f64> = Vec::new();
    for i in iterator {
        let input = i.parse::<f64>();
        vec64.push(input.unwrap())
    }
    return vec64;
}

// An internal function to get a vector of vectors of type i64, each vector representing
// a dimention and each dimention having multiple points. This is calculated taking a
// reference to a TaggedImage as an input.
// CAUTION: inputs MUST BE of type i64. Else, the error will not be handeled correctly.
// This function takes no ctx, so no panic can be induced.
pub fn unsafe_input_tgimg_to_vecs(tagged_image: &TaggedImage) -> Vec<Vec<f64>> {
    let x = unsafe_input_str_to_vecf64(&tagged_image.x);
    let y = unsafe_input_str_to_vecf64(&tagged_image.y);
    let h = unsafe_input_str_to_vecf64(&tagged_image.h);
    let w = unsafe_input_str_to_vecf64(&tagged_image.w);

    let mut vectors: Vec<Vec<f64>> = Vec::new();

    for i in 0..x.len() {
        vectors.push(vec![x[i], y[i], h[i], w[i]]);
    }
    return vectors;
}