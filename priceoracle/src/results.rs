// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use crate::*;

#[derive(Clone)]
pub struct ImmutableGetMiotaPriceResults {
	pub(crate) proxy: Proxy,
}

impl ImmutableGetMiotaPriceResults {
    pub fn price(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.proxy.root(RESULT_PRICE))
	}
}

#[derive(Clone)]
pub struct MutableGetMiotaPriceResults {
	pub(crate) proxy: Proxy,
}

impl MutableGetMiotaPriceResults {
    pub fn price(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.proxy.root(RESULT_PRICE))
	}
}

#[derive(Clone)]
pub struct ImmutableGetOwnerResults {
	pub(crate) proxy: Proxy,
}

impl ImmutableGetOwnerResults {
    pub fn owner(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.proxy.root(RESULT_OWNER))
	}
}

#[derive(Clone)]
pub struct MutableGetOwnerResults {
	pub(crate) proxy: Proxy,
}

impl MutableGetOwnerResults {
    pub fn owner(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.proxy.root(RESULT_OWNER))
	}
}
