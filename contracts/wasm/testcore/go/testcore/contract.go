// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package testcore

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type CallOnChainCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableCallOnChainParams
	Results ImmutableCallOnChainResults
}

type CheckContextFromFullEPCall struct {
	Func   *wasmlib.ScFunc
	Params MutableCheckContextFromFullEPParams
}

type DoNothingCall struct {
	Func *wasmlib.ScFunc
}

type GetMintedSupplyCall struct {
	Func    *wasmlib.ScFunc
	Results ImmutableGetMintedSupplyResults
}

type IncCounterCall struct {
	Func *wasmlib.ScFunc
}

type InitCall struct {
	Func   *wasmlib.ScInitFunc
	Params MutableInitParams
}

type PassTypesFullCall struct {
	Func   *wasmlib.ScFunc
	Params MutablePassTypesFullParams
}

type RunRecursionCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableRunRecursionParams
	Results ImmutableRunRecursionResults
}

type SendToAddressCall struct {
	Func   *wasmlib.ScFunc
	Params MutableSendToAddressParams
}

type SetIntCall struct {
	Func   *wasmlib.ScFunc
	Params MutableSetIntParams
}

type SpawnCall struct {
	Func   *wasmlib.ScFunc
	Params MutableSpawnParams
}

type TestBlockContext1Call struct {
	Func *wasmlib.ScFunc
}

type TestBlockContext2Call struct {
	Func *wasmlib.ScFunc
}

type TestCallPanicFullEPCall struct {
	Func *wasmlib.ScFunc
}

type TestCallPanicViewEPFromFullCall struct {
	Func *wasmlib.ScFunc
}

type TestChainOwnerIDFullCall struct {
	Func    *wasmlib.ScFunc
	Results ImmutableTestChainOwnerIDFullResults
}

type TestEventLogDeployCall struct {
	Func *wasmlib.ScFunc
}

type TestEventLogEventDataCall struct {
	Func *wasmlib.ScFunc
}

type TestEventLogGenericDataCall struct {
	Func   *wasmlib.ScFunc
	Params MutableTestEventLogGenericDataParams
}

type TestPanicFullEPCall struct {
	Func *wasmlib.ScFunc
}

type WithdrawToChainCall struct {
	Func   *wasmlib.ScFunc
	Params MutableWithdrawToChainParams
}

type CheckContextFromViewEPCall struct {
	Func   *wasmlib.ScView
	Params MutableCheckContextFromViewEPParams
}

type FibonacciCall struct {
	Func    *wasmlib.ScView
	Params  MutableFibonacciParams
	Results ImmutableFibonacciResults
}

type GetCounterCall struct {
	Func    *wasmlib.ScView
	Results ImmutableGetCounterResults
}

type GetIntCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetIntParams
	Results ImmutableGetIntResults
}

type GetStringValueCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetStringValueParams
	Results ImmutableGetStringValueResults
}

type JustViewCall struct {
	Func *wasmlib.ScView
}

type PassTypesViewCall struct {
	Func   *wasmlib.ScView
	Params MutablePassTypesViewParams
}

type TestCallPanicViewEPFromViewCall struct {
	Func *wasmlib.ScView
}

type TestChainOwnerIDViewCall struct {
	Func    *wasmlib.ScView
	Results ImmutableTestChainOwnerIDViewResults
}

type TestPanicViewEPCall struct {
	Func *wasmlib.ScView
}

type TestSandboxCallCall struct {
	Func    *wasmlib.ScView
	Results ImmutableTestSandboxCallResults
}

type Funcs struct{}

var ScFuncs Funcs

func (sc Funcs) CallOnChain(ctx wasmlib.ScFuncCallContext) *CallOnChainCall {
	f := &CallOnChainCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncCallOnChain)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) CheckContextFromFullEP(ctx wasmlib.ScFuncCallContext) *CheckContextFromFullEPCall {
	f := &CheckContextFromFullEPCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncCheckContextFromFullEP)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) DoNothing(ctx wasmlib.ScFuncCallContext) *DoNothingCall {
	return &DoNothingCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncDoNothing)}
}

func (sc Funcs) GetMintedSupply(ctx wasmlib.ScFuncCallContext) *GetMintedSupplyCall {
	f := &GetMintedSupplyCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncGetMintedSupply)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func (sc Funcs) IncCounter(ctx wasmlib.ScFuncCallContext) *IncCounterCall {
	return &IncCounterCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncIncCounter)}
}

func (sc Funcs) Init(ctx wasmlib.ScFuncCallContext) *InitCall {
	f := &InitCall{Func: wasmlib.NewScInitFunc(ctx, HScName, HFuncInit, keyMap[:], idxMap[:])}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) PassTypesFull(ctx wasmlib.ScFuncCallContext) *PassTypesFullCall {
	f := &PassTypesFullCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncPassTypesFull)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) RunRecursion(ctx wasmlib.ScFuncCallContext) *RunRecursionCall {
	f := &RunRecursionCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncRunRecursion)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) SendToAddress(ctx wasmlib.ScFuncCallContext) *SendToAddressCall {
	f := &SendToAddressCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncSendToAddress)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) SetInt(ctx wasmlib.ScFuncCallContext) *SetIntCall {
	f := &SetIntCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncSetInt)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) Spawn(ctx wasmlib.ScFuncCallContext) *SpawnCall {
	f := &SpawnCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncSpawn)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) TestBlockContext1(ctx wasmlib.ScFuncCallContext) *TestBlockContext1Call {
	return &TestBlockContext1Call{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestBlockContext1)}
}

func (sc Funcs) TestBlockContext2(ctx wasmlib.ScFuncCallContext) *TestBlockContext2Call {
	return &TestBlockContext2Call{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestBlockContext2)}
}

func (sc Funcs) TestCallPanicFullEP(ctx wasmlib.ScFuncCallContext) *TestCallPanicFullEPCall {
	return &TestCallPanicFullEPCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestCallPanicFullEP)}
}

func (sc Funcs) TestCallPanicViewEPFromFull(ctx wasmlib.ScFuncCallContext) *TestCallPanicViewEPFromFullCall {
	return &TestCallPanicViewEPFromFullCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestCallPanicViewEPFromFull)}
}

func (sc Funcs) TestChainOwnerIDFull(ctx wasmlib.ScFuncCallContext) *TestChainOwnerIDFullCall {
	f := &TestChainOwnerIDFullCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestChainOwnerIDFull)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func (sc Funcs) TestEventLogDeploy(ctx wasmlib.ScFuncCallContext) *TestEventLogDeployCall {
	return &TestEventLogDeployCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestEventLogDeploy)}
}

func (sc Funcs) TestEventLogEventData(ctx wasmlib.ScFuncCallContext) *TestEventLogEventDataCall {
	return &TestEventLogEventDataCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestEventLogEventData)}
}

func (sc Funcs) TestEventLogGenericData(ctx wasmlib.ScFuncCallContext) *TestEventLogGenericDataCall {
	f := &TestEventLogGenericDataCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestEventLogGenericData)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) TestPanicFullEP(ctx wasmlib.ScFuncCallContext) *TestPanicFullEPCall {
	return &TestPanicFullEPCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTestPanicFullEP)}
}

func (sc Funcs) WithdrawToChain(ctx wasmlib.ScFuncCallContext) *WithdrawToChainCall {
	f := &WithdrawToChainCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncWithdrawToChain)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) CheckContextFromViewEP(ctx wasmlib.ScViewCallContext) *CheckContextFromViewEPCall {
	f := &CheckContextFromViewEPCall{Func: wasmlib.NewScView(ctx, HScName, HViewCheckContextFromViewEP)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) Fibonacci(ctx wasmlib.ScViewCallContext) *FibonacciCall {
	f := &FibonacciCall{Func: wasmlib.NewScView(ctx, HScName, HViewFibonacci)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) GetCounter(ctx wasmlib.ScViewCallContext) *GetCounterCall {
	f := &GetCounterCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetCounter)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func (sc Funcs) GetInt(ctx wasmlib.ScViewCallContext) *GetIntCall {
	f := &GetIntCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetInt)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) GetStringValue(ctx wasmlib.ScViewCallContext) *GetStringValueCall {
	f := &GetStringValueCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetStringValue)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) JustView(ctx wasmlib.ScViewCallContext) *JustViewCall {
	return &JustViewCall{Func: wasmlib.NewScView(ctx, HScName, HViewJustView)}
}

func (sc Funcs) PassTypesView(ctx wasmlib.ScViewCallContext) *PassTypesViewCall {
	f := &PassTypesViewCall{Func: wasmlib.NewScView(ctx, HScName, HViewPassTypesView)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) TestCallPanicViewEPFromView(ctx wasmlib.ScViewCallContext) *TestCallPanicViewEPFromViewCall {
	return &TestCallPanicViewEPFromViewCall{Func: wasmlib.NewScView(ctx, HScName, HViewTestCallPanicViewEPFromView)}
}

func (sc Funcs) TestChainOwnerIDView(ctx wasmlib.ScViewCallContext) *TestChainOwnerIDViewCall {
	f := &TestChainOwnerIDViewCall{Func: wasmlib.NewScView(ctx, HScName, HViewTestChainOwnerIDView)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func (sc Funcs) TestPanicViewEP(ctx wasmlib.ScViewCallContext) *TestPanicViewEPCall {
	return &TestPanicViewEPCall{Func: wasmlib.NewScView(ctx, HScName, HViewTestPanicViewEP)}
}

func (sc Funcs) TestSandboxCall(ctx wasmlib.ScViewCallContext) *TestSandboxCallCall {
	f := &TestSandboxCallCall{Func: wasmlib.NewScView(ctx, HScName, HViewTestSandboxCall)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}
