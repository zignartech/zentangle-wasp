// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package dtag

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type CreateGameCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableCreateGameParams
}

type EndGameCall struct {
	Func    *wasmlib.ScFunc
}

type RequestPlayCall struct {
	Func    *wasmlib.ScFunc
	Results ImmutableRequestPlayResults
}

type SendTagsCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableSendTagsParams
}

type GetPlaysPerImageCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetPlaysPerImageParams
	Results ImmutableGetPlaysPerImageResults
}

type GetResultsCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetResultsParams
	Results ImmutableGetResultsResults
}

type Funcs struct{}

var ScFuncs Funcs

func (sc Funcs) CreateGame(ctx wasmlib.ScFuncCallContext) *CreateGameCall {
	f := &CreateGameCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncCreateGame)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) EndGame(ctx wasmlib.ScFuncCallContext) *EndGameCall {
	return &EndGameCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncEndGame)}
}

func (sc Funcs) RequestPlay(ctx wasmlib.ScFuncCallContext) *RequestPlayCall {
	f := &RequestPlayCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncRequestPlay)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func (sc Funcs) SendTags(ctx wasmlib.ScFuncCallContext) *SendTagsCall {
	f := &SendTagsCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncSendTags)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) GetPlaysPerImage(ctx wasmlib.ScViewCallContext) *GetPlaysPerImageCall {
	f := &GetPlaysPerImageCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetPlaysPerImage)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) GetResults(ctx wasmlib.ScViewCallContext) *GetResultsCall {
	f := &GetResultsCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetResults)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}
