// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package coreblob

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type StoreBlobCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableStoreBlobParams
	Results ImmutableStoreBlobResults
}

type GetBlobFieldCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetBlobFieldParams
	Results ImmutableGetBlobFieldResults
}

type GetBlobInfoCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetBlobInfoParams
	Results ImmutableGetBlobInfoResults
}

type ListBlobsCall struct {
	Func    *wasmlib.ScView
	Results ImmutableListBlobsResults
}

type Funcs struct{}

var ScFuncs Funcs

func (sc Funcs) StoreBlob(ctx wasmlib.ScFuncCallContext) *StoreBlobCall {
	f := &StoreBlobCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncStoreBlob)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) GetBlobField(ctx wasmlib.ScViewCallContext) *GetBlobFieldCall {
	f := &GetBlobFieldCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetBlobField)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) GetBlobInfo(ctx wasmlib.ScViewCallContext) *GetBlobInfoCall {
	f := &GetBlobInfoCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetBlobInfo)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) ListBlobs(ctx wasmlib.ScViewCallContext) *ListBlobsCall {
	f := &ListBlobsCall{Func: wasmlib.NewScView(ctx, HScName, HViewListBlobs)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func OnLoad() {
	exports := wasmlib.NewScExports()
	exports.AddFunc(FuncStoreBlob, wasmlib.FuncError)
	exports.AddView(ViewGetBlobField, wasmlib.ViewError)
	exports.AddView(ViewGetBlobInfo, wasmlib.ViewError)
	exports.AddView(ViewListBlobs, wasmlib.ViewError)
}
