// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;
use crate::structs::*;

pub fn func_assign_image(_ctx: &ScFuncContext, _f: &AssignImageContext) {
}

pub fn func_create_game(_ctx: &ScFuncContext, _f: &CreateGameContext) {
}

pub fn func_end_game(_ctx: &ScFuncContext, _f: &EndGameContext) {
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.contract_creator());
}

pub fn func_request_play(_ctx: &ScFuncContext, _f: &RequestPlayContext) {
}

pub fn func_send_tag(_ctx: &ScFuncContext, _f: &SendTagContext) {
}

pub fn view_get_plays_per_image(_ctx: &ScViewContext, _f: &GetPlaysPerImageContext) {
}

pub fn view_get_results(_ctx: &ScViewContext, _f: &GetResultsContext) {
}

pub fn view_get_tagged_images(_ctx: &ScViewContext, _f: &GetTaggedImagesContext) {
}
