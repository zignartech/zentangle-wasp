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
pub struct MapAgentIDToImmutableBytes {
	pub(crate) obj_id: i32,
}

impl MapAgentIDToImmutableBytes {
    pub fn get_bytes(&self, key: &ScAgentID) -> ScImmutableBytes {
        ScImmutableBytes::new(self.obj_id, key.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableAccountsResults {
    pub(crate) id: i32,
}

impl ImmutableAccountsResults {
    pub fn agents(&self) -> MapAgentIDToImmutableBytes {
		MapAgentIDToImmutableBytes { obj_id: self.id }
	}
}

#[derive(Clone, Copy)]
pub struct MapAgentIDToMutableBytes {
	pub(crate) obj_id: i32,
}

impl MapAgentIDToMutableBytes {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn get_bytes(&self, key: &ScAgentID) -> ScMutableBytes {
        ScMutableBytes::new(self.obj_id, key.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableAccountsResults {
    pub(crate) id: i32,
}

impl MutableAccountsResults {
    pub fn agents(&self) -> MapAgentIDToMutableBytes {
		MapAgentIDToMutableBytes { obj_id: self.id }
	}
}

#[derive(Clone, Copy)]
pub struct MapColorToImmutableInt64 {
	pub(crate) obj_id: i32,
}

impl MapColorToImmutableInt64 {
    pub fn get_int64(&self, key: &ScColor) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.obj_id, key.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableBalanceResults {
    pub(crate) id: i32,
}

impl ImmutableBalanceResults {
    pub fn balances(&self) -> MapColorToImmutableInt64 {
		MapColorToImmutableInt64 { obj_id: self.id }
	}
}

#[derive(Clone, Copy)]
pub struct MapColorToMutableInt64 {
	pub(crate) obj_id: i32,
}

impl MapColorToMutableInt64 {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn get_int64(&self, key: &ScColor) -> ScMutableInt64 {
        ScMutableInt64::new(self.obj_id, key.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableBalanceResults {
    pub(crate) id: i32,
}

impl MutableBalanceResults {
    pub fn balances(&self) -> MapColorToMutableInt64 {
		MapColorToMutableInt64 { obj_id: self.id }
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableGetAccountNonceResults {
    pub(crate) id: i32,
}

impl ImmutableGetAccountNonceResults {
    pub fn account_nonce(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, RESULT_ACCOUNT_NONCE.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct MutableGetAccountNonceResults {
    pub(crate) id: i32,
}

impl MutableGetAccountNonceResults {
    pub fn account_nonce(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, RESULT_ACCOUNT_NONCE.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableTotalAssetsResults {
    pub(crate) id: i32,
}

impl ImmutableTotalAssetsResults {
    pub fn balances(&self) -> MapColorToImmutableInt64 {
		MapColorToImmutableInt64 { obj_id: self.id }
	}
}

#[derive(Clone, Copy)]
pub struct MutableTotalAssetsResults {
    pub(crate) id: i32,
}

impl MutableTotalAssetsResults {
    pub fn balances(&self) -> MapColorToMutableInt64 {
		MapColorToMutableInt64 { obj_id: self.id }
	}
}
