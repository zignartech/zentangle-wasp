// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import * as wasmlib from "wasmlib";
import * as wasmtypes from "wasmlib/wasmtypes";
import * as sc from "./index";

export function funcCreateGame(ctx: wasmlib.ScFuncContext, f: sc.CreateGameContext): void {
}

export function funcEndGame(ctx: wasmlib.ScFuncContext, f: sc.EndGameContext): void {
}

export function funcInit(ctx: wasmlib.ScFuncContext, f: sc.InitContext): void {
	if (f.params.owner().exists()) {
		f.state.owner().setValue(f.params.owner().value());
		return;
	}
	f.state.owner().setValue(ctx.contractCreator());
}

export function funcRequestPlay(ctx: wasmlib.ScFuncContext, f: sc.RequestPlayContext): void {
}

export function funcSendTags(ctx: wasmlib.ScFuncContext, f: sc.SendTagsContext): void {
}

export function funcSetOwner(ctx: wasmlib.ScFuncContext, f: sc.SetOwnerContext): void {
	f.state.owner().setValue(f.params.owner().value());
}

export function funcWithdraw(ctx: wasmlib.ScFuncContext, f: sc.WithdrawContext): void {
}

export function viewGetOwner(ctx: wasmlib.ScViewContext, f: sc.GetOwnerContext): void {
	f.results.owner().setValue(f.state.owner().value());
}

export function viewGetPlayerBets(ctx: wasmlib.ScViewContext, f: sc.GetPlayerBetsContext): void {
}

export function viewGetPlayerInfo(ctx: wasmlib.ScViewContext, f: sc.GetPlayerInfoContext): void {
}

export function viewGetPlaysPerImage(ctx: wasmlib.ScViewContext, f: sc.GetPlaysPerImageContext): void {
}

export function viewGetResults(ctx: wasmlib.ScViewContext, f: sc.GetResultsContext): void {
}
