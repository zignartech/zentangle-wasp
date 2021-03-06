// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package fairauction

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"

type Auction struct {
	Color         wasmtypes.ScColor   // color of tokens for sale
	Creator       wasmtypes.ScAgentID // issuer of start_auction transaction
	Deposit       uint64              // deposit by auction owner to cover the SC fees
	Description   string              // auction description
	Duration      uint32              // auction duration in minutes
	HighestBid    uint64              // the current highest bid amount
	HighestBidder wasmtypes.ScAgentID // the current highest bidder
	MinimumBid    uint64              // minimum bid amount
	NumTokens     uint64              // number of tokens for sale
	OwnerMargin   uint64              // auction owner's margin in promilles
	WhenStarted   uint64              // timestamp when auction started
}

func NewAuctionFromBytes(buf []byte) *Auction {
	dec := wasmtypes.NewWasmDecoder(buf)
	data := &Auction{}
	data.Color = wasmtypes.ColorDecode(dec)
	data.Creator = wasmtypes.AgentIDDecode(dec)
	data.Deposit = wasmtypes.Uint64Decode(dec)
	data.Description = wasmtypes.StringDecode(dec)
	data.Duration = wasmtypes.Uint32Decode(dec)
	data.HighestBid = wasmtypes.Uint64Decode(dec)
	data.HighestBidder = wasmtypes.AgentIDDecode(dec)
	data.MinimumBid = wasmtypes.Uint64Decode(dec)
	data.NumTokens = wasmtypes.Uint64Decode(dec)
	data.OwnerMargin = wasmtypes.Uint64Decode(dec)
	data.WhenStarted = wasmtypes.Uint64Decode(dec)
	dec.Close()
	return data
}

func (o *Auction) Bytes() []byte {
	enc := wasmtypes.NewWasmEncoder()
	wasmtypes.ColorEncode(enc, o.Color)
	wasmtypes.AgentIDEncode(enc, o.Creator)
	wasmtypes.Uint64Encode(enc, o.Deposit)
	wasmtypes.StringEncode(enc, o.Description)
	wasmtypes.Uint32Encode(enc, o.Duration)
	wasmtypes.Uint64Encode(enc, o.HighestBid)
	wasmtypes.AgentIDEncode(enc, o.HighestBidder)
	wasmtypes.Uint64Encode(enc, o.MinimumBid)
	wasmtypes.Uint64Encode(enc, o.NumTokens)
	wasmtypes.Uint64Encode(enc, o.OwnerMargin)
	wasmtypes.Uint64Encode(enc, o.WhenStarted)
	return enc.Buf()
}

type ImmutableAuction struct {
	proxy wasmtypes.Proxy
}

func (o ImmutableAuction) Exists() bool {
	return o.proxy.Exists()
}

func (o ImmutableAuction) Value() *Auction {
	return NewAuctionFromBytes(o.proxy.Get())
}

type MutableAuction struct {
	proxy wasmtypes.Proxy
}

func (o MutableAuction) Delete() {
	o.proxy.Delete()
}

func (o MutableAuction) Exists() bool {
	return o.proxy.Exists()
}

func (o MutableAuction) SetValue(value *Auction) {
	o.proxy.Set(value.Bytes())
}

func (o MutableAuction) Value() *Auction {
	return NewAuctionFromBytes(o.proxy.Get())
}

type Bid struct {
	Amount    uint64 // cumulative amount of bids from same bidder
	Index     uint32 // index of bidder in bidder list
	Timestamp uint64 // timestamp of most recent bid
}

func NewBidFromBytes(buf []byte) *Bid {
	dec := wasmtypes.NewWasmDecoder(buf)
	data := &Bid{}
	data.Amount = wasmtypes.Uint64Decode(dec)
	data.Index = wasmtypes.Uint32Decode(dec)
	data.Timestamp = wasmtypes.Uint64Decode(dec)
	dec.Close()
	return data
}

func (o *Bid) Bytes() []byte {
	enc := wasmtypes.NewWasmEncoder()
	wasmtypes.Uint64Encode(enc, o.Amount)
	wasmtypes.Uint32Encode(enc, o.Index)
	wasmtypes.Uint64Encode(enc, o.Timestamp)
	return enc.Buf()
}

type ImmutableBid struct {
	proxy wasmtypes.Proxy
}

func (o ImmutableBid) Exists() bool {
	return o.proxy.Exists()
}

func (o ImmutableBid) Value() *Bid {
	return NewBidFromBytes(o.proxy.Get())
}

type MutableBid struct {
	proxy wasmtypes.Proxy
}

func (o MutableBid) Delete() {
	o.proxy.Delete()
}

func (o MutableBid) Exists() bool {
	return o.proxy.Exists()
}

func (o MutableBid) SetValue(value *Bid) {
	o.proxy.Set(value.Bytes())
}

func (o MutableBid) Value() *Bid {
	return NewBidFromBytes(o.proxy.Get())
}
