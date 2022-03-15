// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

//nolint:gocritic
package zentangle

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type zentangleEvents struct {
}

func (e zentangleEvents) GameEnded() {
	wasmlib.NewEventEncoder("zentangle.gameEnded").
		Emit()
}

func (e zentangleEvents) GameStarted(description string, numberOfImages uint32, reward uint64, tagsRequiredPerImage uint32) {
	wasmlib.NewEventEncoder("zentangle.gameStarted").
		String(description).
		Uint32(numberOfImages).
		Uint64(reward).
		Uint32(tagsRequiredPerImage).
		Emit()
}

func (e zentangleEvents) Imagetagged(address string, imageId uint32, playsPerImage uint32) {
	wasmlib.NewEventEncoder("zentangle.imagetagged").
		String(address).
		Uint32(imageId).
		Uint32(playsPerImage).
		Emit()
}

func (e zentangleEvents) Paid(accuracy string, amount uint64, bet uint64, boost uint8, player string, position uint64) {
	wasmlib.NewEventEncoder("zentangle.paid").
		String(accuracy).
		Uint64(amount).
		Uint64(bet).
		Uint8(boost).
		String(player).
		Uint64(position).
		Emit()
}

func (e zentangleEvents) PlayRequested(address string, amount uint64, imageId uint32) {
	wasmlib.NewEventEncoder("zentangle.playRequested").
		String(address).
		Uint64(amount).
		Uint32(imageId).
		Emit()
}
