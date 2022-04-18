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

// An internal function that adds a send_tags or request_play funtion value to the player's game's bets
pub fn add_bet(ctx: &ScFuncContext, f_send_tags: Option<&SendTagsContext>, f_request_play: Option<&RequestPlayContext>, image_id: u32) {
    
    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = ctx.incoming();
    let bet = Bet {
        amount: incoming.balance(&ScColor::IOTA) as u64,
        player: ctx.caller().address(),
        image_id: image_id as u32,
    };
    // Append the bet data to the bets array and to the pending plays map.
    // They will automatically take care of serializing the bet struct into a bytes representation.
    if f_send_tags.is_some() {
        let f = f_send_tags.unwrap();
        let bets: ArrayOfMutableBet = f.state.bets();
        bets.append_bet().set_value(&bet);
    }
    else if f_request_play.is_some() {
        let f = f_request_play.unwrap();
        let bets: ArrayOfMutableBet = f.state.bets();
        bets.append_bet().set_value(&bet);
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

    let player_info = f.state.player_info().get_player_info(player_address);
    // Fill in the variables if the player has a history
    if player_info.exists() {
        n_double_boosts = player_info.value().n_double_boosts;
        n_tripple_boosts = player_info.value().n_tripple_boosts;
        n_tags = player_info.value().n_tags;
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

        let player = PlayerInfo {
            player: ctx.caller(),
            n_tags: n_tags,
            n_double_boosts: n_double_boosts,
            n_tripple_boosts: n_tripple_boosts,
            n_valid_tags: 0
        };

        // if the player isn't on the players list, add it
        if !f.state.player_info().get_player_info(player_address).exists() {
            f.state
            .players_info()
            .append_string()
            .set_value(&player.player.address().to_string());
        }
        // update player's boost data on map
        f.state
            .player_info()
            .get_player_info(player_address)
            .set_value(&player);
        f.state.total_player_tags().get_uint64(player_address).set_value(total_player_tags);
    }
}

// An internal function to check if the amount betted doesn't exceed the maximum allowed
pub fn check_ingots(amount_betted: u64, f: &RequestPlayContext, ctx: &ScFuncContext) {
    let miota: u64 = 1000000;
    if amount_betted <= 60*miota { return } // Everyone can bet 60 Mi or less

    let player_address = &ctx.caller().address().to_string();
    let mut total_player_tags = 0;
    if f.state.total_player_tags().get_uint64(player_address).exists() {
        total_player_tags = f.state.total_player_tags().get_uint64(player_address).value();
    }
    let ingots = total_player_tags / 576;

    let err_msg = "Error: Not enough ingots";
    if amount_betted <= 180*miota {
        ctx.require(ingots >= 1, err_msg);
    }
    else if amount_betted <= 480*miota {
        ctx.require(ingots >= 2, err_msg);
    }
    else if amount_betted <= 780*miota {
        ctx.require(ingots >= 4, err_msg);
    }
    else if amount_betted <= 1260*miota {
        ctx.require(ingots >= 6, err_msg);
    }
    else if amount_betted <= 2040*miota {
        ctx.require(ingots >= 8, err_msg);
    }
    else if amount_betted <= 3300*miota {
        ctx.require(ingots >= 10, err_msg);
    }
    else {
        ctx.require(ingots >= 12, err_msg);
    }
}

// An internal funtion that checks if the player can tag more images or not. 
// If it can, do a new request_image. It does not use the request_image function
// so it is much quicker.
pub fn internal_request_play(f: &SendTagsContext, ctx: &ScFuncContext) {
    // Initialize variables
    let plays_required_per_image = f.state.plays_required_per_image().value();
    let number_of_images = f.state.number_of_images().value();
    let plays_per_image = f.state.plays_per_image();
    let player = ctx.caller();

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
            if tgd_img.player
                == player.address()
            {
                counter += 1;
                continue 'image;
            }
            
        }
        break;
    }

    if counter < f.state.number_of_images().value() {
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

        f
            .state
            .pending_play()
            .get_bet(&player.address().to_string())
            .set_value(&bet);
        f.state
            .pending_plays()
            .append_bet()
            .set_value(&bet);

        f.events
            .play_requested(&bet.player.to_string(), bet.amount, bet.image_id);

        f.results.image_id().set_value(image_id);
    }
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

// An internal function to clear the player map and the players list
pub fn clear_player(f: &EndGameContext) {
    for player_id in 0..f.state.players_info().length() {
        let player_address = f
            .state
            .players_info()
            .get_string(player_id)
            .value();
        let player = f.state.player_info().get_player_info(&player_address);
        if player.exists() {
            player.delete();
        }
    }
    f.state.players_info().clear();
}

// An internal function that takes many clusters and merges them using the Aglomerative Hierarchical Clustering
// algorithm and MIN_INTER_CLUSTER_DISTANCE as a parameter to prevent merging clusters too far apart from each other
pub fn clustering(
    mut clusters: Vec<Vec<f64>>,
    min_inter_cluster_distance: f64,
    n_clusters_option: Option<usize>
) -> Vec<Vec<f64>> {
    // define maximum number of clusters, useful if we want to fix the number of clusters in the output
    let mut max_n_clusters = 0;
    if n_clusters_option.is_some() { max_n_clusters = n_clusters_option.unwrap(); }

    let mut min_distance = 0.0; // stores the distance between two clusters: a and b
    let mut merger = [0, 0]; // stores the a and b clusters ids to be merged

    // Here, we apply the Aglomerative Hierarchical Clustering: Merging all clusters that are the closest to each other
    // until the closest are more than min_inter_cluster_distance pixels⁴ or there is only one cluster left (in that
    // case, f64::MAX will not be overwritten).
    // If min_inter_cluster_distance is set to zero and there is a n_clusters, it will merge until it reaches that number of clusters 
    while min_distance < min_inter_cluster_distance {
        // Evaluate the distance matrix and store the shortest euclidean distance in 'min_distance[0]'
        min_distance = f64::MAX;
        for x in 0..clusters.len() {
            for y in (x+1)..clusters.len() { // this way we dont evaluete a pair twice nor a cluster against itself
                let distance = euclidean_distance(clusters[x].clone(), clusters[y].clone());
                if distance < min_distance {
                    min_distance = distance; 
                    merger = [x, y];
                }
            }
        }
        // If the four dimentional distance is greater than min_inter_cluster_distance, then we dont merge the clusters.
        // Clusters that are this far apart are considered different final clusters
        if min_distance < min_inter_cluster_distance {
            // define the indexes of the clusters a and b to be merged
            let index_a = merger[0] as usize;
            let index_b = merger[1] as usize;
            // the weights are equal to the number of point's that conform the cluster
            let weight_a = (clusters[index_a].len() - 4) as f64;
            let weight_b = (clusters[index_b].len() - 4) as f64;

            // Calculating the coordiantes of the new cluster. The more weight,
            // the more influence on the new coordinate it has. This way, the
            // coordinate represents the average of all points in the cluster
            let mut new_cluster = vec![
                (clusters[index_a][0] * weight_a + clusters[index_b][0] * weight_b)
                    / (weight_a + weight_b),
                (clusters[index_a][1] * weight_a + clusters[index_b][1] * weight_b)
                    / (weight_a + weight_b),
                (clusters[index_a][2] * weight_a + clusters[index_b][2] * weight_b)
                    / (weight_a + weight_b),
                (clusters[index_a][3] * weight_a + clusters[index_b][3] * weight_b)
                    / (weight_a + weight_b),
            ];
            // Push the point's inside both clusters to the new cluster
            for i in 0..weight_a as i32 {
                new_cluster.push(clusters[index_a][i as usize + 4]);
            }
            for i in 0..weight_b as i32 {
                new_cluster.push(clusters[index_b][i as usize + 4]);
            }

            // Remove the old clusters and replace with the new one. Note that index_b > index_a.
            // When removing index_b first, we don't alter index_a.
            clusters.remove(index_b);
            clusters.remove(index_a);
            clusters.push(new_cluster);
        }
        // for the manage_repeated_players function, we need to get to a number of clusters
        if clusters.len() <= max_n_clusters { break; }
    }
    return clusters;
}

// An internal function to prevent a players to be more than once in a single culster. If this happens, it can mean one of two things:
// Either there are actually more than one objects in that cluster and should be separated in more clusters, or
// the player has tagged an object where there is none. In that case, the player's tags inside the cluster will be deleted to not distord the data. 
pub fn manage_repeated_players(
    f: &EndGameContext, 
    ctx: &ScFuncContext, 
    mut clusters: Vec<Vec<f64>>, 
    mut hash_image_id: HashMap<u64, u32>,
    mut hash_play_tag_id: HashMap<u64, u32>, 
    plays_for_this_image: i32
) -> (
    Vec<Vec<f64>>, 
    HashMap<u64, u32>, 
    HashMap<u64, u32>
) {
    // analize clusters with repeated players
    let clusters_len =  clusters.len(); // clusters will mutate over the iterations. We want the initial original cluster's value.
    for i in 0.. clusters_len {
        let cluster_id = clusters_len - i - 1; // this way it's a backwards iterator and we dont change the id's as we remove them.
        let cluster = clusters[cluster_id].clone();
        let mut player_repeats_counter: HashMap<String, u16> = HashMap::new(); // count the n of times a player is in the cluster

        // count how many times each player is in this cluster
        // for this, we get the player for each point and add one to it's counter
        for i in 4..cluster.len() { // start at four to skip coordinates of the tag and just read point id's
            // get the player
            let image_id = *hash_image_id.get(&(cluster[i] as u64)).unwrap();
            let tgdimg = f.state.tgd_imgs().get_tgd_img(image_id).value();
            let player = tgdimg.player;

            // add one to it's counter
            if !player_repeats_counter.contains_key(&player.to_string()) {
                player_repeats_counter.insert(player.to_string(), 1);
            } else {
                let player_repeats = *player_repeats_counter.get(&player.to_string()).unwrap();
                player_repeats_counter.insert(player.to_string(), player_repeats + 1);
            }
        }
        // count how many times a number of repeats is present.
        let mut n_repeats: Vec<u16> = vec![0; cluster.len()-3]; // the position in the vec indicates the number of repeats a player was inside the cluster
                                                                // the number in that position indicates how many players had this number of repeats.
                                                                // the vector is initialized with zeros and it's length is the amount of points in the cluster.
                                                                // Worst case scenario, all point's belong to the same player.
                                                                // Note that position 0 will be empty.
        let mut max_repeats = 0;
        for (_, &value) in player_repeats_counter.iter(){
            if value > max_repeats {max_repeats = value};
            n_repeats[value as usize] += 1;
        }
        // if no repeated players exist on the cluster, skip to the sext cluster
        if max_repeats == 1 {
            continue
        }

        // otherwise, see how many clusters must be generated and which players have done too many tags and must be deleted from the cluster entirely
        let min_n_players = (CONFIRMATION_PERCENTAGE*plays_for_this_image as f32) as u16; // indicates how many players must be in a cluster for it to be valid
        let mut n_players = 0;
        let mut n_clusters = 1; // number of clusters that should result form the division of the current cluster
        for i in 0..n_repeats.len() {
            n_players += n_repeats[i];
            // n_clusters should be updated only on the limit and on further iterations
            if n_players >= min_n_players && n_clusters == 1 {
                n_clusters = i;
            }
        }
        // separate the cluster in unique points again to untangle the mess
        let mut new_clusters: Vec<Vec<f64>> = Vec::new();
        for i in 4..cluster.len() { // start at four to skip coordinates of the tag and just read point id's
            let tag_id = &(cluster[i] as u64);
            let image_id = *hash_image_id.get(tag_id).unwrap();
            let play_tag_id = *hash_play_tag_id.get(tag_id).unwrap();
            let tgdimg = f
                .state
                .tgd_imgs()
                .get_tgd_img(image_id)
                .value();

            // players that have too many tangs get discarded completely from this cluster.
            let player_repeats = *player_repeats_counter.get(&tgdimg.player.to_string()).unwrap();
            if player_repeats > n_clusters as u16 {
                continue
            }

            // get the coordinates back from the point id and create a new cluster
            let tgimg_coords = input_tgimg_to_vecs(&tgdimg, ctx);
            let mut tgimg_point = (&tgimg_coords[play_tag_id as usize]).to_vec();

            // add new points to the maps and to the new cluster.
            let total_n_points = hash_image_id.len();
            tgimg_point.push(total_n_points as f64);
            hash_image_id.insert(total_n_points as u64, image_id);
            hash_play_tag_id.insert(total_n_points as u64, play_tag_id);
            new_clusters.push(tgimg_point);
        }

        // clusterize again. We wannt to have n_clusters as a result
        new_clusters = clustering(new_clusters,f64::MAX, Some(n_clusters));

        // remove old cluster and add new one(s)
        clusters.remove(cluster_id);
        for i in new_clusters {
            clusters.push(i);
        }
    }

    return (clusters, hash_image_id, hash_play_tag_id)
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
        let valid_tag = f.state.valid_tags().get_valid_tag(i as u32).value();

        let tgd_img = f
            .state
            .tgd_imgs()
            .get_tgd_img(valid_tag.tgd_img)
            .value();
        let tgd_img_coords = input_tgimg_to_vecs(&tgd_img, ctx);
        let tgd_img_point = &tgd_img_coords[valid_tag.play_tag_id as usize];
        let boost = input_str_to_vecu8(&tgd_img.boost, ctx)[valid_tag.play_tag_id as usize];

        let clusters_centers = f
            .state
            .processed_images()
            .get_tgd_img(tgd_img.image_id as u32)
            .value();
        let cluster_center_coords = input_tgimg_to_vecs(&clusters_centers, ctx);
        let mut distance_to_cluster_center =
            euclidean_distance(tgd_img_point.clone(), cluster_center_coords[0].clone());
        for j in 1..cluster_center_coords.len() {
            let cluster_center_point = &cluster_center_coords[j];
            let distance =
                euclidean_distance(tgd_img_point.to_vec(), cluster_center_point.to_vec());
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

// An internal function that calculates distance between points in four dimentions (x, y, h, w)
pub fn euclidean_distance(a: Vec<f64>, b: Vec<f64>) -> f64 {
    (((a[0] - b[0]) * (a[0] - b[0])
        + (a[1] - b[1]) * (a[1] - b[1])
        + (a[2] - b[2]) * (a[2] - b[2])
        + (a[3] - b[3]) * (a[3] - b[3])) as f64)
        .sqrt()
}

// An internal function to calculate the average position of a tag (center) inside an image.
pub fn find_image_centers(image: u32, f: &EndGameContext, ctx: &ScFuncContext) -> Vec<TgdImg> {
    let tags_req_per_image = f.state.plays_required_per_image().value();
    let mut hash_image_id: HashMap<u64, u32> = HashMap::new(); // a hashmap to retrieve an image_id from a tag_id
    let mut hash_play_tag_id: HashMap<u64, u32> = HashMap::new(); // a hashmap to retrieve an play_tag_id from a tag_id.
                                                                  // it defines the id of one tag inside a tgd_img

    let mut clusters: Vec<Vec<f64>> = Vec::new(); // stores clusters with their centers and all the id's of the point's that conform it
    let mut plays_for_this_image = 0; // counts the real amount of players that tagged this image. This is because
                                     // the game could end before images are tagged with the required amount
                                     // it will be used to calculate the amount of players needed to agree for a valid tag
    for i in image * tags_req_per_image..(image + 1) * tags_req_per_image {
        // I'm forced to do this is because there are no nested arrays in schema yet
        if f.state.tgd_imgs().get_tgd_img(i).value().image_id == -1 {
            break;
        }
        let tgimg = f.state.tgd_imgs().get_tgd_img(i).value();
        let tgimg_coords = input_tgimg_to_vecs(&tgimg, ctx);
        for j in 0..tgimg_coords.len() {
            // clusters have the following form:
            // [x, y, h, w, tag_id1, tag_id2, tag_id3, ... ],
            // where x, y, h and w are the center coordinates of the cluster
            let mut cluster = (&tgimg_coords[j]).to_vec();
            cluster.push(clusters.len() as f64);

            // update the maps
            hash_image_id.insert(clusters.len() as u64, i);
            hash_play_tag_id.insert(clusters.len() as u64, j as u32);
            clusters.push(cluster);
        }
        plays_for_this_image += 1;
    }

    // every tag starts as a different cluster. We merge them until they are at least
    // MIN_INTER_CLUSTER_DISTANCE pixels⁴ apart or there is only one cluster for the image.
    clusters = clustering(clusters, MIN_INTER_CLUSTER_DISTANCE ,None);

    // It may happen that players are found in the same cluster more than once. This means there could be
    // more than one object on that cluster and if many players indicate so, it has to be separated in many clusters.
    // If a player tags many times the same object to exploit the algorithm, it will be stopped here.
    (clusters, hash_image_id, hash_play_tag_id) = manage_repeated_players(f, ctx, clusters, hash_image_id, hash_play_tag_id, plays_for_this_image);

    // We should be left with clusters with high player participation.
    // The ones that have fewer points get discarted.
    // Here we also store all the players that made correct taggs. They can be stored multiple times.
    let length = clusters.len(); // clusters.len() will be changing, so we want to fix it here
    for i in 0..length {
        let id = length - i - 1; // this way it's a backwards iterator and we dont change the id's as we remove them.
        if clusters[id].len() - 4 < (plays_for_this_image as f32 * CONFIRMATION_PERCENTAGE) as usize
        {
            clusters.remove(id);
        } else {
            // here we push the players that tagged correctly to the reward-list and add the tag to valid_tags
            for j in 4..clusters[id].len() {
                let tgdimg = *hash_image_id.get(&(clusters[id][j] as u64)).unwrap();
                let player = f
                    .state
                    .tgd_imgs()
                    .get_tgd_img(tgdimg)
                    .value()
                    .player;
                // increment the valid tags in the player's name. This is to calculate rewards in the end-
                let mut player_info = f.state.player_info().get_player_info(&player.to_string()).value();
                player_info.n_valid_tags = player_info.n_valid_tags + 1;
                f.state.player_info().get_player_info(&player.to_string()).set_value(&player_info);

                let vaid_tag = ValidTag {
                    player: player,
                    tgd_img: tgdimg,
                    play_tag_id: *hash_play_tag_id.get(&(clusters[id][j] as u64)).unwrap(),
                };
                f.state
                    .valid_tags()
                    .append_valid_tag()
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

    let mut centers: Vec<TgdImg> = Vec::new();
    for i in 0..clusters.len() {
        let center = TgdImg {
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
pub fn vec_to_tgd_img(vec: Vec<TgdImg>, ctx: &ScFuncContext) -> TgdImg {
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
    let processed_image = TgdImg {
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
// reference to a tagged image (TgdImg) as an input.
pub fn input_tgimg_to_vecs(tgd_img: &TgdImg, ctx: &ScFuncContext) -> Vec<Vec<f64>> {
    let x = input_str_to_vecf64(&tgd_img.x, ctx);
    let y = input_str_to_vecf64(&tgd_img.y, ctx);
    let h = input_str_to_vecf64(&tgd_img.h, ctx);
    let w = input_str_to_vecf64(&tgd_img.w, ctx);

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
// reference to a tagged image (TgdImg) as an input.
// CAUTION: inputs MUST BE of type i64. Else, the error will not be handeled correctly.
// This function takes no ctx, so no panic can be induced.
pub fn unsafe_input_tgimg_to_vecs(tgd_img: &TgdImg) -> Vec<Vec<f64>> {
    let x = unsafe_input_str_to_vecf64(&tgd_img.x);
    let y = unsafe_input_str_to_vecf64(&tgd_img.y);
    let h = unsafe_input_str_to_vecf64(&tgd_img.h);
    let w = unsafe_input_str_to_vecf64(&tgd_img.w);

    let mut vectors: Vec<Vec<f64>> = Vec::new();

    for i in 0..x.len() {
        vectors.push(vec![x[i], y[i], h[i], w[i]]);
    }
    return vectors;
}