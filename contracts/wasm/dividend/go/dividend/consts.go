// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package dividend

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

const (
	ScName        = "dividend"
	ScDescription = "Simple dividend smart contract"
	HScName       = wasmlib.ScHname(0xcce2e239)
)

const (
	ParamAddress = wasmlib.Key("address")
	ParamFactor  = wasmlib.Key("factor")
	ParamOwner   = wasmlib.Key("owner")
)

const (
	ResultFactor = wasmlib.Key("factor")
	ResultOwner  = wasmlib.Key("owner")
)

const (
	StateMemberList  = wasmlib.Key("memberList")
	StateMembers     = wasmlib.Key("members")
	StateOwner       = wasmlib.Key("owner")
	StateTotalFactor = wasmlib.Key("totalFactor")
)

const (
	FuncDivide    = "divide"
	FuncInit      = "init"
	FuncMember    = "member"
	FuncSetOwner  = "setOwner"
	ViewGetFactor = "getFactor"
	ViewGetOwner  = "getOwner"
)

const (
	HFuncDivide    = wasmlib.ScHname(0xc7878107)
	HFuncInit      = wasmlib.ScHname(0x1f44d644)
	HFuncMember    = wasmlib.ScHname(0xc07da2cb)
	HFuncSetOwner  = wasmlib.ScHname(0x2a15fe7b)
	HViewGetFactor = wasmlib.ScHname(0x0ee668fe)
	HViewGetOwner  = wasmlib.ScHname(0x137107a6)
)
