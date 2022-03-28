// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

//nolint:dupl
package giveaway

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

func OnLoad() {
	exports := wasmlib.NewScExports()
	exports.AddFunc(FuncInit,            funcInitThunk)
	exports.AddFunc(FuncLoadAddresses,   funcLoadAddressesThunk)
	exports.AddFunc(FuncRuffle,          funcRuffleThunk)
	exports.AddFunc(FuncSetOwner,        funcSetOwnerThunk)
	exports.AddFunc(FuncUnloadAddresses, funcUnloadAddressesThunk)
	exports.AddView(ViewGetOwner,        viewGetOwnerThunk)

	for i, key := range keyMap {
		idxMap[i] = key.KeyID()
	}
}

type InitContext struct {
	Events  giveawayEvents
	Params  ImmutableInitParams
	State   MutablegiveawayState
}

func funcInitThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("giveaway.funcInit")
	f := &InitContext{
		Params: ImmutableInitParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutablegiveawayState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	funcInit(ctx, f)
	ctx.Log("giveaway.funcInit ok")
}

type LoadAddressesContext struct {
	Events  giveawayEvents
	Params  ImmutableLoadAddressesParams
	State   MutablegiveawayState
}

func funcLoadAddressesThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("giveaway.funcLoadAddresses")

	// current owner of this smart contract
	access := ctx.State().GetAgentID(wasmlib.Key("owner"))
	ctx.Require(access.Exists(), "access not set: owner")
	ctx.Require(ctx.Caller() == access.Value(), "no permission")

	f := &LoadAddressesContext{
		Params: ImmutableLoadAddressesParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutablegiveawayState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.Addresses().Exists(), "missing mandatory addresses")
	funcLoadAddresses(ctx, f)
	ctx.Log("giveaway.funcLoadAddresses ok")
}

type RuffleContext struct {
	Events  giveawayEvents
	Params  ImmutableRuffleParams
	Results MutableRuffleResults
	State   MutablegiveawayState
}

func funcRuffleThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("giveaway.funcRuffle")

	// current owner of this smart contract
	access := ctx.State().GetAgentID(wasmlib.Key("owner"))
	ctx.Require(access.Exists(), "access not set: owner")
	ctx.Require(ctx.Caller() == access.Value(), "no permission")

	f := &RuffleContext{
		Params: ImmutableRuffleParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		Results: MutableRuffleResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: MutablegiveawayState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.NWinners().Exists(), "missing mandatory nWinners")
	funcRuffle(ctx, f)
	ctx.Log("giveaway.funcRuffle ok")
}

type SetOwnerContext struct {
	Events  giveawayEvents
	Params  ImmutableSetOwnerParams
	State   MutablegiveawayState
}

func funcSetOwnerThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("giveaway.funcSetOwner")

	// current owner of this smart contract
	access := ctx.State().GetAgentID(wasmlib.Key("owner"))
	ctx.Require(access.Exists(), "access not set: owner")
	ctx.Require(ctx.Caller() == access.Value(), "no permission")

	f := &SetOwnerContext{
		Params: ImmutableSetOwnerParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutablegiveawayState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.Owner().Exists(), "missing mandatory owner")
	funcSetOwner(ctx, f)
	ctx.Log("giveaway.funcSetOwner ok")
}

type UnloadAddressesContext struct {
	Events  giveawayEvents
	State   MutablegiveawayState
}

func funcUnloadAddressesThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("giveaway.funcUnloadAddresses")

	// current owner of this smart contract
	access := ctx.State().GetAgentID(wasmlib.Key("owner"))
	ctx.Require(access.Exists(), "access not set: owner")
	ctx.Require(ctx.Caller() == access.Value(), "no permission")

	f := &UnloadAddressesContext{
		State: MutablegiveawayState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	funcUnloadAddresses(ctx, f)
	ctx.Log("giveaway.funcUnloadAddresses ok")
}

type GetOwnerContext struct {
	Results MutableGetOwnerResults
	State   ImmutablegiveawayState
}

func viewGetOwnerThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("giveaway.viewGetOwner")
	f := &GetOwnerContext{
		Results: MutableGetOwnerResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: ImmutablegiveawayState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	viewGetOwner(ctx, f)
	ctx.Log("giveaway.viewGetOwner ok")
}
