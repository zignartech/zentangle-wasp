// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;

// This function approves a new oracle to recieve it's price data.
// Only the current smart contract owner can call this function.
// Note that an oracle can not be taken out of it's position once it is approved.
pub fn func_approve_oracle(_ctx: &ScFuncContext, f: &ApproveOracleContext) {
    let agent_id = f.params.agentid().value();
    f.state.approved_map().get_bool(&agent_id).set_value(true);
    f.state.approved_list().append_agent_id().set_value(&agent_id);
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.contract_creator());
}

// This function takes the price (in cents) of MIOTA and stores it.
// Then, it calculates the new MITOA price using the find_price() function
// considering the prices set by the oracles that updated their price in the
// last minute. It stores that price in the last_price state variable.
pub fn func_set_miota_price(ctx: &ScFuncContext, f: &SetMiotaPriceContext) {
    // AgentID must be approved to be able to set a price
    ctx.require(f.state.approved_map().get_bool(&ctx.caller()).value(), "AgentID is not approved");

    let price = f.params.price().value();

    f.state.oracle_price().get_int64(&ctx.caller()).set_value(price);
    f.state.oracle_last_update().get_uint64(&ctx.caller()).set_value(ctx.timestamp());

    f.events.price_set(price);

    /*
    * Now we calculate the smart contract last_price value, based on the other oracles prices.
    */

    let mut prices: Vec<i64> = Vec::new();
    for oracle_id in 0..f.state.approved_list().length() {
        let oracle = f.state.approved_list().get_agent_id(oracle_id).value();

        let last_update = f.state.oracle_last_update().get_uint64(&oracle);
        if !last_update.exists() {continue; } // we skip the iteration because the oracle has never set a price.

        // last update must be not older than one minute ago for the oracle to be considered
        if ctx.timestamp() - last_update.value() < 60000000000 { // timestamps are saved in nanosecons
            let price = f.state.oracle_price().get_int64(&oracle).value();
            prices.push(price);
        }
    }

    let mut last_price = f.state.last_price().value();
    if !f.state.last_price().exists() {
        last_price = i64::MAX;
    }


    let new_price = find_price(prices, last_price);
    f.state.last_price().set_value(new_price);
    f.events.price_set(new_price);
}

pub fn func_set_owner(_ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

// This function calculates the average price of all oracles and returns it.
// Note that only prices that have been updated in the last minute are considered
pub fn view_get_miota_price(ctx: &ScViewContext, f: &GetMiotaPriceContext) {
    let price = f.state.last_price().value();

    ctx.log(&format!("price: {}", price));

    f.results.price().set_value(price);
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

fn find_price(prices: Vec<i64>, last_price: i64) -> i64 {

    // clusters are similar to prices, only that they are acompaied by a weight after the price,
    // indicating how many oracle's prices were merged into that price.
    let mut clusters: Vec<Vec<i64>> = Vec::new();

    for i in 0..prices.len() {
        clusters.push(vec![prices[i], 1]); // weight starts at one
    }

    let mut minimum_distance = 0; // stores the distance between the two closest clusters
    let mut merger = [0, 0]; // variable that stores the a and b clusters to be merged
    // not more than a 10% change bethween oracles is allowed
    while minimum_distance < last_price/10 {
        minimum_distance = i64::MAX;

        // evaluate every cluster against each other, but only once
        for a in 0..clusters.len() {
            for b in a+1..clusters.len() {
                let mut distance = clusters[a][0] - clusters[b][0];
                if distance < 0 {distance = -distance}
                if distance < minimum_distance {
                    minimum_distance = distance;
                    merger = [a, b]
                }
            }
        }

        // If the four dimentional distance is greater than last_price/10, then we dont merge the clusters.
        // Clusters that are this far apart are considered different final clusters
        if minimum_distance < last_price/10 {
            // define the indexes of the clusters one and two to be merged
            let [a, b] = merger;
            // also, read their respective weights
            let weight_a = clusters[a][1];
            let weight_b = clusters[b][1];

            // Calculating the coordiantes of the new cluster. The more weight,
            // the more influence on the new coordinate it has. This way, the
            // coordinate represents the average of all points in the cluster
            let new_cluster = vec![
                (clusters[a][0] * weight_a + clusters[b][0] * weight_b) / (weight_a + weight_b),
                weight_a + weight_b
            ];

            // Remove the old clusters and replace with the new one. Note that inxex_b > index_a.
            // When removing b first, we don't alter a.
            clusters.remove(b);
            clusters.remove(a);
            clusters.push(new_cluster);

        }
    }

    // If many clusters are left, we have a problem. 
    // They should not be >1 because it means prices prices are more than 10% apart.
    // the biggest cluster will win here.
    let mut cluster = &clusters[0];
    for i in 1.. clusters.len() {
        if clusters[i][1] > cluster[1] {
            cluster = &clusters[i];
        }
    }
    let price = cluster[0];

    return price;
}
