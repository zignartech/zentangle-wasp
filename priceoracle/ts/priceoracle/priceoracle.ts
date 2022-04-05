// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import * as wasmlib from "wasmlib";
import * as wasmtypes from "wasmlib/wasmtypes";
import * as sc from "./index";

export function funcApproveOracle(ctx: wasmlib.ScFuncContext, f: sc.ApproveOracleContext): void {
}

export function funcInit(ctx: wasmlib.ScFuncContext, f: sc.InitContext): void {
	if (f.params.owner().exists()) {
		f.state.owner().setValue(f.params.owner().value());
		return;
	}
	f.state.owner().setValue(ctx.contractCreator());
}

export function funcSetMiotaPrice(ctx: wasmlib.ScFuncContext, f: sc.SetMiotaPriceContext): void {
}

export function funcSetOwner(ctx: wasmlib.ScFuncContext, f: sc.SetOwnerContext): void {
	f.state.owner().setValue(f.params.owner().value());
}

export function viewGetMiotaPrice(ctx: wasmlib.ScViewContext, f: sc.GetMiotaPriceContext): void {
}

export function viewGetOwner(ctx: wasmlib.ScViewContext, f: sc.GetOwnerContext): void {
	f.results.owner().setValue(f.state.owner().value());
}
