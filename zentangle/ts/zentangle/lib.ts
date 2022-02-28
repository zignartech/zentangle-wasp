// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";
import * as sc from "./index";

export function on_call(index: i32): void {
    return wasmlib.onCall(index);
}

export function on_load(): void {
    let exports = new wasmlib.ScExports();
    exports.addFunc(sc.FuncCreateGame,       funcCreateGameThunk);
    exports.addFunc(sc.FuncEndGame,          funcEndGameThunk);
    exports.addFunc(sc.FuncInit,             funcInitThunk);
    exports.addFunc(sc.FuncRequestPlay,      funcRequestPlayThunk);
    exports.addFunc(sc.FuncSendTags,         funcSendTagsThunk);
    exports.addFunc(sc.FuncSetOwner,         funcSetOwnerThunk);
    exports.addFunc(sc.FuncWithdraw,         funcWithdrawThunk);
    exports.addView(sc.ViewGetOwner,         viewGetOwnerThunk);
    exports.addView(sc.ViewGetPlayerBets,    viewGetPlayerBetsThunk);
    exports.addView(sc.ViewGetPlayerInfo,    viewGetPlayerInfoThunk);
    exports.addView(sc.ViewGetPlaysPerImage, viewGetPlaysPerImageThunk);
    exports.addView(sc.ViewGetResults,       viewGetResultsThunk);

    for (let i = 0; i < sc.keyMap.length; i++) {
        sc.idxMap[i] = wasmlib.Key32.fromString(sc.keyMap[i]);
    }
}

function funcCreateGameThunk(ctx: wasmlib.ScFuncContext): void {
	ctx.log("zentangle.funcCreateGame");
	let f = new sc.CreateGameContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	ctx.require(f.params.description().exists(), "missing mandatory description");
	ctx.require(f.params.numberOfImages().exists(), "missing mandatory numberOfImages");
	sc.funcCreateGame(ctx, f);
	ctx.log("zentangle.funcCreateGame ok");
}

function funcEndGameThunk(ctx: wasmlib.ScFuncContext): void {
	ctx.log("zentangle.funcEndGame");
	let f = new sc.EndGameContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	sc.funcEndGame(ctx, f);
	ctx.log("zentangle.funcEndGame ok");
}

function funcInitThunk(ctx: wasmlib.ScFuncContext): void {
	ctx.log("zentangle.funcInit");
	let f = new sc.InitContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	sc.funcInit(ctx, f);
	ctx.log("zentangle.funcInit ok");
}

function funcRequestPlayThunk(ctx: wasmlib.ScFuncContext): void {
	ctx.log("zentangle.funcRequestPlay");
	let f = new sc.RequestPlayContext();
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	sc.funcRequestPlay(ctx, f);
	ctx.log("zentangle.funcRequestPlay ok");
}

function funcSendTagsThunk(ctx: wasmlib.ScFuncContext): void {
	ctx.log("zentangle.funcSendTags");
	let f = new sc.SendTagsContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	ctx.require(f.params.inputJson().exists(), "missing mandatory inputJson");
	sc.funcSendTags(ctx, f);
	ctx.log("zentangle.funcSendTags ok");
}

function funcSetOwnerThunk(ctx: wasmlib.ScFuncContext): void {
	ctx.log("zentangle.funcSetOwner");

	// current owner of this smart contract
	let access = ctx.state().getAgentID(wasmlib.Key32.fromString("owner"));
	ctx.require(access.exists(), "access not set: owner");
	ctx.require(ctx.caller().equals(access.value()), "no permission");

	let f = new sc.SetOwnerContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	ctx.require(f.params.owner().exists(), "missing mandatory owner");
	sc.funcSetOwner(ctx, f);
	ctx.log("zentangle.funcSetOwner ok");
}

function funcWithdrawThunk(ctx: wasmlib.ScFuncContext): void {
	ctx.log("zentangle.funcWithdraw");

	// current owner of this smart contract
	let access = ctx.state().getAgentID(wasmlib.Key32.fromString("owner"));
	ctx.require(access.exists(), "access not set: owner");
	ctx.require(ctx.caller().equals(access.value()), "no permission");

	let f = new sc.WithdrawContext();
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	sc.funcWithdraw(ctx, f);
	ctx.log("zentangle.funcWithdraw ok");
}

function viewGetOwnerThunk(ctx: wasmlib.ScViewContext): void {
	ctx.log("zentangle.viewGetOwner");
	let f = new sc.GetOwnerContext();
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	sc.viewGetOwner(ctx, f);
	ctx.log("zentangle.viewGetOwner ok");
}

function viewGetPlayerBetsThunk(ctx: wasmlib.ScViewContext): void {
	ctx.log("zentangle.viewGetPlayerBets");
	let f = new sc.GetPlayerBetsContext();
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	sc.viewGetPlayerBets(ctx, f);
	ctx.log("zentangle.viewGetPlayerBets ok");
}

function viewGetPlayerInfoThunk(ctx: wasmlib.ScViewContext): void {
	ctx.log("zentangle.viewGetPlayerInfo");
	let f = new sc.GetPlayerInfoContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	ctx.require(f.params.playerAddress().exists(), "missing mandatory playerAddress");
	sc.viewGetPlayerInfo(ctx, f);
	ctx.log("zentangle.viewGetPlayerInfo ok");
}

function viewGetPlaysPerImageThunk(ctx: wasmlib.ScViewContext): void {
	ctx.log("zentangle.viewGetPlaysPerImage");
	let f = new sc.GetPlaysPerImageContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	ctx.require(f.params.imageId().exists(), "missing mandatory imageId");
	sc.viewGetPlaysPerImage(ctx, f);
	ctx.log("zentangle.viewGetPlaysPerImage ok");
}

function viewGetResultsThunk(ctx: wasmlib.ScViewContext): void {
	ctx.log("zentangle.viewGetResults");
	let f = new sc.GetResultsContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
	ctx.require(f.params.imageId().exists(), "missing mandatory imageId");
	sc.viewGetResults(ctx, f);
	ctx.log("zentangle.viewGetResults ok");
}