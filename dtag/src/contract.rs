// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

// @formatter:off

#![allow(dead_code)]

use std::ptr;

use wasmlib::*;

use crate::consts::*;
use crate::params::*;
use crate::results::*;

pub struct CreateGameCall {
    pub func:   ScFunc,
    pub params: MutableCreateGameParams,
}

pub struct EndGameCall {
    pub func: ScFunc,
}

pub struct RequestPlayCall {
    pub func:    ScFunc,
    pub results: ImmutableRequestPlayResults,
}

pub struct SendTagsCall {
    pub func:   ScFunc,
    pub params: MutableSendTagsParams,
}

pub struct GetPlaysPerImageCall {
    pub func:    ScView,
    pub results: ImmutableGetPlaysPerImageResults,
}

pub struct GetResultsCall {
    pub func:    ScView,
    pub results: ImmutableGetResultsResults,
}

pub struct GetTaggedImagesCall {
    pub func:    ScView,
    pub results: ImmutableGetTaggedImagesResults,
}

pub struct ScFuncs {
}

impl ScFuncs {
    pub fn create_game(_ctx: & dyn ScFuncCallContext) -> CreateGameCall {
        let mut f = CreateGameCall {
            func:   ScFunc::new(HSC_NAME, HFUNC_CREATE_GAME),
            params: MutableCreateGameParams { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, ptr::null_mut());
        f
    }
    pub fn end_game(_ctx: & dyn ScFuncCallContext) -> EndGameCall {
        EndGameCall {
            func: ScFunc::new(HSC_NAME, HFUNC_END_GAME),
        }
    }
    pub fn request_play(_ctx: & dyn ScFuncCallContext) -> RequestPlayCall {
        let mut f = RequestPlayCall {
            func:    ScFunc::new(HSC_NAME, HFUNC_REQUEST_PLAY),
            results: ImmutableRequestPlayResults { id: 0 },
        };
        f.func.set_ptrs(ptr::null_mut(), &mut f.results.id);
        f
    }
    pub fn send_tags(_ctx: & dyn ScFuncCallContext) -> SendTagsCall {
        let mut f = SendTagsCall {
            func:   ScFunc::new(HSC_NAME, HFUNC_SEND_TAGS),
            params: MutableSendTagsParams { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, ptr::null_mut());
        f
    }
    pub fn get_plays_per_image(_ctx: & dyn ScViewCallContext) -> GetPlaysPerImageCall {
        let mut f = GetPlaysPerImageCall {
            func:    ScView::new(HSC_NAME, HVIEW_GET_PLAYS_PER_IMAGE),
            results: ImmutableGetPlaysPerImageResults { id: 0 },
        };
        f.func.set_ptrs(ptr::null_mut(), &mut f.results.id);
        f
    }
    pub fn get_results(_ctx: & dyn ScViewCallContext) -> GetResultsCall {
        let mut f = GetResultsCall {
            func:    ScView::new(HSC_NAME, HVIEW_GET_RESULTS),
            results: ImmutableGetResultsResults { id: 0 },
        };
        f.func.set_ptrs(ptr::null_mut(), &mut f.results.id);
        f
    }
    pub fn get_tagged_images(_ctx: & dyn ScViewCallContext) -> GetTaggedImagesCall {
        let mut f = GetTaggedImagesCall {
            func:    ScView::new(HSC_NAME, HVIEW_GET_TAGGED_IMAGES),
            results: ImmutableGetTaggedImagesResults { id: 0 },
        };
        f.func.set_ptrs(ptr::null_mut(), &mut f.results.id);
        f
    }
}

// @formatter:on
