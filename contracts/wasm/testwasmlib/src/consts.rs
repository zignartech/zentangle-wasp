// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]

use wasmlib::*;

pub const SC_NAME        : &str = "testwasmlib";
pub const SC_DESCRIPTION : &str = "Exercise several aspects of WasmLib";
pub const HSC_NAME       : ScHname = ScHname(0x89703a45);

pub const PARAM_ADDRESS      : &str = "address";
pub const PARAM_AGENT_ID     : &str = "agentID";
pub const PARAM_BLOCK_INDEX  : &str = "blockIndex";
pub const PARAM_BOOL         : &str = "bool";
pub const PARAM_BYTES        : &str = "bytes";
pub const PARAM_CHAIN_ID     : &str = "chainID";
pub const PARAM_COLOR        : &str = "color";
pub const PARAM_HASH         : &str = "hash";
pub const PARAM_HNAME        : &str = "hname";
pub const PARAM_INDEX        : &str = "index";
pub const PARAM_INT16        : &str = "int16";
pub const PARAM_INT32        : &str = "int32";
pub const PARAM_INT64        : &str = "int64";
pub const PARAM_INT8         : &str = "int8";
pub const PARAM_NAME         : &str = "name";
pub const PARAM_PARAM        : &str = "this";
pub const PARAM_RECORD_INDEX : &str = "recordIndex";
pub const PARAM_REQUEST_ID   : &str = "requestID";
pub const PARAM_STRING       : &str = "string";
pub const PARAM_UINT16       : &str = "uint16";
pub const PARAM_UINT32       : &str = "uint32";
pub const PARAM_UINT64       : &str = "uint64";
pub const PARAM_UINT8        : &str = "uint8";
pub const PARAM_VALUE        : &str = "value";

pub const RESULT_COUNT  : &str = "count";
pub const RESULT_IOTAS  : &str = "iotas";
pub const RESULT_LENGTH : &str = "length";
pub const RESULT_RANDOM : &str = "random";
pub const RESULT_RECORD : &str = "record";
pub const RESULT_VALUE  : &str = "value";

pub const STATE_ARRAYS : &str = "arrays";
pub const STATE_RANDOM : &str = "random";

pub const FUNC_ARRAY_CLEAR   : &str = "arrayClear";
pub const FUNC_ARRAY_CREATE  : &str = "arrayCreate";
pub const FUNC_ARRAY_SET     : &str = "arraySet";
pub const FUNC_PARAM_TYPES   : &str = "paramTypes";
pub const FUNC_RANDOM        : &str = "random";
pub const FUNC_TRIGGER_EVENT : &str = "triggerEvent";
pub const VIEW_ARRAY_LENGTH  : &str = "arrayLength";
pub const VIEW_ARRAY_VALUE   : &str = "arrayValue";
pub const VIEW_BLOCK_RECORD  : &str = "blockRecord";
pub const VIEW_BLOCK_RECORDS : &str = "blockRecords";
pub const VIEW_GET_RANDOM    : &str = "getRandom";
pub const VIEW_IOTA_BALANCE  : &str = "iotaBalance";

pub const HFUNC_ARRAY_CLEAR   : ScHname = ScHname(0x88021821);
pub const HFUNC_ARRAY_CREATE  : ScHname = ScHname(0x1ed5b23b);
pub const HFUNC_ARRAY_SET     : ScHname = ScHname(0x2c4150b3);
pub const HFUNC_PARAM_TYPES   : ScHname = ScHname(0x6921c4cd);
pub const HFUNC_RANDOM        : ScHname = ScHname(0xe86c97ca);
pub const HFUNC_TRIGGER_EVENT : ScHname = ScHname(0xd5438ac6);
pub const HVIEW_ARRAY_LENGTH  : ScHname = ScHname(0x3a831021);
pub const HVIEW_ARRAY_VALUE   : ScHname = ScHname(0x662dbd81);
pub const HVIEW_BLOCK_RECORD  : ScHname = ScHname(0xad13b2f8);
pub const HVIEW_BLOCK_RECORDS : ScHname = ScHname(0x16e249ea);
pub const HVIEW_GET_RANDOM    : ScHname = ScHname(0x46263045);
pub const HVIEW_IOTA_BALANCE  : ScHname = ScHname(0x9d3920bd);
