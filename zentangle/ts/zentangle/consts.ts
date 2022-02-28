// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";

export const ScName        = "zentangle";
export const ScDescription = "Incentivized AI Training Casino";
export const HScName       = new wasmlib.ScHname(0xf707a9c6);

export const ParamDescription          = "description";
export const ParamImageId              = "imageId";
export const ParamInputJson            = "inputJson";
export const ParamNumberOfImages       = "numberOfImages";
export const ParamOwner                = "owner";
export const ParamPlayerAddress        = "playerAddress";
export const ParamResetPlayerInfo      = "resetPlayerInfo";
export const ParamTagsRequiredPerImage = "tagsRequiredPerImage";

export const ResultImageId       = "imageId";
export const ResultInfo          = "info";
export const ResultOwner         = "owner";
export const ResultPlayerBets    = "playerBets";
export const ResultPlaysPerImage = "playsPerImage";
export const ResultResults       = "results";

export const StateBets                 = "bets";
export const StateCreator              = "creator";
export const StateDescription          = "description";
export const StateNumberOfImages       = "numberOfImages";
export const StateOwner                = "owner";
export const StatePendingPlay          = "pendingPlay";
export const StatePlayer               = "player";
export const StatePlaysPerImage        = "playsPerImage";
export const StateProcessedImages      = "processedImages";
export const StateReward               = "reward";
export const StateTaggedImages         = "taggedImages";
export const StateTagsRequiredPerImage = "tagsRequiredPerImage";
export const StateValidTags            = "validTags";

export const FuncCreateGame       = "createGame";
export const FuncEndGame          = "endGame";
export const FuncInit             = "init";
export const FuncRequestPlay      = "requestPlay";
export const FuncSendTags         = "sendTags";
export const FuncSetOwner         = "setOwner";
export const FuncWithdraw         = "withdraw";
export const ViewGetOwner         = "getOwner";
export const ViewGetPlayerBets    = "getPlayerBets";
export const ViewGetPlayerInfo    = "getPlayerInfo";
export const ViewGetPlaysPerImage = "getPlaysPerImage";
export const ViewGetResults       = "getResults";

export const HFuncCreateGame       = new wasmlib.ScHname(0x585dcce2);
export const HFuncEndGame          = new wasmlib.ScHname(0xb2303ef2);
export const HFuncInit             = new wasmlib.ScHname(0x1f44d644);
export const HFuncRequestPlay      = new wasmlib.ScHname(0x74f0bf82);
export const HFuncSendTags         = new wasmlib.ScHname(0xc31816cb);
export const HFuncSetOwner         = new wasmlib.ScHname(0x2a15fe7b);
export const HFuncWithdraw         = new wasmlib.ScHname(0x9dcc0f41);
export const HViewGetOwner         = new wasmlib.ScHname(0x137107a6);
export const HViewGetPlayerBets    = new wasmlib.ScHname(0x842b0ef5);
export const HViewGetPlayerInfo    = new wasmlib.ScHname(0x504151da);
export const HViewGetPlaysPerImage = new wasmlib.ScHname(0x749519e8);
export const HViewGetResults       = new wasmlib.ScHname(0xc2ed9edb);