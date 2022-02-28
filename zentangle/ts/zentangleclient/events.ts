// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmclient from "wasmclient"
import * as app from "./zentangle"

export const eventHandlers: wasmclient.EventHandlers = {
	"zentangle.gameEnded": (msg: string[]) => app.onZentangleGameEnded(new EventGameEnded(msg)),
	"zentangle.gameStarted": (msg: string[]) => app.onZentangleGameStarted(new EventGameStarted(msg)),
	"zentangle.imagetagged": (msg: string[]) => app.onZentangleImagetagged(new EventImagetagged(msg)),
	"zentangle.playRequested": (msg: string[]) => app.onZentanglePlayRequested(new EventPlayRequested(msg)),
};

export class EventGameEnded extends wasmclient.Event {
	
	public constructor(msg: string[]) {
		super(msg)
	}
}

export class EventGameStarted extends wasmclient.Event {
	public readonly description: wasmclient.String;
	public readonly numberOfImages: wasmclient.Int32;
	public readonly reward: wasmclient.Int64;
	public readonly tagsRequiredPerImage: wasmclient.Int32;
	
	public constructor(msg: string[]) {
		super(msg)
		this.description = this.nextString();
		this.numberOfImages = this.nextInt32();
		this.reward = this.nextInt64();
		this.tagsRequiredPerImage = this.nextInt32();
	}
}

export class EventImagetagged extends wasmclient.Event {
	public readonly address: wasmclient.String;
	public readonly imageId: wasmclient.Int32;
	public readonly playsPerImage: wasmclient.Int32;
	
	public constructor(msg: string[]) {
		super(msg)
		this.address = this.nextString();
		this.imageId = this.nextInt32();
		this.playsPerImage = this.nextInt32();
	}
}

export class EventPlayRequested extends wasmclient.Event {
	public readonly address: wasmclient.String;
	public readonly amount: wasmclient.Int64;
	public readonly imageId: wasmclient.Int32;
	
	public constructor(msg: string[]) {
		super(msg)
		this.address = this.nextString();
		this.amount = this.nextInt64();
		this.imageId = this.nextInt32();
	}
}