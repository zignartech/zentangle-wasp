// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib"

export const ScName  = "tokenregistry";
export const HScName = new wasmlib.ScHname(0xe1ba0c78);

export const ParamColor       = "color";
export const ParamDescription = "description";
export const ParamUserDefined = "userDefined";

export const StateColorList = "colorList";
export const StateRegistry  = "registry";

export const FuncMintSupply        = "mintSupply";
export const FuncTransferOwnership = "transferOwnership";
export const FuncUpdateMetadata    = "updateMetadata";
export const ViewGetInfo           = "getInfo";

export const HFuncMintSupply        = new wasmlib.ScHname(0x564349a7);
export const HFuncTransferOwnership = new wasmlib.ScHname(0xbb9eb5af);
export const HFuncUpdateMetadata    = new wasmlib.ScHname(0xa26b23b6);
export const HViewGetInfo           = new wasmlib.ScHname(0xcfedba5f);
