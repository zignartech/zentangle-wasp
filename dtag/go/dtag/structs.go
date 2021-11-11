// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package dtag

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type Bet struct {
	Amount  int64 
	ImageId int32 
	Player  wasmlib.ScAgentID  // player placing the bet
}

func NewBetFromBytes(bytes []byte) *Bet {
	decode := wasmlib.NewBytesDecoder(bytes)
	data := &Bet{}
	data.Amount  = decode.Int64()
	data.ImageId = decode.Int32()
	data.Player  = decode.AgentID()
	decode.Close()
	return data
}

func (o *Bet) Bytes() []byte {
	return wasmlib.NewBytesEncoder().
		Int64(o.Amount).
		Int32(o.ImageId).
		AgentID(o.Player).
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

type TaggedImage struct {
	H       int64  // height of the Tag
	ImageId int32 
	Player  wasmlib.ScAgentID  // player that has tagged this image
	W       int64  // width of the Tag
	X       int64  // x top-left position of the Tag TODO: This should be a nested constructor in the future
	Y       int64  // y top-left position of the Tag
}

func NewTaggedImageFromBytes(bytes []byte) *TaggedImage {
	decode := wasmlib.NewBytesDecoder(bytes)
	data := &TaggedImage{}
	data.H       = decode.Int64()
	data.ImageId = decode.Int32()
	data.Player  = decode.AgentID()
	data.W       = decode.Int64()
	data.X       = decode.Int64()
	data.Y       = decode.Int64()
	decode.Close()
	return data
}

func (o *TaggedImage) Bytes() []byte {
	return wasmlib.NewBytesEncoder().
		Int64(o.H).
		Int32(o.ImageId).
		AgentID(o.Player).
		Int64(o.W).
		Int64(o.X).
		Int64(o.Y).
		Data()
}

type ImmutableTaggedImage struct {
	objID int32
	keyID wasmlib.Key32
}

func (o ImmutableTaggedImage) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o ImmutableTaggedImage) Value() *TaggedImage {
	return NewTaggedImageFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}

type MutableTaggedImage struct {
	objID int32
	keyID wasmlib.Key32
}

func (o MutableTaggedImage) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o MutableTaggedImage) SetValue(value *TaggedImage) {
	wasmlib.SetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES, value.Bytes())
}

func (o MutableTaggedImage) Value() *TaggedImage {
	return NewTaggedImageFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}
