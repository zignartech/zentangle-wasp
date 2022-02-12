// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]

use std::ptr;

use wasmlib::*;

use crate::consts::*;
use crate::params::*;
use crate::results::*;

pub struct CreateGameCall {
	pub func: ScFunc,
	pub params: MutableCreateGameParams,
}

pub struct EndGameCall {
	pub func: ScFunc,
}

pub struct InitCall {
	pub func: ScInitFunc,
	pub params: MutableInitParams,
}

pub struct RequestPlayCall {
	pub func: ScFunc,
	pub results: ImmutableRequestPlayResults,
}

pub struct SendTagsCall {
	pub func: ScFunc,
	pub params: MutableSendTagsParams,
}

pub struct SetOwnerCall {
	pub func: ScFunc,
	pub params: MutableSetOwnerParams,
}

pub struct WithdrawCall {
	pub func: ScFunc,
}

pub struct GetOwnerCall {
	pub func: ScView,
	pub results: ImmutableGetOwnerResults,
}

pub struct GetPlayerBetsCall {
	pub func: ScView,
	pub results: ImmutableGetPlayerBetsResults,
}

pub struct GetPlayerInfoCall {
	pub func: ScView,
	pub params: MutableGetPlayerInfoParams,
	pub results: ImmutableGetPlayerInfoResults,
}

pub struct GetPlaysPerImageCall {
	pub func: ScView,
	pub params: MutableGetPlaysPerImageParams,
	pub results: ImmutableGetPlaysPerImageResults,
}

pub struct GetResultsCall {
	pub func: ScView,
	pub params: MutableGetResultsParams,
	pub results: ImmutableGetResultsResults,
}

pub struct ScFuncs {
}

impl ScFuncs {
    pub fn create_game(_ctx: & dyn ScFuncCallContext) -> CreateGameCall {
        let mut f = CreateGameCall {
            func: ScFunc::new(HSC_NAME, HFUNC_CREATE_GAME),
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

    pub fn init(_ctx: & dyn ScFuncCallContext) -> InitCall {
        let mut f = InitCall {
            func: ScInitFunc::new(HSC_NAME, HFUNC_INIT),
            params: MutableInitParams { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, ptr::null_mut());
        f
    }

    pub fn request_play(_ctx: & dyn ScFuncCallContext) -> RequestPlayCall {
        let mut f = RequestPlayCall {
            func: ScFunc::new(HSC_NAME, HFUNC_REQUEST_PLAY),
            results: ImmutableRequestPlayResults { id: 0 },
        };
        f.func.set_ptrs(ptr::null_mut(), &mut f.results.id);
        f
    }

    pub fn send_tags(_ctx: & dyn ScFuncCallContext) -> SendTagsCall {
        let mut f = SendTagsCall {
            func: ScFunc::new(HSC_NAME, HFUNC_SEND_TAGS),
            params: MutableSendTagsParams { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, ptr::null_mut());
        f
    }

    pub fn set_owner(_ctx: & dyn ScFuncCallContext) -> SetOwnerCall {
        let mut f = SetOwnerCall {
            func: ScFunc::new(HSC_NAME, HFUNC_SET_OWNER),
            params: MutableSetOwnerParams { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, ptr::null_mut());
        f
    }

    pub fn withdraw(_ctx: & dyn ScFuncCallContext) -> WithdrawCall {
        WithdrawCall {
            func: ScFunc::new(HSC_NAME, HFUNC_WITHDRAW),
        }
    }

    pub fn get_owner(_ctx: & dyn ScViewCallContext) -> GetOwnerCall {
        let mut f = GetOwnerCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_OWNER),
            results: ImmutableGetOwnerResults { id: 0 },
        };
        f.func.set_ptrs(ptr::null_mut(), &mut f.results.id);
        f
    }

    pub fn get_player_bets(_ctx: & dyn ScViewCallContext) -> GetPlayerBetsCall {
        let mut f = GetPlayerBetsCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_PLAYER_BETS),
            results: ImmutableGetPlayerBetsResults { id: 0 },
        };
        f.func.set_ptrs(ptr::null_mut(), &mut f.results.id);
        f
    }

    pub fn get_player_info(_ctx: & dyn ScViewCallContext) -> GetPlayerInfoCall {
        let mut f = GetPlayerInfoCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_PLAYER_INFO),
            params: MutableGetPlayerInfoParams { id: 0 },
            results: ImmutableGetPlayerInfoResults { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, &mut f.results.id);
        f
    }

    pub fn get_plays_per_image(_ctx: & dyn ScViewCallContext) -> GetPlaysPerImageCall {
        let mut f = GetPlaysPerImageCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_PLAYS_PER_IMAGE),
            params: MutableGetPlaysPerImageParams { id: 0 },
            results: ImmutableGetPlaysPerImageResults { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, &mut f.results.id);
        f
    }

    pub fn get_results(_ctx: & dyn ScViewCallContext) -> GetResultsCall {
        let mut f = GetResultsCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_RESULTS),
            params: MutableGetResultsParams { id: 0 },
            results: ImmutableGetResultsResults { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, &mut f.results.id);
        f
    }
}
