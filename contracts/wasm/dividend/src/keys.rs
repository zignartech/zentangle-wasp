// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

// @formatter:off

#![allow(dead_code)]

use wasmlib::*;

use crate::*;

pub(crate) const IDX_PARAM_ADDRESS:      usize = 0;
pub(crate) const IDX_PARAM_FACTOR:       usize = 1;
pub(crate) const IDX_PARAM_OWNER:        usize = 2;
pub(crate) const IDX_RESULT_FACTOR:      usize = 3;
pub(crate) const IDX_RESULT_OWNER:       usize = 4;
pub(crate) const IDX_STATE_MEMBER_LIST:  usize = 5;
pub(crate) const IDX_STATE_MEMBERS:      usize = 6;
pub(crate) const IDX_STATE_OWNER:        usize = 7;
pub(crate) const IDX_STATE_TOTAL_FACTOR: usize = 8;

pub const KEY_MAP_LEN: usize = 9;

pub const KEY_MAP: [&str; KEY_MAP_LEN] = [
    PARAM_ADDRESS,
    PARAM_FACTOR,
    PARAM_OWNER,
    RESULT_FACTOR,
    RESULT_OWNER,
    STATE_MEMBER_LIST,
    STATE_MEMBERS,
    STATE_OWNER,
    STATE_TOTAL_FACTOR,
];

pub static mut IDX_MAP: [Key32; KEY_MAP_LEN] = [Key32(0); KEY_MAP_LEN];

pub fn idx_map(idx: usize) -> Key32 {
    unsafe {
        IDX_MAP[idx]
    }
}

// @formatter:on
