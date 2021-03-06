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
pub struct ImmutableGetHelloWorldResults {
	pub(crate) proxy: Proxy,
}

impl ImmutableGetHelloWorldResults {
    pub fn hello_world(&self) -> ScImmutableString {
		ScImmutableString::new(self.proxy.root(RESULT_HELLO_WORLD))
	}
}

#[derive(Clone)]
pub struct MutableGetHelloWorldResults {
	pub(crate) proxy: Proxy,
}

impl MutableGetHelloWorldResults {
    pub fn hello_world(&self) -> ScMutableString {
		ScMutableString::new(self.proxy.root(RESULT_HELLO_WORLD))
	}
}
