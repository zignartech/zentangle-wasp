// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]

use crate::coregovernance::*;
use crate::*;

pub struct AddAllowedStateControllerAddressCall {
	pub func: ScFunc,
	pub params: MutableAddAllowedStateControllerAddressParams,
}

pub struct ClaimChainOwnershipCall {
	pub func: ScFunc,
}

pub struct DelegateChainOwnershipCall {
	pub func: ScFunc,
	pub params: MutableDelegateChainOwnershipParams,
}

pub struct RemoveAllowedStateControllerAddressCall {
	pub func: ScFunc,
	pub params: MutableRemoveAllowedStateControllerAddressParams,
}

pub struct RotateStateControllerCall {
	pub func: ScFunc,
	pub params: MutableRotateStateControllerParams,
}

pub struct SetChainInfoCall {
	pub func: ScFunc,
	pub params: MutableSetChainInfoParams,
}

pub struct SetContractFeeCall {
	pub func: ScFunc,
	pub params: MutableSetContractFeeParams,
}

pub struct SetDefaultFeeCall {
	pub func: ScFunc,
	pub params: MutableSetDefaultFeeParams,
}

pub struct GetAllowedStateControllerAddressesCall {
	pub func: ScView,
	pub results: ImmutableGetAllowedStateControllerAddressesResults,
}

pub struct GetChainInfoCall {
	pub func: ScView,
	pub results: ImmutableGetChainInfoResults,
}

pub struct GetFeeInfoCall {
	pub func: ScView,
	pub params: MutableGetFeeInfoParams,
	pub results: ImmutableGetFeeInfoResults,
}

pub struct GetMaxBlobSizeCall {
	pub func: ScView,
	pub results: ImmutableGetMaxBlobSizeResults,
}

pub struct ScFuncs {
}

impl ScFuncs {
    pub fn add_allowed_state_controller_address(_ctx: &dyn ScFuncCallContext) -> AddAllowedStateControllerAddressCall {
        let mut f = AddAllowedStateControllerAddressCall {
            func: ScFunc::new(HSC_NAME, HFUNC_ADD_ALLOWED_STATE_CONTROLLER_ADDRESS),
            params: MutableAddAllowedStateControllerAddressParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn claim_chain_ownership(_ctx: &dyn ScFuncCallContext) -> ClaimChainOwnershipCall {
        ClaimChainOwnershipCall {
            func: ScFunc::new(HSC_NAME, HFUNC_CLAIM_CHAIN_OWNERSHIP),
        }
    }

    pub fn delegate_chain_ownership(_ctx: &dyn ScFuncCallContext) -> DelegateChainOwnershipCall {
        let mut f = DelegateChainOwnershipCall {
            func: ScFunc::new(HSC_NAME, HFUNC_DELEGATE_CHAIN_OWNERSHIP),
            params: MutableDelegateChainOwnershipParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn remove_allowed_state_controller_address(_ctx: &dyn ScFuncCallContext) -> RemoveAllowedStateControllerAddressCall {
        let mut f = RemoveAllowedStateControllerAddressCall {
            func: ScFunc::new(HSC_NAME, HFUNC_REMOVE_ALLOWED_STATE_CONTROLLER_ADDRESS),
            params: MutableRemoveAllowedStateControllerAddressParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn rotate_state_controller(_ctx: &dyn ScFuncCallContext) -> RotateStateControllerCall {
        let mut f = RotateStateControllerCall {
            func: ScFunc::new(HSC_NAME, HFUNC_ROTATE_STATE_CONTROLLER),
            params: MutableRotateStateControllerParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn set_chain_info(_ctx: &dyn ScFuncCallContext) -> SetChainInfoCall {
        let mut f = SetChainInfoCall {
            func: ScFunc::new(HSC_NAME, HFUNC_SET_CHAIN_INFO),
            params: MutableSetChainInfoParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn set_contract_fee(_ctx: &dyn ScFuncCallContext) -> SetContractFeeCall {
        let mut f = SetContractFeeCall {
            func: ScFunc::new(HSC_NAME, HFUNC_SET_CONTRACT_FEE),
            params: MutableSetContractFeeParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn set_default_fee(_ctx: &dyn ScFuncCallContext) -> SetDefaultFeeCall {
        let mut f = SetDefaultFeeCall {
            func: ScFunc::new(HSC_NAME, HFUNC_SET_DEFAULT_FEE),
            params: MutableSetDefaultFeeParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn get_allowed_state_controller_addresses(_ctx: &dyn ScViewCallContext) -> GetAllowedStateControllerAddressesCall {
        let mut f = GetAllowedStateControllerAddressesCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_ALLOWED_STATE_CONTROLLER_ADDRESSES),
            results: ImmutableGetAllowedStateControllerAddressesResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_chain_info(_ctx: &dyn ScViewCallContext) -> GetChainInfoCall {
        let mut f = GetChainInfoCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_CHAIN_INFO),
            results: ImmutableGetChainInfoResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_fee_info(_ctx: &dyn ScViewCallContext) -> GetFeeInfoCall {
        let mut f = GetFeeInfoCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_FEE_INFO),
            params: MutableGetFeeInfoParams { proxy: Proxy::nil() },
            results: ImmutableGetFeeInfoResults { proxy: Proxy::nil() },
        };
        ScView::link_params(&mut f.params.proxy, &f.func);
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_max_blob_size(_ctx: &dyn ScViewCallContext) -> GetMaxBlobSizeCall {
        let mut f = GetMaxBlobSizeCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_MAX_BLOB_SIZE),
            results: ImmutableGetMaxBlobSizeResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }
}
