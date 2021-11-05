// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib"
import * as sc from "./index";

export class ImmutablePlaceBetParams extends wasmlib.ScMapID {

    number(): wasmlib.ScImmutableInt64 {
        return new wasmlib.ScImmutableInt64(this.mapID, sc.idxMap[sc.IdxParamNumber]);
    }
}

export class MutablePlaceBetParams extends wasmlib.ScMapID {

    number(): wasmlib.ScMutableInt64 {
        return new wasmlib.ScMutableInt64(this.mapID, sc.idxMap[sc.IdxParamNumber]);
    }
}

export class ImmutablePlayPeriodParams extends wasmlib.ScMapID {

    playPeriod(): wasmlib.ScImmutableInt32 {
        return new wasmlib.ScImmutableInt32(this.mapID, sc.idxMap[sc.IdxParamPlayPeriod]);
    }
}

export class MutablePlayPeriodParams extends wasmlib.ScMapID {

    playPeriod(): wasmlib.ScMutableInt32 {
        return new wasmlib.ScMutableInt32(this.mapID, sc.idxMap[sc.IdxParamPlayPeriod]);
    }
}
