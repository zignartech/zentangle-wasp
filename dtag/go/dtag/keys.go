// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package dtag

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

const (
	IdxParamDescription          = 0
	IdxParamH                    = 1
	IdxParamImageId              = 2
	IdxParamNumberOfImages       = 3
	IdxParamTagsRequiredPerImage = 4
	IdxParamW                    = 5
	IdxParamX                    = 6
	IdxParamY                    = 7

	IdxResultImageId       = 8
	IdxResultPlaysPerImage = 9
	IdxResultResults       = 10

	IdxStateBets                 = 11
	IdxStateCreator              = 12
	IdxStateDescription          = 13
	IdxStateNumberOfImages       = 14
	IdxStatePendingPlays         = 15
	IdxStatePlaysPerImage        = 16
	IdxStateProcessedImages      = 17
	IdxStateReward               = 18
	IdxStateTaggedImages         = 19
	IdxStateTagsRequiredPerImage = 20
)

const keyMapLen = 21

var keyMap = [keyMapLen]wasmlib.Key{
	ParamDescription,
	ParamH,
	ParamImageId,
	ParamNumberOfImages,
	ParamTagsRequiredPerImage,
	ParamW,
	ParamX,
	ParamY,
	ResultImageId,
	ResultPlaysPerImage,
	ResultResults,
	StateBets,
	StateCreator,
	StateDescription,
	StateNumberOfImages,
	StatePendingPlays,
	StatePlaysPerImage,
	StateProcessedImages,
	StateReward,
	StateTaggedImages,
	StateTagsRequiredPerImage,
}

var idxMap [keyMapLen]wasmlib.Key32
