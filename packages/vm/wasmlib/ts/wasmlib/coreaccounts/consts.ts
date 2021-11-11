// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";

export const ScName        = "accounts";
export const ScDescription = "Core chain account ledger contract";
export const HScName       = new wasmlib.ScHname(0x3c4b5e02);

export const ParamAgentID        = "a";
export const ParamWithdrawAmount = "m";
export const ParamWithdrawColor  = "c";

export const ResultAccountNonce = "n";
export const ResultAgents       = "this";
export const ResultBalances     = "this";

export const FuncDeposit         = "deposit";
export const FuncHarvest         = "harvest";
export const FuncWithdraw        = "withdraw";
export const ViewAccounts        = "accounts";
export const ViewBalance         = "balance";
export const ViewGetAccountNonce = "getAccountNonce";
export const ViewTotalAssets     = "totalAssets";

export const HFuncDeposit         = new wasmlib.ScHname(0xbdc9102d);
export const HFuncHarvest         = new wasmlib.ScHname(0x7b40efbd);
export const HFuncWithdraw        = new wasmlib.ScHname(0x9dcc0f41);
export const HViewAccounts        = new wasmlib.ScHname(0x3c4b5e02);
export const HViewBalance         = new wasmlib.ScHname(0x84168cb4);
export const HViewGetAccountNonce = new wasmlib.ScHname(0x529d7df9);
export const HViewTotalAssets     = new wasmlib.ScHname(0xfab0f8d2);
