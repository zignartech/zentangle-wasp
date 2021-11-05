// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package fairroulette

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type Bet struct {
	Amount int64
	Better wasmlib.ScAgentID
	Number int64
}

func NewBetFromBytes(bytes []byte) *Bet {
	decode := wasmlib.NewBytesDecoder(bytes)
	data := &Bet{}
	data.Amount = decode.Int64()
	data.Better = decode.AgentID()
	data.Number = decode.Int64()
	decode.Close()
	return data
}

func (o *Bet) Bytes() []byte {
	return wasmlib.NewBytesEncoder().
		Int64(o.Amount).
		AgentID(o.Better).
		Int64(o.Number).
		Data()
}

type ImmutableBet struct {
	objID int32
	keyID wasmlib.Key32
}

func (o ImmutableBet) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o ImmutableBet) Value() *Bet {
	return NewBetFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}

type MutableBet struct {
	objID int32
	keyID wasmlib.Key32
}

func (o MutableBet) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o MutableBet) SetValue(value *Bet) {
	wasmlib.SetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES, value.Bytes())
}

func (o MutableBet) Value() *Bet {
	return NewBetFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}
