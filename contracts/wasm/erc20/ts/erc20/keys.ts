// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib"
import * as sc from "./index";

export const IdxParamAccount       = 0;
export const IdxParamAmount        = 1;
export const IdxParamCreator       = 2;
export const IdxParamDelegation    = 3;
export const IdxParamRecipient     = 4;
export const IdxParamSupply        = 5;
export const IdxResultAmount       = 6;
export const IdxResultSupply       = 7;
export const IdxStateAllAllowances = 8;
export const IdxStateBalances      = 9;
export const IdxStateSupply        = 10;

export let keyMap: string[] = [
    sc.ParamAccount,
    sc.ParamAmount,
    sc.ParamCreator,
    sc.ParamDelegation,
    sc.ParamRecipient,
    sc.ParamSupply,
    sc.ResultAmount,
    sc.ResultSupply,
    sc.StateAllAllowances,
    sc.StateBalances,
    sc.StateSupply,
];

export let idxMap: wasmlib.Key32[] = new Array(keyMap.length);
