// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package zentangle

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

func (o MutableBet) Delete() {
	wasmlib.DelKey(o.objID, o.keyID, wasmlib.TYPE_BYTES)
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

type Player struct {
	NDoubleBoosts  int64  // Number of 2x boost used in the round
	NTags          int64  // Number of tags made by the player in the current round
	NTrippleBoosts int64  // Number of 3x boosts used in the round
	PlayerId       wasmlib.ScAgentID  // The player
}

func NewPlayerFromBytes(bytes []byte) *Player {
	decode := wasmlib.NewBytesDecoder(bytes)
	data := &Player{}
	data.NDoubleBoosts  = decode.Int64()
	data.NTags          = decode.Int64()
	data.NTrippleBoosts = decode.Int64()
	data.PlayerId       = decode.AgentID()
	decode.Close()
	return data
}

func (o *Player) Bytes() []byte {
	return wasmlib.NewBytesEncoder().
		Int64(o.NDoubleBoosts).
		Int64(o.NTags).
		Int64(o.NTrippleBoosts).
		AgentID(o.PlayerId).
		Data()
}

type ImmutablePlayer struct {
	objID int32
	keyID wasmlib.Key32
}

func (o ImmutablePlayer) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o ImmutablePlayer) Value() *Player {
	return NewPlayerFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}

type MutablePlayer struct {
	objID int32
	keyID wasmlib.Key32
}

func (o MutablePlayer) Delete() {
	wasmlib.DelKey(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o MutablePlayer) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o MutablePlayer) SetValue(value *Player) {
	wasmlib.SetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES, value.Bytes())
}

func (o MutablePlayer) Value() *Player {
	return NewPlayerFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}

type TaggedImage struct {
	Boost   string  // if the tags will be boosted or not
	H       string  // heights of the Tags
	ImageId int32 
	Player  wasmlib.ScAgentID  // player that has tagged this image
	W       string  // widths of the Tags
	X       string  // x top-left positions of the Tags
	Y       string  // y top-left positions of the Tags
}

func NewTaggedImageFromBytes(bytes []byte) *TaggedImage {
	decode := wasmlib.NewBytesDecoder(bytes)
	data := &TaggedImage{}
	data.Boost   = decode.String()
	data.H       = decode.String()
	data.ImageId = decode.Int32()
	data.Player  = decode.AgentID()
	data.W       = decode.String()
	data.X       = decode.String()
	data.Y       = decode.String()
	decode.Close()
	return data
}

func (o *TaggedImage) Bytes() []byte {
	return wasmlib.NewBytesEncoder().
		String(o.Boost).
		String(o.H).
		Int32(o.ImageId).
		AgentID(o.Player).
		String(o.W).
		String(o.X).
		String(o.Y).
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

func (o MutableTaggedImage) Delete() {
	wasmlib.DelKey(o.objID, o.keyID, wasmlib.TYPE_BYTES)
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

type ValidTag struct {
	PlayTagId   int32  // Identifier to distinguish different tags in the same play
	Player      wasmlib.ScAgentID  // player placing the bet
	TaggedImage int32 
}

func NewValidTagFromBytes(bytes []byte) *ValidTag {
	decode := wasmlib.NewBytesDecoder(bytes)
	data := &ValidTag{}
	data.PlayTagId   = decode.Int32()
	data.Player      = decode.AgentID()
	data.TaggedImage = decode.Int32()
	decode.Close()
	return data
}

func (o *ValidTag) Bytes() []byte {
	return wasmlib.NewBytesEncoder().
		Int32(o.PlayTagId).
		AgentID(o.Player).
		Int32(o.TaggedImage).
		Data()
}

type ImmutableValidTag struct {
	objID int32
	keyID wasmlib.Key32
}

func (o ImmutableValidTag) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o ImmutableValidTag) Value() *ValidTag {
	return NewValidTagFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}

type MutableValidTag struct {
	objID int32
	keyID wasmlib.Key32
}

func (o MutableValidTag) Delete() {
	wasmlib.DelKey(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o MutableValidTag) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o MutableValidTag) SetValue(value *ValidTag) {
	wasmlib.SetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES, value.Bytes())
}

func (o MutableValidTag) Value() *ValidTag {
	return NewValidTagFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}