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

pub struct InitCall {
	pub func: ScInitFunc,
	pub params: MutableInitParams,
}

pub struct LoadAddressesCall {
	pub func: ScFunc,
	pub params: MutableLoadAddressesParams,
}

pub struct RuffleCall {
	pub func: ScFunc,
	pub params: MutableRuffleParams,
	pub results: ImmutableRuffleResults,
}

pub struct SetOwnerCall {
	pub func: ScFunc,
	pub params: MutableSetOwnerParams,
}

pub struct UnloadAddressesCall {
	pub func: ScFunc,
}

pub struct GetOwnerCall {
	pub func: ScView,
	pub results: ImmutableGetOwnerResults,
}

pub struct ScFuncs {
}

impl ScFuncs {
    pub fn init(_ctx: & dyn ScFuncCallContext) -> InitCall {
        let mut f = InitCall {
            func: ScInitFunc::new(HSC_NAME, HFUNC_INIT),
            params: MutableInitParams { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, ptr::null_mut());
        f
    }

    pub fn load_addresses(_ctx: & dyn ScFuncCallContext) -> LoadAddressesCall {
        let mut f = LoadAddressesCall {
            func: ScFunc::new(HSC_NAME, HFUNC_LOAD_ADDRESSES),
            params: MutableLoadAddressesParams { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, ptr::null_mut());
        f
    }

    pub fn ruffle(_ctx: & dyn ScFuncCallContext) -> RuffleCall {
        let mut f = RuffleCall {
            func: ScFunc::new(HSC_NAME, HFUNC_RUFFLE),
            params: MutableRuffleParams { id: 0 },
            results: ImmutableRuffleResults { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, &mut f.results.id);
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

    pub fn unload_addresses(_ctx: & dyn ScFuncCallContext) -> UnloadAddressesCall {
        UnloadAddressesCall {
            func: ScFunc::new(HSC_NAME, HFUNC_UNLOAD_ADDRESSES),
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
}
