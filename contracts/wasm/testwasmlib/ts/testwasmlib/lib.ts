// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib"
import * as sc from "./index";

export function on_call(index: i32): void {
    return wasmlib.onCall(index);
}

export function on_load(): void {
    let exports = new wasmlib.ScExports();
    exports.addFunc(sc.FuncArrayClear, funcArrayClearThunk);
    exports.addFunc(sc.FuncArrayCreate, funcArrayCreateThunk);
    exports.addFunc(sc.FuncArraySet, funcArraySetThunk);
    exports.addFunc(sc.FuncParamTypes, funcParamTypesThunk);
    exports.addView(sc.ViewArrayLength, viewArrayLengthThunk);
    exports.addView(sc.ViewArrayValue, viewArrayValueThunk);
    exports.addView(sc.ViewBlockRecord, viewBlockRecordThunk);
    exports.addView(sc.ViewBlockRecords, viewBlockRecordsThunk);
    exports.addView(sc.ViewIotaBalance, viewIotaBalanceThunk);

    for (let i = 0; i < sc.keyMap.length; i++) {
        sc.idxMap[i] = wasmlib.Key32.fromString(sc.keyMap[i]);
    }
}

function funcArrayClearThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log("testwasmlib.funcArrayClear");
    let f = new sc.ArrayClearContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    ctx.require(f.params.name().exists(), "missing mandatory name")
    sc.funcArrayClear(ctx, f);
    ctx.log("testwasmlib.funcArrayClear ok");
}

function funcArrayCreateThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log("testwasmlib.funcArrayCreate");
    let f = new sc.ArrayCreateContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    ctx.require(f.params.name().exists(), "missing mandatory name")
    sc.funcArrayCreate(ctx, f);
    ctx.log("testwasmlib.funcArrayCreate ok");
}

function funcArraySetThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log("testwasmlib.funcArraySet");
    let f = new sc.ArraySetContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    ctx.require(f.params.index().exists(), "missing mandatory index")
    ctx.require(f.params.name().exists(), "missing mandatory name")
    ctx.require(f.params.value().exists(), "missing mandatory value")
    sc.funcArraySet(ctx, f);
    ctx.log("testwasmlib.funcArraySet ok");
}

function funcParamTypesThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log("testwasmlib.funcParamTypes");
    let f = new sc.ParamTypesContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    sc.funcParamTypes(ctx, f);
    ctx.log("testwasmlib.funcParamTypes ok");
}

function viewArrayLengthThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log("testwasmlib.viewArrayLength");
    let f = new sc.ArrayLengthContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    ctx.require(f.params.name().exists(), "missing mandatory name")
    sc.viewArrayLength(ctx, f);
    ctx.log("testwasmlib.viewArrayLength ok");
}

function viewArrayValueThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log("testwasmlib.viewArrayValue");
    let f = new sc.ArrayValueContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    ctx.require(f.params.index().exists(), "missing mandatory index")
    ctx.require(f.params.name().exists(), "missing mandatory name")
    sc.viewArrayValue(ctx, f);
    ctx.log("testwasmlib.viewArrayValue ok");
}

function viewBlockRecordThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log("testwasmlib.viewBlockRecord");
    let f = new sc.BlockRecordContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    ctx.require(f.params.blockIndex().exists(), "missing mandatory blockIndex")
    ctx.require(f.params.recordIndex().exists(), "missing mandatory recordIndex")
    sc.viewBlockRecord(ctx, f);
    ctx.log("testwasmlib.viewBlockRecord ok");
}

function viewBlockRecordsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log("testwasmlib.viewBlockRecords");
    let f = new sc.BlockRecordsContext();
    f.params.mapID = wasmlib.OBJ_ID_PARAMS;
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    ctx.require(f.params.blockIndex().exists(), "missing mandatory blockIndex")
    sc.viewBlockRecords(ctx, f);
    ctx.log("testwasmlib.viewBlockRecords ok");
}

function viewIotaBalanceThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log("testwasmlib.viewIotaBalance");
    let f = new sc.IotaBalanceContext();
    f.results.mapID = wasmlib.OBJ_ID_RESULTS;
    f.state.mapID = wasmlib.OBJ_ID_STATE;
    sc.viewIotaBalance(ctx, f);
    ctx.log("testwasmlib.viewIotaBalance ok");
}
