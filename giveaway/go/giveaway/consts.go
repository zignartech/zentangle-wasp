// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package giveaway

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"

const (
	ScName        = "giveaway"
	ScDescription = "giveaway smart contract to choose the winners of the Zentangle Drangon's giveaways. Addresses should not be repeated"
	HScName       = wasmtypes.ScHname(0x31774d34)
)

const (
	ParamAddresses = "addresses"
	ParamNWinners  = "nWinners"
	ParamOwner     = "owner"
)

const (
	ResultOwner   = "owner"
	ResultWinners = "winners"
)

const (
	StateAddresses = "addresses"
	StateOwner     = "owner"
)

const (
	FuncInit            = "init"
	FuncLoadAddresses   = "loadAddresses"
	FuncRuffle          = "ruffle"
	FuncSetOwner        = "setOwner"
	FuncUnloadAddresses = "unloadAddresses"
	ViewGetOwner        = "getOwner"
)

const (
	HFuncInit            = wasmtypes.ScHname(0x1f44d644)
	HFuncLoadAddresses   = wasmtypes.ScHname(0x2e3febf1)
	HFuncRuffle          = wasmtypes.ScHname(0x1a23d876)
	HFuncSetOwner        = wasmtypes.ScHname(0x2a15fe7b)
	HFuncUnloadAddresses = wasmtypes.ScHname(0xb8992203)
	HViewGetOwner        = wasmtypes.ScHname(0x137107a6)
)
