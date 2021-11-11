// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]

use wasmlib::*;

use crate::*;

pub(crate) const IDX_PARAM_ADDRESS          : usize = 0;
pub(crate) const IDX_PARAM_AGENT_ID         : usize = 1;
pub(crate) const IDX_PARAM_CALLER           : usize = 2;
pub(crate) const IDX_PARAM_CHAIN_ID         : usize = 3;
pub(crate) const IDX_PARAM_CHAIN_OWNER_ID   : usize = 4;
pub(crate) const IDX_PARAM_CONTRACT_CREATOR : usize = 5;
pub(crate) const IDX_PARAM_CONTRACT_ID      : usize = 6;
pub(crate) const IDX_PARAM_COUNTER          : usize = 7;
pub(crate) const IDX_PARAM_FAIL             : usize = 8;
pub(crate) const IDX_PARAM_HASH             : usize = 9;
pub(crate) const IDX_PARAM_HNAME            : usize = 10;
pub(crate) const IDX_PARAM_HNAME_CONTRACT   : usize = 11;
pub(crate) const IDX_PARAM_HNAME_EP         : usize = 12;
pub(crate) const IDX_PARAM_HNAME_ZERO       : usize = 13;
pub(crate) const IDX_PARAM_INT64            : usize = 14;
pub(crate) const IDX_PARAM_INT64_ZERO       : usize = 15;
pub(crate) const IDX_PARAM_INT_VALUE        : usize = 16;
pub(crate) const IDX_PARAM_NAME             : usize = 17;
pub(crate) const IDX_PARAM_PROG_HASH        : usize = 18;
pub(crate) const IDX_PARAM_STRING           : usize = 19;
pub(crate) const IDX_PARAM_STRING_ZERO      : usize = 20;
pub(crate) const IDX_PARAM_VAR_NAME         : usize = 21;

pub(crate) const IDX_RESULT_CHAIN_OWNER_ID : usize = 22;
pub(crate) const IDX_RESULT_COUNTER        : usize = 23;
pub(crate) const IDX_RESULT_INT_VALUE      : usize = 24;
pub(crate) const IDX_RESULT_MINTED_COLOR   : usize = 25;
pub(crate) const IDX_RESULT_MINTED_SUPPLY  : usize = 26;
pub(crate) const IDX_RESULT_SANDBOX_CALL   : usize = 27;
pub(crate) const IDX_RESULT_VALUES         : usize = 28;
pub(crate) const IDX_RESULT_VARS           : usize = 29;

pub(crate) const IDX_STATE_COUNTER       : usize = 30;
pub(crate) const IDX_STATE_HNAME_EP      : usize = 31;
pub(crate) const IDX_STATE_INTS          : usize = 32;
pub(crate) const IDX_STATE_MINTED_COLOR  : usize = 33;
pub(crate) const IDX_STATE_MINTED_SUPPLY : usize = 34;

pub const KEY_MAP_LEN: usize = 35;

pub const KEY_MAP: [&str; KEY_MAP_LEN] = [
	PARAM_ADDRESS,
	PARAM_AGENT_ID,
	PARAM_CALLER,
	PARAM_CHAIN_ID,
	PARAM_CHAIN_OWNER_ID,
	PARAM_CONTRACT_CREATOR,
	PARAM_CONTRACT_ID,
	PARAM_COUNTER,
	PARAM_FAIL,
	PARAM_HASH,
	PARAM_HNAME,
	PARAM_HNAME_CONTRACT,
	PARAM_HNAME_EP,
	PARAM_HNAME_ZERO,
	PARAM_INT64,
	PARAM_INT64_ZERO,
	PARAM_INT_VALUE,
	PARAM_NAME,
	PARAM_PROG_HASH,
	PARAM_STRING,
	PARAM_STRING_ZERO,
	PARAM_VAR_NAME,
	RESULT_CHAIN_OWNER_ID,
	RESULT_COUNTER,
	RESULT_INT_VALUE,
	RESULT_MINTED_COLOR,
	RESULT_MINTED_SUPPLY,
	RESULT_SANDBOX_CALL,
	RESULT_VALUES,
	RESULT_VARS,
	STATE_COUNTER,
	STATE_HNAME_EP,
	STATE_INTS,
	STATE_MINTED_COLOR,
	STATE_MINTED_SUPPLY,
];

pub static mut IDX_MAP: [Key32; KEY_MAP_LEN] = [Key32(0); KEY_MAP_LEN];

pub fn idx_map(idx: usize) -> Key32 {
    unsafe {
        IDX_MAP[idx]
    }
}
