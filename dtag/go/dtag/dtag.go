// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package dtag

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

func funcCreateGame(ctx wasmlib.ScFuncContext, f *CreateGameContext) {
}

func funcEndGame(ctx wasmlib.ScFuncContext, f *EndGameContext) {
}

func funcRequestPlay(ctx wasmlib.ScFuncContext, f *RequestPlayContext) {
}

func funcSendTags(ctx wasmlib.ScFuncContext, f *SendTagsContext) {
}

func viewGetPlaysPerImage(ctx wasmlib.ScViewContext, f *GetPlaysPerImageContext) {
}

func viewGetResults(ctx wasmlib.ScViewContext, f *GetResultsContext) {
}
