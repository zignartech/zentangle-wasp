// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]

use wasmlib::*;

pub const SC_NAME        : &str = "timestamp";
pub const SC_DESCRIPTION : &str = "Extremely simple timestamp server";
pub const HSC_NAME       : ScHname = ScHname(0x3988002e);

pub const RESULT_TIMESTAMP : &str = "timestamp";

pub const FUNC_NOW           : &str = "now";
pub const VIEW_GET_TIMESTAMP : &str = "getTimestamp";

pub const HFUNC_NOW           : ScHname = ScHname(0xd73b7fc9);
pub const HVIEW_GET_TIMESTAMP : ScHname = ScHname(0x40c6376a);
