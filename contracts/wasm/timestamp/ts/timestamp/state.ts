// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";
import * as sc from "./index";

export class ImmutabletimestampState extends wasmlib.ScMapID {
    timestamp(): wasmlib.ScImmutableInt64 {
		return new wasmlib.ScImmutableInt64(this.mapID, sc.idxMap[sc.IdxStateTimestamp]);
	}
}

export class MutabletimestampState extends wasmlib.ScMapID {
    asImmutable(): sc.ImmutabletimestampState {
		const imm = new sc.ImmutabletimestampState();
		imm.mapID = this.mapID;
		return imm;
	}

    timestamp(): wasmlib.ScMutableInt64 {
		return new wasmlib.ScMutableInt64(this.mapID, sc.idxMap[sc.IdxStateTimestamp]);
	}
}
