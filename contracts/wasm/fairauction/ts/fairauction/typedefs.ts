// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmtypes from "wasmlib/wasmtypes";
import * as sc from "./index";

export class ArrayOfImmutableAgentID extends wasmtypes.ScProxy {

	length(): u32 {
		return this.proxy.length();
	}

	getAgentID(index: u32): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.index(index));
	}
}

export class ImmutableBidderList extends ArrayOfImmutableAgentID {
}

export class ArrayOfMutableAgentID extends wasmtypes.ScProxy {

	appendAgentID(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.append());
	}

	clear(): void {
		this.proxy.clearArray();
	}

	length(): u32 {
		return this.proxy.length();
	}

	getAgentID(index: u32): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.index(index));
	}
}

export class MutableBidderList extends ArrayOfMutableAgentID {
}

export class MapAgentIDToImmutableBid extends wasmtypes.ScProxy {

	getBid(key: wasmtypes.ScAgentID): sc.ImmutableBid {
		return new sc.ImmutableBid(this.proxy.key(wasmtypes.agentIDToBytes(key)));
	}
}

export class ImmutableBids extends MapAgentIDToImmutableBid {
}

export class MapAgentIDToMutableBid extends wasmtypes.ScProxy {

	clear(): void {
		this.proxy.clearMap();
	}

	getBid(key: wasmtypes.ScAgentID): sc.MutableBid {
		return new sc.MutableBid(this.proxy.key(wasmtypes.agentIDToBytes(key)));
	}
}

export class MutableBids extends MapAgentIDToMutableBid {
}
