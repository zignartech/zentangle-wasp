// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";
import * as sc from "./index";

export class ImmutableRequestPlayResults extends wasmlib.ScMapID {
    imageId(): wasmlib.ScImmutableInt32 {
		return new wasmlib.ScImmutableInt32(this.mapID, sc.idxMap[sc.IdxResultImageId]);
	}
}

export class MutableRequestPlayResults extends wasmlib.ScMapID {
    imageId(): wasmlib.ScMutableInt32 {
		return new wasmlib.ScMutableInt32(this.mapID, sc.idxMap[sc.IdxResultImageId]);
	}
}

export class ImmutableGetOwnerResults extends wasmlib.ScMapID {
    owner(): wasmlib.ScImmutableAgentID {
		return new wasmlib.ScImmutableAgentID(this.mapID, sc.idxMap[sc.IdxResultOwner]);
	}
}

export class MutableGetOwnerResults extends wasmlib.ScMapID {
    owner(): wasmlib.ScMutableAgentID {
		return new wasmlib.ScMutableAgentID(this.mapID, sc.idxMap[sc.IdxResultOwner]);
	}
}

export class ImmutableGetPlayerBetsResults extends wasmlib.ScMapID {
    playerBets(): wasmlib.ScImmutableString {
		return new wasmlib.ScImmutableString(this.mapID, sc.idxMap[sc.IdxResultPlayerBets]);
	}
}

export class MutableGetPlayerBetsResults extends wasmlib.ScMapID {
    playerBets(): wasmlib.ScMutableString {
		return new wasmlib.ScMutableString(this.mapID, sc.idxMap[sc.IdxResultPlayerBets]);
	}
}

export class ImmutableGetPlayerInfoResults extends wasmlib.ScMapID {
    info(): wasmlib.ScImmutableString {
		return new wasmlib.ScImmutableString(this.mapID, sc.idxMap[sc.IdxResultInfo]);
	}
}

export class MutableGetPlayerInfoResults extends wasmlib.ScMapID {
    info(): wasmlib.ScMutableString {
		return new wasmlib.ScMutableString(this.mapID, sc.idxMap[sc.IdxResultInfo]);
	}
}

export class ImmutableGetPlaysPerImageResults extends wasmlib.ScMapID {
    playsPerImage(): wasmlib.ScImmutableInt32 {
		return new wasmlib.ScImmutableInt32(this.mapID, sc.idxMap[sc.IdxResultPlaysPerImage]);
	}
}

export class MutableGetPlaysPerImageResults extends wasmlib.ScMapID {
    playsPerImage(): wasmlib.ScMutableInt32 {
		return new wasmlib.ScMutableInt32(this.mapID, sc.idxMap[sc.IdxResultPlaysPerImage]);
	}
}

export class ImmutableGetResultsResults extends wasmlib.ScMapID {
    results(): wasmlib.ScImmutableString {
		return new wasmlib.ScImmutableString(this.mapID, sc.idxMap[sc.IdxResultResults]);
	}
}

export class MutableGetResultsResults extends wasmlib.ScMapID {
    results(): wasmlib.ScMutableString {
		return new wasmlib.ScMutableString(this.mapID, sc.idxMap[sc.IdxResultResults]);
	}
}