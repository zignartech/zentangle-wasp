// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;
use serde::de::EnumAccess;
use serde::{Deserialize, Serialize};

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.contract_creator());
}

pub fn func_set_owner(_ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn func_load_addresses(ctx: &ScFuncContext, f: &LoadAddressesContext) {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Addresses {
        addresses: Vec<String>,
    }
    // convert input as strings to vectors of integers
    let input: Result<_, serde_json::Error> = serde_json::from_str(&f.params.addresses().value());
    let mut addresses_option: Option<Addresses> = None;
    match input {
        Ok(a) => addresses_option = Some(a),
        Err(e) => ctx.panic(&e.to_string()),
    }
    let addresses = addresses_option.unwrap();
    for i in 0..addresses.addresses.len() {
        f.state.addresses().append_string().set_value(&addresses.addresses[i]);
        f.events.address_loaded(&addresses.addresses[i]);
    }
}

pub fn func_unload_addresses(_ctx: &ScFuncContext, f: &UnloadAddressesContext) {
    f.state.addresses().clear();
    f.events.addresses_unloaded();
}

pub fn func_ruffle(ctx: &ScFuncContext, f: &RuffleContext) {
    let addresses = f.state.addresses();

    let n_winners = f.params.n_winners().value();

    ctx.require(
        n_winners as u32 <= addresses.length(),
        "Error: Not sufficient addresses",
    );

    let mut winners: Vec<String> = Vec::new();
    let mut counter = 0;

    'a: while counter < n_winners {
        let winner_id = ctx.random(addresses.length() as u64);

        let winner = addresses.get_string(winner_id as u32).value();
        // if this address is allready a winner, skip it
        for w in 0..winners.len() {
            if winner == winners[w] {
                continue 'a;
            }
        }
        winners.push(winner.clone());
        counter += 1;
    }

    let mut winners_json = "{\n\"addresses\": [ \n".to_string();
    for i in 0..(n_winners as usize) - 1 {
        winners_json.push_str("\"");
        winners_json.push_str(&winners[i]);
        winners_json.push_str("\",\n");
    }

    winners_json.push_str("\"");
    winners_json.push_str(&winners[n_winners as usize - 1]);
    winners_json.push_str("\"\n");
    winners_json.push_str("]\n}");

    f.events.winner(&winners_json);
    f.results.winners().set_value(&winners_json);
}
