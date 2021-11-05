// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;
use crate::structs::*;

// Defining default configuration parameters

// The default amount of players to tag an image for it to be considered processed
const DEFAULT_TAGS_REQUIRED_PER_IMAGE: i16 = 10;

// This function is used for a researcher to start a game for players to tag images, wich can then be used
// for training an Artificial Intelligence (Deep Learning). It requires the researcher to place a reward for
// the players efforts. If the players tag correctly an image, based on other plays, it gets rewarded.
// The 'crateGame' function takes 2 mandatory parameters:
// - 'numberOfImages', which must be an Int32 number, and
// - 'description', which must be a String
// It can also take a non-mandatory parameter:
// - 'tagsRequiredPerImage', which must be an Int16
pub fn func_create_game(_ctx: &ScFuncContext, _f: &CreateGameContext) {

    // No game can be in progress in order to be able to crate a new one
    _ctx.require(_f.state.reward().value() != 0_i64, "Error: Game already in progress");

    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = _ctx.incoming();

    // Set the state variables of the game.
    let reward = incoming.balance(&ScColor::IOTA);
    let number_of_images = _f.params.number_of_images().value();
    let description = &_f.params.description().value();
    let mut tags_required_per_image = _f.params.tags_required_per_image().value();

    if tags_required_per_image < 1 {
        tags_required_per_image = DEFAULT_TAGS_REQUIRED_PER_IMAGE;
        _f.state.tags_required_per_image().set_value(tags_required_per_image);
    } 
    _f.state.reward().set_value(reward);
    _f.state.number_of_images().set_value(number_of_images);
    _f.state.description().set_value(description);
    _f.state.creator().set_value(&_ctx.caller());
    
    // Reward must be at least one iota per tag
    _ctx.require(reward >= tags_required_per_image as i64 * number_of_images as i64, "Error: Reward too low!");

    _ctx.event(&format!(
        "game.started {0} {1} {2} {3}",
        reward.to_string(),
        number_of_images,
        description,
        tags_required_per_image
    ));
}

// This function can only be executed by the creator of the game (alias: the investigator.
// Ideally, it should be executed when all images have been tagged the required amount of times,
// although it can be executed before that. What this function does, is to arrange the tags of 
// the individual images into clusters and finding outliers this way. Outliers are tags that are 
// not inside of a cluster, and represent a bad tag and arre not considered valid. For every 
// valid tag, it's corresponding player recieves a reward. This reward is the monney put into the 
// contract by the investigator, and it gets split equally between valid tags. We calculate the 
// center of the clusters and find the top best tags by finding the shortest distances from this tags 
// to the center of their cluster. The betters that placed this tags, win the betting monney, 
// apart from the default reward for making valid tags.
// The 'endGame' function takes no parameters.
pub fn func_end_game(_ctx: &ScFuncContext, _f: &EndGameContext) {

    // The context caller has to be the game crator.
    _ctx.require(_f.state.creator().value() == _ctx.caller(), 
    "Error: Only the game creator can end the game");

    // Iterate across all the images, and save: 
    // - the centers of the clusters for every image
    // - the players realted to the valid tags
    // - the distances of valid tags to the center of it's cluster

    let mut centers: Vec<Vec<i64>> = Vec::new();
    let number_of_images = _f.state.number_of_images().value();
    for image in 0..number_of_images {

        // Apply Aglomerative Hierarchical clustering:
        let mut clusters:Vec<Vec<i64>> = Vec::new();
        let mut n_clusters = 0; // every tag starts as a cluster
        
        // 'cluster' is a vector storing the data of the 4 dimentional center of the cluster 
        // and all the id's of the point's that conform it
        for i in image..image+number_of_images{
            if _f.state.tagged_images().get_tagged_image(i).exists() == false {break}
            let tagged_image = _f.state.tagged_images().get_tagged_image(i).value();
            let cluster = vec![tagged_image.x, tagged_image.y, tagged_image.h, tagged_image.w, i as i64];
            clusters.push(cluster);
            n_clusters +=1;
        }

        // every tag starts as a different cluster. We merge them until they are more than 100 pixels⁴ apart.
        let mut min_distance= [0.0, 0.0, 0.0]; // stores [min_distance between two clusters, 1st cluster, 2nd cluster]

        while min_distance[0] < 100.0 {
            // Evaluate the distance matrix and store the shortest euclidean distance in 'min_distance[0]'
            fn ed(a: Vec<i64>, b: Vec<i64>) -> f64 { // Euclidean Distance function in four dimentions
                (((a[0]-b[0])*(a[0]-b[0]) + (a[1]-b[1])*(a[1]-b[1]) + (a[2]-b[2])*(a[2]-b[2]) + (a[3]-b[3])*(a[3]-b[3])) as f64).sqrt()
            }

            min_distance[0]= 9999999.0;
            for x in 0..n_clusters {
                for y in 0..n_clusters {
                    let distance = ed(clusters[x].clone(), clusters[y].clone());
                    if distance < min_distance[0] {
                        min_distance = [distance, x as f64, y as f64];
                    }
                }
            }

            // If the four dimentional distance is greter than 100, then we dont merge the clusters.
            // Points thet are this far away are considered a different clusters
            if min_distance[0] < 100.0 {

                // define the indexes of the clusters one and two, to be merged
                let index_1 = min_distance[1] as usize;
                let index_2 = min_distance[2] as usize;
                // the weight is equal to the number of point's that conform the cluster
                let weight_1 = (clusters[index_1].len() - 4) as i64;
                let weight_2 = (clusters[index_2].len() - 4) as i64;
                
                // Calculating the coordiantes of the new cluster. The more weight a cluster has, 
                // the more influence on the new coordinate it has.
                let mut new_cluster = vec![
                    (clusters[index_1][0] * weight_1 + clusters[index_2][0] * weight_2)/(weight_1 + weight_2),
                    (clusters[index_1][1] * weight_1 + clusters[index_2][1] * weight_2)/(weight_1 + weight_2),
                    (clusters[index_1][2] * weight_1 + clusters[index_2][2] * weight_2)/(weight_1 + weight_2),
                    (clusters[index_1][3] * weight_1 + clusters[index_2][3] * weight_2)/(weight_1 + weight_2)
                ];
                // Copying the point's inside both clusters to the new one
                for i in 0..weight_1 {
                    new_cluster.push(clusters[index_1][i as usize + 5]);
                }
                for i in 0..weight_2 {
                    new_cluster.push(clusters[index_2][i as usize + 5]);
                }

                // Remove the old clusters and replaces with the new one.
                clusters.remove(index_1);
                clusters.remove(index_2);
                clusters.push(new_cluster);
            }         
        }
        
    }
      
    // We clear all the state variables, so a new game can begin
    _f.state.bets().clear();
	_f.state.plays_per_image().clear();
	_f.state.tagged_images().clear();
	_f.state.processed_images().clear();
    _f.state.reward().set_value(0);

}

// This function is used by players to be assigned an image and for them to place a bet on their tags.
// If an image has all the tags it requires by the 'tags_required_per_image' variable, this image can no
// longer be assigned to a player. Also, if all images have their required tags, no image can be assigned at all.
// The 'requestPlay' function takes no parameters.
pub fn func_request_play(_ctx: &ScFuncContext, _f: &RequestPlayContext) {
    
    // Defining relevant variables for the request 
    let tags_required_per_image = _f.state.tags_required_per_image().value();
    let number_of_images = _f.state.number_of_images().value();
    let player = _ctx.caller();
    let plays_per_image = _f.state.plays_per_image();
    let pending_plays: ArrayOfMutableBet = _f.state.pending_plays();

    // Check if the player has an open request. If it does, panic. 
    for i in 0..pending_plays.length() {
        if pending_plays.get_bet(i).value().player == _ctx.caller() {
            panic("Error: Player already has an open request");
        }
    }

    // Check if any images are available for the player to tag. If all are tagged the required amount of times,
    // or if the ones available have been already tagged by the player, the counter will be equal to the number of images.
    let mut counter = 0;
    for i in 0..plays_per_image.length() {
        if plays_per_image.get_int16(i).value() >= tags_required_per_image {
            counter += 1;
            continue;
        }
        for j in i..i+tags_required_per_image as i32 {
            if _f.state.tagged_images().get_tagged_image(j).value().player.address() == player.address() {
                counter +=1;
            }
        }
    }

    // If no more images are available to tag, we dont accept the request and panic.
    _ctx.require(counter != number_of_images, "Error: Sorry, no more images are available to tag");

    // We choose an image randomly to assign to the player for tagging.
    // This code can be hard to swallow, but it simply checks if the image has been tagged the required amount of times, 
    // or if it has already been tagged by the player. If so, we choose another one.
    // Im sorry if it's too messy. I couldn't figure out a better way.
    let mut image_id = _ctx.utility().random((number_of_images-1).into()) as i32;
    while plays_per_image.get_int16(image_id).value() == tags_required_per_image || check_image_tagged_by_player(image_id, tags_required_per_image, _f, player.address()){
        image_id = _ctx.utility().random((number_of_images-1).into()) as i32;
    }

    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = _ctx.incoming();
    let bet = Bet {
        amount: incoming.balance(&ScColor::IOTA),
        player: _ctx.caller(),
        image_id: image_id,
    };

    // Append the bet data to the bets array and to the pending plays array. 
    // They will automatically take care of serializing the bet struct into a bytes representation.
    let bets: ArrayOfMutableBet = _f.state.bets();
    let bets_nr: i32 = bets.length();
    bets.get_bet(bets_nr).set_value(&bet);
    let pending_plays_nr: i32 = pending_plays.length();
    pending_plays.get_bet(pending_plays_nr).set_value(&bet);

    _ctx.event(&format!(
        "play.requested {0} {1} {2}",
        &bet.player.address().to_string(),
        bet.amount,
        bet.image_id
    ));

    _f.results.image_id().set_value(image_id);
}

// This function is necessary to check if an image is to be assigned to a player.
// It just checks if an image has allready tagged by a player in order to avoid giving
// The same image twice to a player.
fn check_image_tagged_by_player(image_id: i32, tags_required_per_image: i16, _f: &RequestPlayContext, address: ScAddress) -> bool {
    for j in image_id..image_id+tags_required_per_image as i32 {
        if _f.state.tagged_images().get_tagged_image(j).value().player.address() == address {
            return true;
        }   
    }
    return false
}

// This function is used for a player to tag an image that has been assigned to it. 
// It basically deletes the request from the request list 'pending_plays' and adds the
// information of the tag to the tagged images list and adds one to the number of times
// the image has been tagged, using the 'plays_per_game' list.
// The 'sendTags' function takes 4 mandatory parameters, corresponding to the coordinates and dimentions of the tag:
// - 'x', which must be an Int64 number,
// - 'y', which must be an Int64 number, 
// - 'h', which must be an Int64 number and
// - 'w', which must be an Int64 number
pub fn func_send_tags(_ctx: &ScFuncContext, _f: &SendTagsContext) {

    let pending_plays: ArrayOfMutableBet = _f.state.pending_plays();
    let mut bet: Option<MutableBet> = None;
    let mut pending_play_id: i32 = -1;
    
    // Searching for the player's open request. If it doesn't, panic.
    // If it does, it will get stored as an option. We will have to use unwrap() to access it
    for i in 0..pending_plays.length() {
        if pending_plays.get_bet(i).value().player == _ctx.caller() {
            bet = Some(pending_plays.get_bet(i));
            pending_play_id = i;
        }
    }
    if bet.is_none() {
        _ctx.panic("Error: No plays requested for this address");
    }

    // We delete the bet from the pending plays by clearing the array and copying again minus the bet of our guy here
    _f.state.pending_plays().clear();
    for i in 0..pending_play_id {
        _f.state.pending_plays().get_bet(i).set_value(&pending_plays.get_bet(i).value());
    }
    for i in pending_play_id+1..pending_plays.length() {
        _f.state.pending_plays().get_bet(i).set_value(&pending_plays.get_bet(i).value());
    }

    // We gather all the information into this struct
    let tagged_image = TaggedImage {
        image_id: bet.unwrap().value().image_id,
        player: _ctx.caller(),
        x: _f.params.x().value(),
        y: _f.params.y().value(),
        h: _f.params.h().value(),
        w: _f.params.w().value()
    };

    // Get the array of current tagged immages from state storage and find it's length.
    let tagged_images: ArrayOfMutableTaggedImage = _f.state.tagged_images();
    let tagged_image_nr: i32 = tagged_images.length();

    // Append the bet data to the bets array. The bet array will automatically take care
    // of serializing the bet struct into a bytes representation.
    tagged_images.get_tagged_image(tagged_image_nr).set_value(&tagged_image);

    _ctx.event(&format!(
        "dtag.image.tagged {0}",
        &tagged_image.player.address().to_string(),
    ));

    // Add one to the number of times this image has been tagged
    let plays_for_this_image: i16 = _f.state.plays_per_image().get_int16(tagged_image.image_id).value();
    _f.state.plays_per_image().get_int16(tagged_image.image_id).set_value(plays_for_this_image + 1);
}

pub fn view_get_plays_per_image(_ctx: &ScViewContext, _f: &GetPlaysPerImageContext) {
}

pub fn view_get_results(_ctx: &ScViewContext, _f: &GetResultsContext) {
}

pub fn view_get_tagged_images(_ctx: &ScViewContext, _f: &GetTaggedImagesContext) {
}
