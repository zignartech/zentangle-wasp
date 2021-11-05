// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package dividend

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type ImmutableInitParams struct {
	id int32
}

func (s ImmutableInitParams) Owner() wasmlib.ScImmutableAgentID {
	return wasmlib.NewScImmutableAgentID(s.id, idxMap[IdxParamOwner])
}

type MutableInitParams struct {
	id int32
}

func (s MutableInitParams) Owner() wasmlib.ScMutableAgentID {
	return wasmlib.NewScMutableAgentID(s.id, idxMap[IdxParamOwner])
}

type ImmutableMemberParams struct {
	id int32
}

func (s ImmutableMemberParams) Address() wasmlib.ScImmutableAddress {
	return wasmlib.NewScImmutableAddress(s.id, idxMap[IdxParamAddress])
}

func (s ImmutableMemberParams) Factor() wasmlib.ScImmutableInt64 {
	return wasmlib.NewScImmutableInt64(s.id, idxMap[IdxParamFactor])
}

type MutableMemberParams struct {
	id int32
}

func (s MutableMemberParams) Address() wasmlib.ScMutableAddress {
	return wasmlib.NewScMutableAddress(s.id, idxMap[IdxParamAddress])
}

func (s MutableMemberParams) Factor() wasmlib.ScMutableInt64 {
	return wasmlib.NewScMutableInt64(s.id, idxMap[IdxParamFactor])
}

type ImmutableSetOwnerParams struct {
	id int32
}

func (s ImmutableSetOwnerParams) Owner() wasmlib.ScImmutableAgentID {
	return wasmlib.NewScImmutableAgentID(s.id, idxMap[IdxParamOwner])
}

type MutableSetOwnerParams struct {
	id int32
}

func (s MutableSetOwnerParams) Owner() wasmlib.ScMutableAgentID {
	return wasmlib.NewScMutableAgentID(s.id, idxMap[IdxParamOwner])
}

type ImmutableGetFactorParams struct {
	id int32
}

func (s ImmutableGetFactorParams) Address() wasmlib.ScImmutableAddress {
	return wasmlib.NewScImmutableAddress(s.id, idxMap[IdxParamAddress])
}

type MutableGetFactorParams struct {
	id int32
}

func (s MutableGetFactorParams) Address() wasmlib.ScMutableAddress {
	return wasmlib.NewScMutableAddress(s.id, idxMap[IdxParamAddress])
}
