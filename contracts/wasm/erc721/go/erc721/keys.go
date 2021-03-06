// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package erc721

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

const (
	IdxParamApproval = 0
	IdxParamApproved = 1
	IdxParamData     = 2
	IdxParamFrom     = 3
	IdxParamName     = 4
	IdxParamOperator = 5
	IdxParamOwner    = 6
	IdxParamSymbol   = 7
	IdxParamTo       = 8
	IdxParamTokenID  = 9

	IdxResultAmount   = 10
	IdxResultApproval = 11
	IdxResultApproved = 12
	IdxResultName     = 13
	IdxResultOwner    = 14
	IdxResultSymbol   = 15
	IdxResultTokenURI = 16

	IdxStateApprovedAccounts  = 17
	IdxStateApprovedOperators = 18
	IdxStateBalances          = 19
	IdxStateName              = 20
	IdxStateOwners            = 21
	IdxStateSymbol            = 22
)

const keyMapLen = 23

var keyMap = [keyMapLen]wasmlib.Key{
	ParamApproval,
	ParamApproved,
	ParamData,
	ParamFrom,
	ParamName,
	ParamOperator,
	ParamOwner,
	ParamSymbol,
	ParamTo,
	ParamTokenID,
	ResultAmount,
	ResultApproval,
	ResultApproved,
	ResultName,
	ResultOwner,
	ResultSymbol,
	ResultTokenURI,
	StateApprovedAccounts,
	StateApprovedOperators,
	StateBalances,
	StateName,
	StateOwners,
	StateSymbol,
}

var idxMap [keyMapLen]wasmlib.Key32
