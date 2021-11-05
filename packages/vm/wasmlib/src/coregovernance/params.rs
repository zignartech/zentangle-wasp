// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use crate::coregovernance::*;
use crate::host::*;

#[derive(Clone, Copy)]
pub struct ImmutableAddAllowedStateControllerAddressParams {
    pub(crate) id: i32,
}

impl ImmutableAddAllowedStateControllerAddressParams {
    pub fn chain_owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.id, PARAM_CHAIN_OWNER.get_key_id())
    }

    pub fn fee_color(&self) -> ScImmutableColor {
        ScImmutableColor::new(self.id, PARAM_FEE_COLOR.get_key_id())
    }

    pub fn state_controller_address(&self) -> ScImmutableAddress {
        ScImmutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableAddAllowedStateControllerAddressParams {
    pub(crate) id: i32,
}

impl MutableAddAllowedStateControllerAddressParams {
    pub fn chain_owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.id, PARAM_CHAIN_OWNER.get_key_id())
    }

    pub fn fee_color(&self) -> ScMutableColor {
        ScMutableColor::new(self.id, PARAM_FEE_COLOR.get_key_id())
    }

    pub fn state_controller_address(&self) -> ScMutableAddress {
        ScMutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableDelegateChainOwnershipParams {
    pub(crate) id: i32,
}

impl ImmutableDelegateChainOwnershipParams {
    pub fn chain_owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.id, PARAM_CHAIN_OWNER.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableDelegateChainOwnershipParams {
    pub(crate) id: i32,
}

impl MutableDelegateChainOwnershipParams {
    pub fn chain_owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.id, PARAM_CHAIN_OWNER.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableRemoveAllowedStateControllerAddressParams {
    pub(crate) id: i32,
}

impl ImmutableRemoveAllowedStateControllerAddressParams {
    pub fn state_controller_address(&self) -> ScImmutableAddress {
        ScImmutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableRemoveAllowedStateControllerAddressParams {
    pub(crate) id: i32,
}

impl MutableRemoveAllowedStateControllerAddressParams {
    pub fn state_controller_address(&self) -> ScMutableAddress {
        ScMutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableRotateStateControllerParams {
    pub(crate) id: i32,
}

impl ImmutableRotateStateControllerParams {
    pub fn state_controller_address(&self) -> ScImmutableAddress {
        ScImmutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableRotateStateControllerParams {
    pub(crate) id: i32,
}

impl MutableRotateStateControllerParams {
    pub fn state_controller_address(&self) -> ScMutableAddress {
        ScMutableAddress::new(self.id, PARAM_STATE_CONTROLLER_ADDRESS.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableSetChainInfoParams {
    pub(crate) id: i32,
}

impl ImmutableSetChainInfoParams {
    pub fn max_blob_size(&self) -> ScImmutableInt32 {
        ScImmutableInt32::new(self.id, PARAM_MAX_BLOB_SIZE.get_key_id())
    }

    pub fn max_event_size(&self) -> ScImmutableInt16 {
        ScImmutableInt16::new(self.id, PARAM_MAX_EVENT_SIZE.get_key_id())
    }

    pub fn max_events_per_req(&self) -> ScImmutableInt16 {
        ScImmutableInt16::new(self.id, PARAM_MAX_EVENTS_PER_REQ.get_key_id())
    }

    pub fn owner_fee(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, PARAM_OWNER_FEE.get_key_id())
    }

    pub fn validator_fee(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, PARAM_VALIDATOR_FEE.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableSetChainInfoParams {
    pub(crate) id: i32,
}

impl MutableSetChainInfoParams {
    pub fn max_blob_size(&self) -> ScMutableInt32 {
        ScMutableInt32::new(self.id, PARAM_MAX_BLOB_SIZE.get_key_id())
    }

    pub fn max_event_size(&self) -> ScMutableInt16 {
        ScMutableInt16::new(self.id, PARAM_MAX_EVENT_SIZE.get_key_id())
    }

    pub fn max_events_per_req(&self) -> ScMutableInt16 {
        ScMutableInt16::new(self.id, PARAM_MAX_EVENTS_PER_REQ.get_key_id())
    }

    pub fn owner_fee(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, PARAM_OWNER_FEE.get_key_id())
    }

    pub fn validator_fee(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, PARAM_VALIDATOR_FEE.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableSetContractFeeParams {
    pub(crate) id: i32,
}

impl ImmutableSetContractFeeParams {
    pub fn hname(&self) -> ScImmutableHname {
        ScImmutableHname::new(self.id, PARAM_HNAME.get_key_id())
    }

    pub fn owner_fee(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, PARAM_OWNER_FEE.get_key_id())
    }

    pub fn validator_fee(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, PARAM_VALIDATOR_FEE.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableSetContractFeeParams {
    pub(crate) id: i32,
}

impl MutableSetContractFeeParams {
    pub fn hname(&self) -> ScMutableHname {
        ScMutableHname::new(self.id, PARAM_HNAME.get_key_id())
    }

    pub fn owner_fee(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, PARAM_OWNER_FEE.get_key_id())
    }

    pub fn validator_fee(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, PARAM_VALIDATOR_FEE.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableSetDefaultFeeParams {
    pub(crate) id: i32,
}

impl ImmutableSetDefaultFeeParams {
    pub fn owner_fee(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, PARAM_OWNER_FEE.get_key_id())
    }

    pub fn validator_fee(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, PARAM_VALIDATOR_FEE.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableSetDefaultFeeParams {
    pub(crate) id: i32,
}

impl MutableSetDefaultFeeParams {
    pub fn owner_fee(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, PARAM_OWNER_FEE.get_key_id())
    }

    pub fn validator_fee(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, PARAM_VALIDATOR_FEE.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableGetFeeInfoParams {
    pub(crate) id: i32,
}

impl ImmutableGetFeeInfoParams {
    pub fn hname(&self) -> ScImmutableHname {
        ScImmutableHname::new(self.id, PARAM_HNAME.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableGetFeeInfoParams {
    pub(crate) id: i32,
}

impl MutableGetFeeInfoParams {
    pub fn hname(&self) -> ScMutableHname {
        ScMutableHname::new(self.id, PARAM_HNAME.get_key_id())
    }
}
