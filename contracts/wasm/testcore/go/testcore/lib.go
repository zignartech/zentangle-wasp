// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

//nolint:dupl
package testcore

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

var exportMap = wasmlib.ScExportMap{
	Names: []string{
		FuncCallOnChain,
		FuncCheckContextFromFullEP,
		FuncDoNothing,
		FuncGetMintedSupply,
		FuncIncCounter,
		FuncInit,
		FuncPassTypesFull,
		FuncRunRecursion,
		FuncSendToAddress,
		FuncSetInt,
		FuncSpawn,
		FuncTestBlockContext1,
		FuncTestBlockContext2,
		FuncTestCallPanicFullEP,
		FuncTestCallPanicViewEPFromFull,
		FuncTestChainOwnerIDFull,
		FuncTestEventLogDeploy,
		FuncTestEventLogEventData,
		FuncTestEventLogGenericData,
		FuncTestPanicFullEP,
		FuncWithdrawToChain,
		ViewCheckContextFromViewEP,
		ViewFibonacci,
		ViewGetCounter,
		ViewGetInt,
		ViewGetStringValue,
		ViewJustView,
		ViewPassTypesView,
		ViewTestCallPanicViewEPFromView,
		ViewTestChainOwnerIDView,
		ViewTestPanicViewEP,
		ViewTestSandboxCall,
	},
	Funcs: []wasmlib.ScFuncContextFunction{
		funcCallOnChainThunk,
		funcCheckContextFromFullEPThunk,
		funcDoNothingThunk,
		funcGetMintedSupplyThunk,
		funcIncCounterThunk,
		funcInitThunk,
		funcPassTypesFullThunk,
		funcRunRecursionThunk,
		funcSendToAddressThunk,
		funcSetIntThunk,
		funcSpawnThunk,
		funcTestBlockContext1Thunk,
		funcTestBlockContext2Thunk,
		funcTestCallPanicFullEPThunk,
		funcTestCallPanicViewEPFromFullThunk,
		funcTestChainOwnerIDFullThunk,
		funcTestEventLogDeployThunk,
		funcTestEventLogEventDataThunk,
		funcTestEventLogGenericDataThunk,
		funcTestPanicFullEPThunk,
		funcWithdrawToChainThunk,
	},
	Views: []wasmlib.ScViewContextFunction{
		viewCheckContextFromViewEPThunk,
		viewFibonacciThunk,
		viewGetCounterThunk,
		viewGetIntThunk,
		viewGetStringValueThunk,
		viewJustViewThunk,
		viewPassTypesViewThunk,
		viewTestCallPanicViewEPFromViewThunk,
		viewTestChainOwnerIDViewThunk,
		viewTestPanicViewEPThunk,
		viewTestSandboxCallThunk,
	},
}

func OnLoad(index int32) {
	if index >= 0 {
		wasmlib.ScExportsCall(index, &exportMap)
		return
	}

	wasmlib.ScExportsExport(&exportMap)
}

type CallOnChainContext struct {
	Params  ImmutableCallOnChainParams
	Results MutableCallOnChainResults
	State   MutableTestCoreState
}

func funcCallOnChainThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcCallOnChain")
	results := wasmlib.NewScDict()
	f := &CallOnChainContext{
		Params: ImmutableCallOnChainParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableCallOnChainResults{
			proxy: results.AsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.IntValue().Exists(), "missing mandatory intValue")
	funcCallOnChain(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.funcCallOnChain ok")
}

type CheckContextFromFullEPContext struct {
	Params ImmutableCheckContextFromFullEPParams
	State  MutableTestCoreState
}

func funcCheckContextFromFullEPThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcCheckContextFromFullEP")
	f := &CheckContextFromFullEPContext{
		Params: ImmutableCheckContextFromFullEPParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.AgentID().Exists(), "missing mandatory agentID")
	ctx.Require(f.Params.Caller().Exists(), "missing mandatory caller")
	ctx.Require(f.Params.ChainID().Exists(), "missing mandatory chainID")
	ctx.Require(f.Params.ChainOwnerID().Exists(), "missing mandatory chainOwnerID")
	ctx.Require(f.Params.ContractCreator().Exists(), "missing mandatory contractCreator")
	funcCheckContextFromFullEP(ctx, f)
	ctx.Log("testcore.funcCheckContextFromFullEP ok")
}

type DoNothingContext struct {
	State MutableTestCoreState
}

func funcDoNothingThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcDoNothing")
	f := &DoNothingContext{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcDoNothing(ctx, f)
	ctx.Log("testcore.funcDoNothing ok")
}

type GetMintedSupplyContext struct {
	Results MutableGetMintedSupplyResults
	State   MutableTestCoreState
}

func funcGetMintedSupplyThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcGetMintedSupply")
	results := wasmlib.NewScDict()
	f := &GetMintedSupplyContext{
		Results: MutableGetMintedSupplyResults{
			proxy: results.AsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcGetMintedSupply(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.funcGetMintedSupply ok")
}

type IncCounterContext struct {
	State MutableTestCoreState
}

func funcIncCounterThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcIncCounter")
	f := &IncCounterContext{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcIncCounter(ctx, f)
	ctx.Log("testcore.funcIncCounter ok")
}

type InitContext struct {
	Params ImmutableInitParams
	State  MutableTestCoreState
}

func funcInitThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcInit")
	f := &InitContext{
		Params: ImmutableInitParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcInit(ctx, f)
	ctx.Log("testcore.funcInit ok")
}

type PassTypesFullContext struct {
	Params ImmutablePassTypesFullParams
	State  MutableTestCoreState
}

func funcPassTypesFullThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcPassTypesFull")
	f := &PassTypesFullContext{
		Params: ImmutablePassTypesFullParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Address().Exists(), "missing mandatory address")
	ctx.Require(f.Params.AgentID().Exists(), "missing mandatory agentID")
	ctx.Require(f.Params.ChainID().Exists(), "missing mandatory chainID")
	ctx.Require(f.Params.ContractID().Exists(), "missing mandatory contractID")
	ctx.Require(f.Params.Hash().Exists(), "missing mandatory hash")
	ctx.Require(f.Params.Hname().Exists(), "missing mandatory hname")
	ctx.Require(f.Params.HnameZero().Exists(), "missing mandatory hnameZero")
	ctx.Require(f.Params.Int64().Exists(), "missing mandatory int64")
	ctx.Require(f.Params.Int64Zero().Exists(), "missing mandatory int64Zero")
	ctx.Require(f.Params.String().Exists(), "missing mandatory string")
	ctx.Require(f.Params.StringZero().Exists(), "missing mandatory stringZero")
	funcPassTypesFull(ctx, f)
	ctx.Log("testcore.funcPassTypesFull ok")
}

type RunRecursionContext struct {
	Params  ImmutableRunRecursionParams
	Results MutableRunRecursionResults
	State   MutableTestCoreState
}

func funcRunRecursionThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcRunRecursion")
	results := wasmlib.NewScDict()
	f := &RunRecursionContext{
		Params: ImmutableRunRecursionParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableRunRecursionResults{
			proxy: results.AsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.IntValue().Exists(), "missing mandatory intValue")
	funcRunRecursion(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.funcRunRecursion ok")
}

type SendToAddressContext struct {
	Params ImmutableSendToAddressParams
	State  MutableTestCoreState
}

func funcSendToAddressThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcSendToAddress")
	f := &SendToAddressContext{
		Params: ImmutableSendToAddressParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(ctx.Caller() == ctx.ContractCreator(), "no permission")

	ctx.Require(f.Params.Address().Exists(), "missing mandatory address")
	funcSendToAddress(ctx, f)
	ctx.Log("testcore.funcSendToAddress ok")
}

type SetIntContext struct {
	Params ImmutableSetIntParams
	State  MutableTestCoreState
}

func funcSetIntThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcSetInt")
	f := &SetIntContext{
		Params: ImmutableSetIntParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.IntValue().Exists(), "missing mandatory intValue")
	ctx.Require(f.Params.Name().Exists(), "missing mandatory name")
	funcSetInt(ctx, f)
	ctx.Log("testcore.funcSetInt ok")
}

type SpawnContext struct {
	Params ImmutableSpawnParams
	State  MutableTestCoreState
}

func funcSpawnThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcSpawn")
	f := &SpawnContext{
		Params: ImmutableSpawnParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.ProgHash().Exists(), "missing mandatory progHash")
	funcSpawn(ctx, f)
	ctx.Log("testcore.funcSpawn ok")
}

type TestBlockContext1Context struct {
	State MutableTestCoreState
}

func funcTestBlockContext1Thunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestBlockContext1")
	f := &TestBlockContext1Context{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcTestBlockContext1(ctx, f)
	ctx.Log("testcore.funcTestBlockContext1 ok")
}

type TestBlockContext2Context struct {
	State MutableTestCoreState
}

func funcTestBlockContext2Thunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestBlockContext2")
	f := &TestBlockContext2Context{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcTestBlockContext2(ctx, f)
	ctx.Log("testcore.funcTestBlockContext2 ok")
}

type TestCallPanicFullEPContext struct {
	State MutableTestCoreState
}

func funcTestCallPanicFullEPThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestCallPanicFullEP")
	f := &TestCallPanicFullEPContext{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcTestCallPanicFullEP(ctx, f)
	ctx.Log("testcore.funcTestCallPanicFullEP ok")
}

type TestCallPanicViewEPFromFullContext struct {
	State MutableTestCoreState
}

func funcTestCallPanicViewEPFromFullThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestCallPanicViewEPFromFull")
	f := &TestCallPanicViewEPFromFullContext{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcTestCallPanicViewEPFromFull(ctx, f)
	ctx.Log("testcore.funcTestCallPanicViewEPFromFull ok")
}

type TestChainOwnerIDFullContext struct {
	Results MutableTestChainOwnerIDFullResults
	State   MutableTestCoreState
}

func funcTestChainOwnerIDFullThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestChainOwnerIDFull")
	results := wasmlib.NewScDict()
	f := &TestChainOwnerIDFullContext{
		Results: MutableTestChainOwnerIDFullResults{
			proxy: results.AsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcTestChainOwnerIDFull(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.funcTestChainOwnerIDFull ok")
}

type TestEventLogDeployContext struct {
	State MutableTestCoreState
}

func funcTestEventLogDeployThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestEventLogDeploy")
	f := &TestEventLogDeployContext{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcTestEventLogDeploy(ctx, f)
	ctx.Log("testcore.funcTestEventLogDeploy ok")
}

type TestEventLogEventDataContext struct {
	State MutableTestCoreState
}

func funcTestEventLogEventDataThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestEventLogEventData")
	f := &TestEventLogEventDataContext{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcTestEventLogEventData(ctx, f)
	ctx.Log("testcore.funcTestEventLogEventData ok")
}

type TestEventLogGenericDataContext struct {
	Params ImmutableTestEventLogGenericDataParams
	State  MutableTestCoreState
}

func funcTestEventLogGenericDataThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestEventLogGenericData")
	f := &TestEventLogGenericDataContext{
		Params: ImmutableTestEventLogGenericDataParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Counter().Exists(), "missing mandatory counter")
	funcTestEventLogGenericData(ctx, f)
	ctx.Log("testcore.funcTestEventLogGenericData ok")
}

type TestPanicFullEPContext struct {
	State MutableTestCoreState
}

func funcTestPanicFullEPThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcTestPanicFullEP")
	f := &TestPanicFullEPContext{
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcTestPanicFullEP(ctx, f)
	ctx.Log("testcore.funcTestPanicFullEP ok")
}

type WithdrawToChainContext struct {
	Params ImmutableWithdrawToChainParams
	State  MutableTestCoreState
}

func funcWithdrawToChainThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("testcore.funcWithdrawToChain")
	f := &WithdrawToChainContext{
		Params: ImmutableWithdrawToChainParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.ChainID().Exists(), "missing mandatory chainID")
	funcWithdrawToChain(ctx, f)
	ctx.Log("testcore.funcWithdrawToChain ok")
}

type CheckContextFromViewEPContext struct {
	Params ImmutableCheckContextFromViewEPParams
	State  ImmutableTestCoreState
}

func viewCheckContextFromViewEPThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewCheckContextFromViewEP")
	f := &CheckContextFromViewEPContext{
		Params: ImmutableCheckContextFromViewEPParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.AgentID().Exists(), "missing mandatory agentID")
	ctx.Require(f.Params.ChainID().Exists(), "missing mandatory chainID")
	ctx.Require(f.Params.ChainOwnerID().Exists(), "missing mandatory chainOwnerID")
	ctx.Require(f.Params.ContractCreator().Exists(), "missing mandatory contractCreator")
	viewCheckContextFromViewEP(ctx, f)
	ctx.Log("testcore.viewCheckContextFromViewEP ok")
}

type FibonacciContext struct {
	Params  ImmutableFibonacciParams
	Results MutableFibonacciResults
	State   ImmutableTestCoreState
}

func viewFibonacciThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewFibonacci")
	results := wasmlib.NewScDict()
	f := &FibonacciContext{
		Params: ImmutableFibonacciParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableFibonacciResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.IntValue().Exists(), "missing mandatory intValue")
	viewFibonacci(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.viewFibonacci ok")
}

type GetCounterContext struct {
	Results MutableGetCounterResults
	State   ImmutableTestCoreState
}

func viewGetCounterThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewGetCounter")
	results := wasmlib.NewScDict()
	f := &GetCounterContext{
		Results: MutableGetCounterResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewGetCounter(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.viewGetCounter ok")
}

type GetIntContext struct {
	Params  ImmutableGetIntParams
	Results MutableGetIntResults
	State   ImmutableTestCoreState
}

func viewGetIntThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewGetInt")
	results := wasmlib.NewScDict()
	f := &GetIntContext{
		Params: ImmutableGetIntParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableGetIntResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Name().Exists(), "missing mandatory name")
	viewGetInt(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.viewGetInt ok")
}

type GetStringValueContext struct {
	Params  ImmutableGetStringValueParams
	Results MutableGetStringValueResults
	State   ImmutableTestCoreState
}

func viewGetStringValueThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewGetStringValue")
	results := wasmlib.NewScDict()
	f := &GetStringValueContext{
		Params: ImmutableGetStringValueParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableGetStringValueResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.VarName().Exists(), "missing mandatory varName")
	viewGetStringValue(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.viewGetStringValue ok")
}

type JustViewContext struct {
	State ImmutableTestCoreState
}

func viewJustViewThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewJustView")
	f := &JustViewContext{
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewJustView(ctx, f)
	ctx.Log("testcore.viewJustView ok")
}

type PassTypesViewContext struct {
	Params ImmutablePassTypesViewParams
	State  ImmutableTestCoreState
}

func viewPassTypesViewThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewPassTypesView")
	f := &PassTypesViewContext{
		Params: ImmutablePassTypesViewParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Address().Exists(), "missing mandatory address")
	ctx.Require(f.Params.AgentID().Exists(), "missing mandatory agentID")
	ctx.Require(f.Params.ChainID().Exists(), "missing mandatory chainID")
	ctx.Require(f.Params.ContractID().Exists(), "missing mandatory contractID")
	ctx.Require(f.Params.Hash().Exists(), "missing mandatory hash")
	ctx.Require(f.Params.Hname().Exists(), "missing mandatory hname")
	ctx.Require(f.Params.HnameZero().Exists(), "missing mandatory hnameZero")
	ctx.Require(f.Params.Int64().Exists(), "missing mandatory int64")
	ctx.Require(f.Params.Int64Zero().Exists(), "missing mandatory int64Zero")
	ctx.Require(f.Params.String().Exists(), "missing mandatory string")
	ctx.Require(f.Params.StringZero().Exists(), "missing mandatory stringZero")
	viewPassTypesView(ctx, f)
	ctx.Log("testcore.viewPassTypesView ok")
}

type TestCallPanicViewEPFromViewContext struct {
	State ImmutableTestCoreState
}

func viewTestCallPanicViewEPFromViewThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewTestCallPanicViewEPFromView")
	f := &TestCallPanicViewEPFromViewContext{
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewTestCallPanicViewEPFromView(ctx, f)
	ctx.Log("testcore.viewTestCallPanicViewEPFromView ok")
}

type TestChainOwnerIDViewContext struct {
	Results MutableTestChainOwnerIDViewResults
	State   ImmutableTestCoreState
}

func viewTestChainOwnerIDViewThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewTestChainOwnerIDView")
	results := wasmlib.NewScDict()
	f := &TestChainOwnerIDViewContext{
		Results: MutableTestChainOwnerIDViewResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewTestChainOwnerIDView(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.viewTestChainOwnerIDView ok")
}

type TestPanicViewEPContext struct {
	State ImmutableTestCoreState
}

func viewTestPanicViewEPThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewTestPanicViewEP")
	f := &TestPanicViewEPContext{
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewTestPanicViewEP(ctx, f)
	ctx.Log("testcore.viewTestPanicViewEP ok")
}

type TestSandboxCallContext struct {
	Results MutableTestSandboxCallResults
	State   ImmutableTestCoreState
}

func viewTestSandboxCallThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("testcore.viewTestSandboxCall")
	results := wasmlib.NewScDict()
	f := &TestSandboxCallContext{
		Results: MutableTestSandboxCallResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableTestCoreState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewTestSandboxCall(ctx, f)
	ctx.Results(results)
	ctx.Log("testcore.viewTestSandboxCall ok")
}
