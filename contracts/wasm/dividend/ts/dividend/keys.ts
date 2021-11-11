// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";
import * as sc from "./index";

export const IdxParamAddress = 0;
export const IdxParamFactor  = 1;
export const IdxParamOwner   = 2;

export const IdxResultFactor = 3;
export const IdxResultOwner  = 4;

export const IdxStateMemberList  = 5;
export const IdxStateMembers     = 6;
export const IdxStateOwner       = 7;
export const IdxStateTotalFactor = 8;

export let keyMap: string[] = [
	sc.ParamAddress,
	sc.ParamFactor,
	sc.ParamOwner,
	sc.ResultFactor,
	sc.ResultOwner,
	sc.StateMemberList,
	sc.StateMembers,
	sc.StateOwner,
	sc.StateTotalFactor,
];

export let idxMap: wasmlib.Key32[] = new Array(keyMap.length);
