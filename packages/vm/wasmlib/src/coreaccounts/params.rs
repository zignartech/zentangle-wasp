// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use crate::coreaccounts::*;
use crate::host::*;

#[derive(Clone, Copy)]
pub struct ImmutableDepositParams {
    pub(crate) id: i32,
}

impl ImmutableDepositParams {
    pub fn agent_id(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.id, PARAM_AGENT_ID.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct MutableDepositParams {
    pub(crate) id: i32,
}

impl MutableDepositParams {
    pub fn agent_id(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.id, PARAM_AGENT_ID.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableHarvestParams {
    pub(crate) id: i32,
}

impl ImmutableHarvestParams {
    pub fn withdraw_amount(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, PARAM_WITHDRAW_AMOUNT.get_key_id())
	}

    pub fn withdraw_color(&self) -> ScImmutableColor {
		ScImmutableColor::new(self.id, PARAM_WITHDRAW_COLOR.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct MutableHarvestParams {
    pub(crate) id: i32,
}

impl MutableHarvestParams {
    pub fn withdraw_amount(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, PARAM_WITHDRAW_AMOUNT.get_key_id())
	}

    pub fn withdraw_color(&self) -> ScMutableColor {
		ScMutableColor::new(self.id, PARAM_WITHDRAW_COLOR.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableBalanceParams {
    pub(crate) id: i32,
}

impl ImmutableBalanceParams {
    pub fn agent_id(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.id, PARAM_AGENT_ID.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct MutableBalanceParams {
    pub(crate) id: i32,
}

impl MutableBalanceParams {
    pub fn agent_id(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.id, PARAM_AGENT_ID.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableGetAccountNonceParams {
    pub(crate) id: i32,
}

impl ImmutableGetAccountNonceParams {
    pub fn agent_id(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.id, PARAM_AGENT_ID.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct MutableGetAccountNonceParams {
    pub(crate) id: i32,
}

impl MutableGetAccountNonceParams {
    pub fn agent_id(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.id, PARAM_AGENT_ID.get_key_id())
	}
}
