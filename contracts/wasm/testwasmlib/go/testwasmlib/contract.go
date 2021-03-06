// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package testwasmlib

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

type ArrayOfArraysAppendCall struct {
	Func   *wasmlib.ScFunc
	Params MutableArrayOfArraysAppendParams
}

type ArrayOfArraysClearCall struct {
	Func *wasmlib.ScFunc
}

type ArrayOfArraysSetCall struct {
	Func   *wasmlib.ScFunc
	Params MutableArrayOfArraysSetParams
}

type ArrayOfMapsClearCall struct {
	Func *wasmlib.ScFunc
}

type ArrayOfMapsSetCall struct {
	Func   *wasmlib.ScFunc
	Params MutableArrayOfMapsSetParams
}

type MapOfArraysAppendCall struct {
	Func   *wasmlib.ScFunc
	Params MutableMapOfArraysAppendParams
}

type MapOfArraysClearCall struct {
	Func   *wasmlib.ScFunc
	Params MutableMapOfArraysClearParams
}

type MapOfArraysSetCall struct {
	Func   *wasmlib.ScFunc
	Params MutableMapOfArraysSetParams
}

type MapOfMapsClearCall struct {
	Func   *wasmlib.ScFunc
	Params MutableMapOfMapsClearParams
}

type MapOfMapsSetCall struct {
	Func   *wasmlib.ScFunc
	Params MutableMapOfMapsSetParams
}

type ParamTypesCall struct {
	Func   *wasmlib.ScFunc
	Params MutableParamTypesParams
}

type RandomCall struct {
	Func *wasmlib.ScFunc
}

type TriggerEventCall struct {
	Func   *wasmlib.ScFunc
	Params MutableTriggerEventParams
}

type ArrayOfArraysLengthCall struct {
	Func    *wasmlib.ScView
	Results ImmutableArrayOfArraysLengthResults
}

type ArrayOfArraysValueCall struct {
	Func    *wasmlib.ScView
	Params  MutableArrayOfArraysValueParams
	Results ImmutableArrayOfArraysValueResults
}

type ArrayOfMapsValueCall struct {
	Func    *wasmlib.ScView
	Params  MutableArrayOfMapsValueParams
	Results ImmutableArrayOfMapsValueResults
}

type BlockRecordCall struct {
	Func    *wasmlib.ScView
	Params  MutableBlockRecordParams
	Results ImmutableBlockRecordResults
}

type BlockRecordsCall struct {
	Func    *wasmlib.ScView
	Params  MutableBlockRecordsParams
	Results ImmutableBlockRecordsResults
}

type GetRandomCall struct {
	Func    *wasmlib.ScView
	Results ImmutableGetRandomResults
}

type IotaBalanceCall struct {
	Func    *wasmlib.ScView
	Results ImmutableIotaBalanceResults
}

type MapOfArraysLengthCall struct {
	Func    *wasmlib.ScView
	Params  MutableMapOfArraysLengthParams
	Results ImmutableMapOfArraysLengthResults
}

type MapOfArraysValueCall struct {
	Func    *wasmlib.ScView
	Params  MutableMapOfArraysValueParams
	Results ImmutableMapOfArraysValueResults
}

type MapOfMapsValueCall struct {
	Func    *wasmlib.ScView
	Params  MutableMapOfMapsValueParams
	Results ImmutableMapOfMapsValueResults
}

type Funcs struct{}

var ScFuncs Funcs

func (sc Funcs) ArrayOfArraysAppend(ctx wasmlib.ScFuncCallContext) *ArrayOfArraysAppendCall {
	f := &ArrayOfArraysAppendCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncArrayOfArraysAppend)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) ArrayOfArraysClear(ctx wasmlib.ScFuncCallContext) *ArrayOfArraysClearCall {
	return &ArrayOfArraysClearCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncArrayOfArraysClear)}
}

func (sc Funcs) ArrayOfArraysSet(ctx wasmlib.ScFuncCallContext) *ArrayOfArraysSetCall {
	f := &ArrayOfArraysSetCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncArrayOfArraysSet)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) ArrayOfMapsClear(ctx wasmlib.ScFuncCallContext) *ArrayOfMapsClearCall {
	return &ArrayOfMapsClearCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncArrayOfMapsClear)}
}

func (sc Funcs) ArrayOfMapsSet(ctx wasmlib.ScFuncCallContext) *ArrayOfMapsSetCall {
	f := &ArrayOfMapsSetCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncArrayOfMapsSet)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) MapOfArraysAppend(ctx wasmlib.ScFuncCallContext) *MapOfArraysAppendCall {
	f := &MapOfArraysAppendCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncMapOfArraysAppend)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) MapOfArraysClear(ctx wasmlib.ScFuncCallContext) *MapOfArraysClearCall {
	f := &MapOfArraysClearCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncMapOfArraysClear)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) MapOfArraysSet(ctx wasmlib.ScFuncCallContext) *MapOfArraysSetCall {
	f := &MapOfArraysSetCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncMapOfArraysSet)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) MapOfMapsClear(ctx wasmlib.ScFuncCallContext) *MapOfMapsClearCall {
	f := &MapOfMapsClearCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncMapOfMapsClear)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) MapOfMapsSet(ctx wasmlib.ScFuncCallContext) *MapOfMapsSetCall {
	f := &MapOfMapsSetCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncMapOfMapsSet)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) ParamTypes(ctx wasmlib.ScFuncCallContext) *ParamTypesCall {
	f := &ParamTypesCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncParamTypes)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) Random(ctx wasmlib.ScFuncCallContext) *RandomCall {
	return &RandomCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncRandom)}
}

func (sc Funcs) TriggerEvent(ctx wasmlib.ScFuncCallContext) *TriggerEventCall {
	f := &TriggerEventCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncTriggerEvent)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	return f
}

func (sc Funcs) ArrayOfArraysLength(ctx wasmlib.ScViewCallContext) *ArrayOfArraysLengthCall {
	f := &ArrayOfArraysLengthCall{Func: wasmlib.NewScView(ctx, HScName, HViewArrayOfArraysLength)}
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) ArrayOfArraysValue(ctx wasmlib.ScViewCallContext) *ArrayOfArraysValueCall {
	f := &ArrayOfArraysValueCall{Func: wasmlib.NewScView(ctx, HScName, HViewArrayOfArraysValue)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) ArrayOfMapsValue(ctx wasmlib.ScViewCallContext) *ArrayOfMapsValueCall {
	f := &ArrayOfMapsValueCall{Func: wasmlib.NewScView(ctx, HScName, HViewArrayOfMapsValue)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) BlockRecord(ctx wasmlib.ScViewCallContext) *BlockRecordCall {
	f := &BlockRecordCall{Func: wasmlib.NewScView(ctx, HScName, HViewBlockRecord)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) BlockRecords(ctx wasmlib.ScViewCallContext) *BlockRecordsCall {
	f := &BlockRecordsCall{Func: wasmlib.NewScView(ctx, HScName, HViewBlockRecords)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) GetRandom(ctx wasmlib.ScViewCallContext) *GetRandomCall {
	f := &GetRandomCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetRandom)}
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) IotaBalance(ctx wasmlib.ScViewCallContext) *IotaBalanceCall {
	f := &IotaBalanceCall{Func: wasmlib.NewScView(ctx, HScName, HViewIotaBalance)}
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) MapOfArraysLength(ctx wasmlib.ScViewCallContext) *MapOfArraysLengthCall {
	f := &MapOfArraysLengthCall{Func: wasmlib.NewScView(ctx, HScName, HViewMapOfArraysLength)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) MapOfArraysValue(ctx wasmlib.ScViewCallContext) *MapOfArraysValueCall {
	f := &MapOfArraysValueCall{Func: wasmlib.NewScView(ctx, HScName, HViewMapOfArraysValue)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}

func (sc Funcs) MapOfMapsValue(ctx wasmlib.ScViewCallContext) *MapOfMapsValueCall {
	f := &MapOfMapsValueCall{Func: wasmlib.NewScView(ctx, HScName, HViewMapOfMapsValue)}
	f.Params.proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.proxy)
	return f
}
