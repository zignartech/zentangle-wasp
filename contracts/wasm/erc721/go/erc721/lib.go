// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package erc721

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

var exportMap = wasmlib.ScExportMap{
	Names: []string{
		FuncApprove,
		FuncBurn,
		FuncInit,
		FuncMint,
		FuncSafeTransferFrom,
		FuncSetApprovalForAll,
		FuncTransferFrom,
		ViewBalanceOf,
		ViewGetApproved,
		ViewIsApprovedForAll,
		ViewName,
		ViewOwnerOf,
		ViewSymbol,
		ViewTokenURI,
	},
	Funcs: []wasmlib.ScFuncContextFunction{
		funcApproveThunk,
		funcBurnThunk,
		funcInitThunk,
		funcMintThunk,
		funcSafeTransferFromThunk,
		funcSetApprovalForAllThunk,
		funcTransferFromThunk,
	},
	Views: []wasmlib.ScViewContextFunction{
		viewBalanceOfThunk,
		viewGetApprovedThunk,
		viewIsApprovedForAllThunk,
		viewNameThunk,
		viewOwnerOfThunk,
		viewSymbolThunk,
		viewTokenURIThunk,
	},
}

func OnLoad(index int32) {
	if index >= 0 {
		wasmlib.ScExportsCall(index, &exportMap)
		return
	}

	wasmlib.ScExportsExport(&exportMap)
}

type ApproveContext struct {
	Events Erc721Events
	Params ImmutableApproveParams
	State  MutableErc721State
}

func funcApproveThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("erc721.funcApprove")
	f := &ApproveContext{
		Params: ImmutableApproveParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.TokenID().Exists(), "missing mandatory tokenID")
	funcApprove(ctx, f)
	ctx.Log("erc721.funcApprove ok")
}

type BurnContext struct {
	Events Erc721Events
	Params ImmutableBurnParams
	State  MutableErc721State
}

func funcBurnThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("erc721.funcBurn")
	f := &BurnContext{
		Params: ImmutableBurnParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.TokenID().Exists(), "missing mandatory tokenID")
	funcBurn(ctx, f)
	ctx.Log("erc721.funcBurn ok")
}

type InitContext struct {
	Events Erc721Events
	Params ImmutableInitParams
	State  MutableErc721State
}

func funcInitThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("erc721.funcInit")
	f := &InitContext{
		Params: ImmutableInitParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Name().Exists(), "missing mandatory name")
	ctx.Require(f.Params.Symbol().Exists(), "missing mandatory symbol")
	funcInit(ctx, f)
	ctx.Log("erc721.funcInit ok")
}

type MintContext struct {
	Events Erc721Events
	Params ImmutableMintParams
	State  MutableErc721State
}

func funcMintThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("erc721.funcMint")
	f := &MintContext{
		Params: ImmutableMintParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.TokenID().Exists(), "missing mandatory tokenID")
	funcMint(ctx, f)
	ctx.Log("erc721.funcMint ok")
}

type SafeTransferFromContext struct {
	Events Erc721Events
	Params ImmutableSafeTransferFromParams
	State  MutableErc721State
}

func funcSafeTransferFromThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("erc721.funcSafeTransferFrom")
	f := &SafeTransferFromContext{
		Params: ImmutableSafeTransferFromParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.From().Exists(), "missing mandatory from")
	ctx.Require(f.Params.To().Exists(), "missing mandatory to")
	ctx.Require(f.Params.TokenID().Exists(), "missing mandatory tokenID")
	funcSafeTransferFrom(ctx, f)
	ctx.Log("erc721.funcSafeTransferFrom ok")
}

type SetApprovalForAllContext struct {
	Events Erc721Events
	Params ImmutableSetApprovalForAllParams
	State  MutableErc721State
}

func funcSetApprovalForAllThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("erc721.funcSetApprovalForAll")
	f := &SetApprovalForAllContext{
		Params: ImmutableSetApprovalForAllParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Approval().Exists(), "missing mandatory approval")
	ctx.Require(f.Params.Operator().Exists(), "missing mandatory operator")
	funcSetApprovalForAll(ctx, f)
	ctx.Log("erc721.funcSetApprovalForAll ok")
}

type TransferFromContext struct {
	Events Erc721Events
	Params ImmutableTransferFromParams
	State  MutableErc721State
}

func funcTransferFromThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("erc721.funcTransferFrom")
	f := &TransferFromContext{
		Params: ImmutableTransferFromParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.From().Exists(), "missing mandatory from")
	ctx.Require(f.Params.To().Exists(), "missing mandatory to")
	ctx.Require(f.Params.TokenID().Exists(), "missing mandatory tokenID")
	funcTransferFrom(ctx, f)
	ctx.Log("erc721.funcTransferFrom ok")
}

type BalanceOfContext struct {
	Params  ImmutableBalanceOfParams
	Results MutableBalanceOfResults
	State   ImmutableErc721State
}

func viewBalanceOfThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("erc721.viewBalanceOf")
	results := wasmlib.NewScDict()
	f := &BalanceOfContext{
		Params: ImmutableBalanceOfParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableBalanceOfResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Owner().Exists(), "missing mandatory owner")
	viewBalanceOf(ctx, f)
	ctx.Results(results)
	ctx.Log("erc721.viewBalanceOf ok")
}

type GetApprovedContext struct {
	Params  ImmutableGetApprovedParams
	Results MutableGetApprovedResults
	State   ImmutableErc721State
}

func viewGetApprovedThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("erc721.viewGetApproved")
	results := wasmlib.NewScDict()
	f := &GetApprovedContext{
		Params: ImmutableGetApprovedParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableGetApprovedResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.TokenID().Exists(), "missing mandatory tokenID")
	viewGetApproved(ctx, f)
	ctx.Results(results)
	ctx.Log("erc721.viewGetApproved ok")
}

type IsApprovedForAllContext struct {
	Params  ImmutableIsApprovedForAllParams
	Results MutableIsApprovedForAllResults
	State   ImmutableErc721State
}

func viewIsApprovedForAllThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("erc721.viewIsApprovedForAll")
	results := wasmlib.NewScDict()
	f := &IsApprovedForAllContext{
		Params: ImmutableIsApprovedForAllParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableIsApprovedForAllResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Operator().Exists(), "missing mandatory operator")
	ctx.Require(f.Params.Owner().Exists(), "missing mandatory owner")
	viewIsApprovedForAll(ctx, f)
	ctx.Results(results)
	ctx.Log("erc721.viewIsApprovedForAll ok")
}

type NameContext struct {
	Results MutableNameResults
	State   ImmutableErc721State
}

func viewNameThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("erc721.viewName")
	results := wasmlib.NewScDict()
	f := &NameContext{
		Results: MutableNameResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewName(ctx, f)
	ctx.Results(results)
	ctx.Log("erc721.viewName ok")
}

type OwnerOfContext struct {
	Params  ImmutableOwnerOfParams
	Results MutableOwnerOfResults
	State   ImmutableErc721State
}

func viewOwnerOfThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("erc721.viewOwnerOf")
	results := wasmlib.NewScDict()
	f := &OwnerOfContext{
		Params: ImmutableOwnerOfParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableOwnerOfResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.TokenID().Exists(), "missing mandatory tokenID")
	viewOwnerOf(ctx, f)
	ctx.Results(results)
	ctx.Log("erc721.viewOwnerOf ok")
}

type SymbolContext struct {
	Results MutableSymbolResults
	State   ImmutableErc721State
}

func viewSymbolThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("erc721.viewSymbol")
	results := wasmlib.NewScDict()
	f := &SymbolContext{
		Results: MutableSymbolResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewSymbol(ctx, f)
	ctx.Results(results)
	ctx.Log("erc721.viewSymbol ok")
}

type TokenURIContext struct {
	Params  ImmutableTokenURIParams
	Results MutableTokenURIResults
	State   ImmutableErc721State
}

func viewTokenURIThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("erc721.viewTokenURI")
	results := wasmlib.NewScDict()
	f := &TokenURIContext{
		Params: ImmutableTokenURIParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableTokenURIResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableErc721State{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.TokenID().Exists(), "missing mandatory tokenID")
	viewTokenURI(ctx, f)
	ctx.Results(results)
	ctx.Log("erc721.viewTokenURI ok")
}
