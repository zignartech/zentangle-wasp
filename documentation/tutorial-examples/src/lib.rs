// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

const PARAM_STRING: &str = "paramString";
const VAR_STRING: &str = "storedString";

const EXPORT_MAP: ScExportMap = ScExportMap {
    names: &[
        "storeString",
        "withdrawIota",
        "getString",
    ],
    funcs: &[
        store_string,
        withdraw_iota,
    ],
    views: &[
        get_string,
    ],
};

#[no_mangle]
fn on_call(index: i32) {
    ScExports::call(index, &EXPORT_MAP);
}

#[no_mangle]
fn on_load() {
    ScExports::export(&EXPORT_MAP);
}

// storeString entry point stores a string provided as parameters
// in the state as a value of the key 'storedString'
// panics if parameter is not provided
fn store_string(ctx: &ScFuncContext) {
    // take parameter paramString
    let params = ctx.params();
    let param_string = string_to_bytes(PARAM_STRING);
    ctx.require(params.exists(&param_string), "string parameter not found");

    let state = ctx.raw_state();
    // store the string in "storedString" variable
    let var_string = string_to_bytes(VAR_STRING);
    let value = params.get(&param_string);
    state.set(&var_string, &value);
     // log the text
    let msg = "Message stored: ".to_string() + &string_from_bytes(&value);
    ctx.log(&msg);
}

// getString view returns the string value of the key 'storedString'
// The call return result as a key/value dictionary.
// the returned value in the result is under key 'paramString'
fn get_string(ctx: &ScViewContext) {
    // take the stored string
    let state = ctx.raw_state();
    let var_string = string_to_bytes(VAR_STRING);
    let value = state.get(&var_string);
    // return the string value in the result dictionary
    let results = ScDict::new(&[]);
    let param_string = string_to_bytes(PARAM_STRING);
    results.set(&param_string, &value);
    ctx.results(&results);
}

// withdraw_iota sends all iotas contained in the contract's account
// to the caller's L1 address.
// Panics of the caller is not an address
// Panics if the address is not the creator of the contract is the caller
// The caller will be address only if request is sent from the wallet on the L1, not a smart contract
fn withdraw_iota(ctx: &ScFuncContext) {
    let creator = ctx.contract_creator();
    let caller = ctx.caller();

    ctx.require(creator == caller, "not authorised");
    ctx.require(caller.is_address(), "caller must be an address");

    let bal = ctx.balances().balance(&ScColor::IOTA);
    if bal > 0 {
        ctx.transfer_to_address(&caller.address(), ScTransfers::iotas(bal))
    }
}
