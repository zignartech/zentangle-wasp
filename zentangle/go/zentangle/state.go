// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package zentangle

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"

type ArrayOfImmutableBet struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfImmutableBet) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfImmutableBet) GetBet(index uint32) ImmutableBet {
	return ImmutableBet{proxy: a.proxy.Index(index)}
}

type MapStringToImmutableBet struct {
	proxy wasmtypes.Proxy
}

func (m MapStringToImmutableBet) GetBet(key string) ImmutableBet {
	return ImmutableBet{proxy: m.proxy.Key(wasmtypes.StringToBytes(key))}
}

type MapStringToImmutablePlayerInfo struct {
	proxy wasmtypes.Proxy
}

func (m MapStringToImmutablePlayerInfo) GetPlayerInfo(key string) ImmutablePlayerInfo {
	return ImmutablePlayerInfo{proxy: m.proxy.Key(wasmtypes.StringToBytes(key))}
}

type ArrayOfImmutableString struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfImmutableString) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfImmutableString) GetString(index uint32) wasmtypes.ScImmutableString {
	return wasmtypes.NewScImmutableString(a.proxy.Index(index))
}

type ArrayOfImmutableUint32 struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfImmutableUint32) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfImmutableUint32) GetUint32(index uint32) wasmtypes.ScImmutableUint32 {
	return wasmtypes.NewScImmutableUint32(a.proxy.Index(index))
}

type ArrayOfImmutableTgdImg struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfImmutableTgdImg) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfImmutableTgdImg) GetTgdImg(index uint32) ImmutableTgdImg {
	return ImmutableTgdImg{proxy: a.proxy.Index(index)}
}

type MapStringToImmutableUint64 struct {
	proxy wasmtypes.Proxy
}

func (m MapStringToImmutableUint64) GetUint64(key string) wasmtypes.ScImmutableUint64 {
	return wasmtypes.NewScImmutableUint64(m.proxy.Key(wasmtypes.StringToBytes(key)))
}

type ArrayOfImmutableValidTag struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfImmutableValidTag) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfImmutableValidTag) GetValidTag(index uint32) ImmutableValidTag {
	return ImmutableValidTag{proxy: a.proxy.Index(index)}
}

type ImmutablezentangleState struct {
	proxy wasmtypes.Proxy
}

func (s ImmutablezentangleState) Bets() ArrayOfImmutableBet {
	return ArrayOfImmutableBet{proxy: s.proxy.Root(StateBets)}
}

func (s ImmutablezentangleState) CompleteImages() wasmtypes.ScImmutableUint32 {
	return wasmtypes.NewScImmutableUint32(s.proxy.Root(StateCompleteImages))
}

func (s ImmutablezentangleState) Creator() wasmtypes.ScImmutableAgentID {
	return wasmtypes.NewScImmutableAgentID(s.proxy.Root(StateCreator))
}

func (s ImmutablezentangleState) Description() wasmtypes.ScImmutableString {
	return wasmtypes.NewScImmutableString(s.proxy.Root(StateDescription))
}

func (s ImmutablezentangleState) NumberOfImages() wasmtypes.ScImmutableUint32 {
	return wasmtypes.NewScImmutableUint32(s.proxy.Root(StateNumberOfImages))
}

func (s ImmutablezentangleState) Owner() wasmtypes.ScImmutableAgentID {
	return wasmtypes.NewScImmutableAgentID(s.proxy.Root(StateOwner))
}

func (s ImmutablezentangleState) PendingPlay() MapStringToImmutableBet {
	return MapStringToImmutableBet{proxy: s.proxy.Root(StatePendingPlay)}
}

func (s ImmutablezentangleState) PendingPlays() ArrayOfImmutableBet {
	return ArrayOfImmutableBet{proxy: s.proxy.Root(StatePendingPlays)}
}

func (s ImmutablezentangleState) PlayerInfo() MapStringToImmutablePlayerInfo {
	return MapStringToImmutablePlayerInfo{proxy: s.proxy.Root(StatePlayerInfo)}
}

func (s ImmutablezentangleState) PlayersInfo() ArrayOfImmutableString {
	return ArrayOfImmutableString{proxy: s.proxy.Root(StatePlayersInfo)}
}

func (s ImmutablezentangleState) PlaysPerImage() ArrayOfImmutableUint32 {
	return ArrayOfImmutableUint32{proxy: s.proxy.Root(StatePlaysPerImage)}
}

func (s ImmutablezentangleState) PlaysRequiredPerImage() wasmtypes.ScImmutableUint32 {
	return wasmtypes.NewScImmutableUint32(s.proxy.Root(StatePlaysRequiredPerImage))
}

func (s ImmutablezentangleState) ProcessedImages() ArrayOfImmutableTgdImg {
	return ArrayOfImmutableTgdImg{proxy: s.proxy.Root(StateProcessedImages)}
}

func (s ImmutablezentangleState) Reward() wasmtypes.ScImmutableUint64 {
	return wasmtypes.NewScImmutableUint64(s.proxy.Root(StateReward))
}

func (s ImmutablezentangleState) TgdImgs() ArrayOfImmutableTgdImg {
	return ArrayOfImmutableTgdImg{proxy: s.proxy.Root(StateTgdImgs)}
}

func (s ImmutablezentangleState) TotalPlayerTags() MapStringToImmutableUint64 {
	return MapStringToImmutableUint64{proxy: s.proxy.Root(StateTotalPlayerTags)}
}

func (s ImmutablezentangleState) ValidTags() ArrayOfImmutableValidTag {
	return ArrayOfImmutableValidTag{proxy: s.proxy.Root(StateValidTags)}
}

type ArrayOfMutableBet struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfMutableBet) AppendBet() MutableBet {
	return MutableBet{proxy: a.proxy.Append()}
}

func (a ArrayOfMutableBet) Clear() {
	a.proxy.ClearArray()
}

func (a ArrayOfMutableBet) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfMutableBet) GetBet(index uint32) MutableBet {
	return MutableBet{proxy: a.proxy.Index(index)}
}

type MapStringToMutableBet struct {
	proxy wasmtypes.Proxy
}

func (m MapStringToMutableBet) Clear() {
	m.proxy.ClearMap()
}

func (m MapStringToMutableBet) GetBet(key string) MutableBet {
	return MutableBet{proxy: m.proxy.Key(wasmtypes.StringToBytes(key))}
}

type MapStringToMutablePlayerInfo struct {
	proxy wasmtypes.Proxy
}

func (m MapStringToMutablePlayerInfo) Clear() {
	m.proxy.ClearMap()
}

func (m MapStringToMutablePlayerInfo) GetPlayerInfo(key string) MutablePlayerInfo {
	return MutablePlayerInfo{proxy: m.proxy.Key(wasmtypes.StringToBytes(key))}
}

type ArrayOfMutableString struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfMutableString) AppendString() wasmtypes.ScMutableString {
	return wasmtypes.NewScMutableString(a.proxy.Append())
}

func (a ArrayOfMutableString) Clear() {
	a.proxy.ClearArray()
}

func (a ArrayOfMutableString) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfMutableString) GetString(index uint32) wasmtypes.ScMutableString {
	return wasmtypes.NewScMutableString(a.proxy.Index(index))
}

type ArrayOfMutableUint32 struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfMutableUint32) AppendUint32() wasmtypes.ScMutableUint32 {
	return wasmtypes.NewScMutableUint32(a.proxy.Append())
}

func (a ArrayOfMutableUint32) Clear() {
	a.proxy.ClearArray()
}

func (a ArrayOfMutableUint32) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfMutableUint32) GetUint32(index uint32) wasmtypes.ScMutableUint32 {
	return wasmtypes.NewScMutableUint32(a.proxy.Index(index))
}

type ArrayOfMutableTgdImg struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfMutableTgdImg) AppendTgdImg() MutableTgdImg {
	return MutableTgdImg{proxy: a.proxy.Append()}
}

func (a ArrayOfMutableTgdImg) Clear() {
	a.proxy.ClearArray()
}

func (a ArrayOfMutableTgdImg) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfMutableTgdImg) GetTgdImg(index uint32) MutableTgdImg {
	return MutableTgdImg{proxy: a.proxy.Index(index)}
}

type MapStringToMutableUint64 struct {
	proxy wasmtypes.Proxy
}

func (m MapStringToMutableUint64) Clear() {
	m.proxy.ClearMap()
}

func (m MapStringToMutableUint64) GetUint64(key string) wasmtypes.ScMutableUint64 {
	return wasmtypes.NewScMutableUint64(m.proxy.Key(wasmtypes.StringToBytes(key)))
}

type ArrayOfMutableValidTag struct {
	proxy wasmtypes.Proxy
}

func (a ArrayOfMutableValidTag) AppendValidTag() MutableValidTag {
	return MutableValidTag{proxy: a.proxy.Append()}
}

func (a ArrayOfMutableValidTag) Clear() {
	a.proxy.ClearArray()
}

func (a ArrayOfMutableValidTag) Length() uint32 {
	return a.proxy.Length()
}

func (a ArrayOfMutableValidTag) GetValidTag(index uint32) MutableValidTag {
	return MutableValidTag{proxy: a.proxy.Index(index)}
}

type MutablezentangleState struct {
	proxy wasmtypes.Proxy
}

func (s MutablezentangleState) AsImmutable() ImmutablezentangleState {
	return ImmutablezentangleState(s)
}

func (s MutablezentangleState) Bets() ArrayOfMutableBet {
	return ArrayOfMutableBet{proxy: s.proxy.Root(StateBets)}
}

func (s MutablezentangleState) CompleteImages() wasmtypes.ScMutableUint32 {
	return wasmtypes.NewScMutableUint32(s.proxy.Root(StateCompleteImages))
}

func (s MutablezentangleState) Creator() wasmtypes.ScMutableAgentID {
	return wasmtypes.NewScMutableAgentID(s.proxy.Root(StateCreator))
}

func (s MutablezentangleState) Description() wasmtypes.ScMutableString {
	return wasmtypes.NewScMutableString(s.proxy.Root(StateDescription))
}

func (s MutablezentangleState) NumberOfImages() wasmtypes.ScMutableUint32 {
	return wasmtypes.NewScMutableUint32(s.proxy.Root(StateNumberOfImages))
}

func (s MutablezentangleState) Owner() wasmtypes.ScMutableAgentID {
	return wasmtypes.NewScMutableAgentID(s.proxy.Root(StateOwner))
}

func (s MutablezentangleState) PendingPlay() MapStringToMutableBet {
	return MapStringToMutableBet{proxy: s.proxy.Root(StatePendingPlay)}
}

func (s MutablezentangleState) PendingPlays() ArrayOfMutableBet {
	return ArrayOfMutableBet{proxy: s.proxy.Root(StatePendingPlays)}
}

func (s MutablezentangleState) PlayerInfo() MapStringToMutablePlayerInfo {
	return MapStringToMutablePlayerInfo{proxy: s.proxy.Root(StatePlayerInfo)}
}

func (s MutablezentangleState) PlayersInfo() ArrayOfMutableString {
	return ArrayOfMutableString{proxy: s.proxy.Root(StatePlayersInfo)}
}

func (s MutablezentangleState) PlaysPerImage() ArrayOfMutableUint32 {
	return ArrayOfMutableUint32{proxy: s.proxy.Root(StatePlaysPerImage)}
}

func (s MutablezentangleState) PlaysRequiredPerImage() wasmtypes.ScMutableUint32 {
	return wasmtypes.NewScMutableUint32(s.proxy.Root(StatePlaysRequiredPerImage))
}

func (s MutablezentangleState) ProcessedImages() ArrayOfMutableTgdImg {
	return ArrayOfMutableTgdImg{proxy: s.proxy.Root(StateProcessedImages)}
}

func (s MutablezentangleState) Reward() wasmtypes.ScMutableUint64 {
	return wasmtypes.NewScMutableUint64(s.proxy.Root(StateReward))
}

func (s MutablezentangleState) TgdImgs() ArrayOfMutableTgdImg {
	return ArrayOfMutableTgdImg{proxy: s.proxy.Root(StateTgdImgs)}
}

func (s MutablezentangleState) TotalPlayerTags() MapStringToMutableUint64 {
	return MapStringToMutableUint64{proxy: s.proxy.Root(StateTotalPlayerTags)}
}

func (s MutablezentangleState) ValidTags() ArrayOfMutableValidTag {
	return ArrayOfMutableValidTag{proxy: s.proxy.Root(StateValidTags)}
}
