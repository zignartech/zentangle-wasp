// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package tokenregistry

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type Token struct {
	Created     int64             // creation timestamp
	Description string            // description what minted token represents
	MintedBy    wasmlib.ScAgentID // original minter
	Owner       wasmlib.ScAgentID // current owner
	Supply      int64             // amount of tokens originally minted
	Updated     int64             // last update timestamp
	UserDefined string            // any user defined text
}

func NewTokenFromBytes(bytes []byte) *Token {
	decode := wasmlib.NewBytesDecoder(bytes)
	data := &Token{}
	data.Created = decode.Int64()
	data.Description = decode.String()
	data.MintedBy = decode.AgentID()
	data.Owner = decode.AgentID()
	data.Supply = decode.Int64()
	data.Updated = decode.Int64()
	data.UserDefined = decode.String()
	decode.Close()
	return data
}

func (o *Token) Bytes() []byte {
	return wasmlib.NewBytesEncoder().
		Int64(o.Created).
		String(o.Description).
		AgentID(o.MintedBy).
		AgentID(o.Owner).
		Int64(o.Supply).
		Int64(o.Updated).
		String(o.UserDefined).
		Data()
}

type ImmutableToken struct {
	objID int32
	keyID wasmlib.Key32
}

func (o ImmutableToken) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o ImmutableToken) Value() *Token {
	return NewTokenFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}

type MutableToken struct {
	objID int32
	keyID wasmlib.Key32
}

func (o MutableToken) Exists() bool {
	return wasmlib.Exists(o.objID, o.keyID, wasmlib.TYPE_BYTES)
}

func (o MutableToken) SetValue(value *Token) {
	wasmlib.SetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES, value.Bytes())
}

func (o MutableToken) Value() *Token {
	return NewTokenFromBytes(wasmlib.GetBytes(o.objID, o.keyID, wasmlib.TYPE_BYTES))
}
