// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package dividend

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

func OnLoad() {
	exports := wasmlib.NewScExports()
	exports.AddFunc(FuncDivide, funcDivideThunk)
	exports.AddFunc(FuncInit, funcInitThunk)
	exports.AddFunc(FuncMember, funcMemberThunk)
	exports.AddFunc(FuncSetOwner, funcSetOwnerThunk)
	exports.AddView(ViewGetFactor, viewGetFactorThunk)
	exports.AddView(ViewGetOwner, viewGetOwnerThunk)

	for i, key := range keyMap {
		idxMap[i] = key.KeyID()
	}
}

type DivideContext struct {
	State MutableDividendState
}

func funcDivideThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("dividend.funcDivide")
	f := &DivideContext{
		State: MutableDividendState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	funcDivide(ctx, f)
	ctx.Log("dividend.funcDivide ok")
}

type InitContext struct {
	Params ImmutableInitParams
	State  MutableDividendState
}

func funcInitThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("dividend.funcInit")
	f := &InitContext{
		Params: ImmutableInitParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutableDividendState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	funcInit(ctx, f)
	ctx.Log("dividend.funcInit ok")
}

type MemberContext struct {
	Params ImmutableMemberParams
	State  MutableDividendState
}

func funcMemberThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("dividend.funcMember")
	// only defined owner of contract can add members
	access := ctx.State().GetAgentID(wasmlib.Key("owner"))
	ctx.Require(access.Exists(), "access not set: owner")
	ctx.Require(ctx.Caller() == access.Value(), "no permission")

	f := &MemberContext{
		Params: ImmutableMemberParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutableDividendState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.Address().Exists(), "missing mandatory address")
	ctx.Require(f.Params.Factor().Exists(), "missing mandatory factor")
	funcMember(ctx, f)
	ctx.Log("dividend.funcMember ok")
}

type SetOwnerContext struct {
	Params ImmutableSetOwnerParams
	State  MutableDividendState
}

func funcSetOwnerThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("dividend.funcSetOwner")
	// only defined owner of contract can change owner
	access := ctx.State().GetAgentID(wasmlib.Key("owner"))
	ctx.Require(access.Exists(), "access not set: owner")
	ctx.Require(ctx.Caller() == access.Value(), "no permission")

	f := &SetOwnerContext{
		Params: ImmutableSetOwnerParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutableDividendState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.Owner().Exists(), "missing mandatory owner")
	funcSetOwner(ctx, f)
	ctx.Log("dividend.funcSetOwner ok")
}

type GetFactorContext struct {
	Params  ImmutableGetFactorParams
	Results MutableGetFactorResults
	State   ImmutableDividendState
}

func viewGetFactorThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("dividend.viewGetFactor")
	f := &GetFactorContext{
		Params: ImmutableGetFactorParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		Results: MutableGetFactorResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: ImmutableDividendState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.Address().Exists(), "missing mandatory address")
	viewGetFactor(ctx, f)
	ctx.Log("dividend.viewGetFactor ok")
}

type GetOwnerContext struct {
	Results MutableGetOwnerResults
	State   ImmutableDividendState
}

func viewGetOwnerThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("dividend.viewGetOwner")
	f := &GetOwnerContext{
		Results: MutableGetOwnerResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: ImmutableDividendState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	viewGetOwner(ctx, f)
	ctx.Log("dividend.viewGetOwner ok")
}
