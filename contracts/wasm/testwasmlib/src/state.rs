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
pub struct ArrayOfImmutableAddressArray {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableAddressArray {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


	pub fn get_address_array(&self, index: u32) -> ImmutableAddressArray {
		ImmutableAddressArray { proxy: self.proxy.index(index) }
	}
}

#[derive(Clone)]
pub struct MapAddressToImmutableAddressArray {
	pub(crate) proxy: Proxy,
}

impl MapAddressToImmutableAddressArray {
    pub fn get_address_array(&self, key: &ScAddress) -> ImmutableAddressArray {
        ImmutableAddressArray { proxy: self.proxy.key(&address_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct ArrayOfImmutableAddressMap {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableAddressMap {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


	pub fn get_address_map(&self, index: u32) -> ImmutableAddressMap {
		ImmutableAddressMap { proxy: self.proxy.index(index) }
	}
}

#[derive(Clone)]
pub struct MapAddressToImmutableAddressMap {
	pub(crate) proxy: Proxy,
}

impl MapAddressToImmutableAddressMap {
    pub fn get_address_map(&self, key: &ScAddress) -> ImmutableAddressMap {
        ImmutableAddressMap { proxy: self.proxy.key(&address_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct MapInt32ToImmutableLongitude {
	pub(crate) proxy: Proxy,
}

impl MapInt32ToImmutableLongitude {
    pub fn get_longitude(&self, key: i32) -> ImmutableLongitude {
        ImmutableLongitude { proxy: self.proxy.key(&int32_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct ArrayOfImmutableStringArray {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableStringArray {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


	pub fn get_string_array(&self, index: u32) -> ImmutableStringArray {
		ImmutableStringArray { proxy: self.proxy.index(index) }
	}
}

#[derive(Clone)]
pub struct ArrayOfImmutableStringMap {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableStringMap {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


	pub fn get_string_map(&self, index: u32) -> ImmutableStringMap {
		ImmutableStringMap { proxy: self.proxy.index(index) }
	}
}

#[derive(Clone)]
pub struct MapStringToImmutableStringArray {
	pub(crate) proxy: Proxy,
}

impl MapStringToImmutableStringArray {
    pub fn get_string_array(&self, key: &str) -> ImmutableStringArray {
        ImmutableStringArray { proxy: self.proxy.key(&string_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct MapStringToImmutableStringMap {
	pub(crate) proxy: Proxy,
}

impl MapStringToImmutableStringMap {
    pub fn get_string_map(&self, key: &str) -> ImmutableStringMap {
        ImmutableStringMap { proxy: self.proxy.key(&string_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct ImmutableTestWasmLibState {
	pub(crate) proxy: Proxy,
}

impl ImmutableTestWasmLibState {
    pub fn address_array_arrays(&self) -> ArrayOfImmutableAddressArray {
		ArrayOfImmutableAddressArray { proxy: self.proxy.root(STATE_ADDRESS_ARRAY_ARRAYS) }
	}

    pub fn address_arrays(&self) -> MapAddressToImmutableAddressArray {
		MapAddressToImmutableAddressArray { proxy: self.proxy.root(STATE_ADDRESS_ARRAYS) }
	}

    pub fn address_map_arrays(&self) -> ArrayOfImmutableAddressMap {
		ArrayOfImmutableAddressMap { proxy: self.proxy.root(STATE_ADDRESS_MAP_ARRAYS) }
	}

    pub fn address_maps(&self) -> MapAddressToImmutableAddressMap {
		MapAddressToImmutableAddressMap { proxy: self.proxy.root(STATE_ADDRESS_MAPS) }
	}

    pub fn lat_long(&self) -> MapInt32ToImmutableLongitude {
		MapInt32ToImmutableLongitude { proxy: self.proxy.root(STATE_LAT_LONG) }
	}

    pub fn random(&self) -> ScImmutableUint64 {
		ScImmutableUint64::new(self.proxy.root(STATE_RANDOM))
	}

    pub fn string_array_of_arrays(&self) -> ArrayOfImmutableStringArray {
		ArrayOfImmutableStringArray { proxy: self.proxy.root(STATE_STRING_ARRAY_OF_ARRAYS) }
	}

    pub fn string_array_of_maps(&self) -> ArrayOfImmutableStringMap {
		ArrayOfImmutableStringMap { proxy: self.proxy.root(STATE_STRING_ARRAY_OF_MAPS) }
	}

    pub fn string_map_of_arrays(&self) -> MapStringToImmutableStringArray {
		MapStringToImmutableStringArray { proxy: self.proxy.root(STATE_STRING_MAP_OF_ARRAYS) }
	}

    pub fn string_map_of_maps(&self) -> MapStringToImmutableStringMap {
		MapStringToImmutableStringMap { proxy: self.proxy.root(STATE_STRING_MAP_OF_MAPS) }
	}
}

#[derive(Clone)]
pub struct ArrayOfMutableAddressArray {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableAddressArray {

	pub fn append_address_array(&self) -> MutableAddressArray {
		MutableAddressArray { proxy: self.proxy.append() }
	}
	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


	pub fn get_address_array(&self, index: u32) -> MutableAddressArray {
		MutableAddressArray { proxy: self.proxy.index(index) }
	}
}

#[derive(Clone)]
pub struct MapAddressToMutableAddressArray {
	pub(crate) proxy: Proxy,
}

impl MapAddressToMutableAddressArray {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_address_array(&self, key: &ScAddress) -> MutableAddressArray {
        MutableAddressArray { proxy: self.proxy.key(&address_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableAddressMap {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableAddressMap {

	pub fn append_address_map(&self) -> MutableAddressMap {
		MutableAddressMap { proxy: self.proxy.append() }
	}
	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


	pub fn get_address_map(&self, index: u32) -> MutableAddressMap {
		MutableAddressMap { proxy: self.proxy.index(index) }
	}
}

#[derive(Clone)]
pub struct MapAddressToMutableAddressMap {
	pub(crate) proxy: Proxy,
}

impl MapAddressToMutableAddressMap {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_address_map(&self, key: &ScAddress) -> MutableAddressMap {
        MutableAddressMap { proxy: self.proxy.key(&address_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct MapInt32ToMutableLongitude {
	pub(crate) proxy: Proxy,
}

impl MapInt32ToMutableLongitude {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_longitude(&self, key: i32) -> MutableLongitude {
        MutableLongitude { proxy: self.proxy.key(&int32_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableStringArray {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableStringArray {

	pub fn append_string_array(&self) -> MutableStringArray {
		MutableStringArray { proxy: self.proxy.append() }
	}
	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


	pub fn get_string_array(&self, index: u32) -> MutableStringArray {
		MutableStringArray { proxy: self.proxy.index(index) }
	}
}

#[derive(Clone)]
pub struct ArrayOfMutableStringMap {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableStringMap {

	pub fn append_string_map(&self) -> MutableStringMap {
		MutableStringMap { proxy: self.proxy.append() }
	}
	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


	pub fn get_string_map(&self, index: u32) -> MutableStringMap {
		MutableStringMap { proxy: self.proxy.index(index) }
	}
}

#[derive(Clone)]
pub struct MapStringToMutableStringArray {
	pub(crate) proxy: Proxy,
}

impl MapStringToMutableStringArray {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_string_array(&self, key: &str) -> MutableStringArray {
        MutableStringArray { proxy: self.proxy.key(&string_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct MapStringToMutableStringMap {
	pub(crate) proxy: Proxy,
}

impl MapStringToMutableStringMap {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_string_map(&self, key: &str) -> MutableStringMap {
        MutableStringMap { proxy: self.proxy.key(&string_to_bytes(key)) }
    }
}

#[derive(Clone)]
pub struct MutableTestWasmLibState {
	pub(crate) proxy: Proxy,
}

impl MutableTestWasmLibState {
    pub fn as_immutable(&self) -> ImmutableTestWasmLibState {
		ImmutableTestWasmLibState { proxy: self.proxy.root("") }
	}

    pub fn address_array_arrays(&self) -> ArrayOfMutableAddressArray {
		ArrayOfMutableAddressArray { proxy: self.proxy.root(STATE_ADDRESS_ARRAY_ARRAYS) }
	}

    pub fn address_arrays(&self) -> MapAddressToMutableAddressArray {
		MapAddressToMutableAddressArray { proxy: self.proxy.root(STATE_ADDRESS_ARRAYS) }
	}

    pub fn address_map_arrays(&self) -> ArrayOfMutableAddressMap {
		ArrayOfMutableAddressMap { proxy: self.proxy.root(STATE_ADDRESS_MAP_ARRAYS) }
	}

    pub fn address_maps(&self) -> MapAddressToMutableAddressMap {
		MapAddressToMutableAddressMap { proxy: self.proxy.root(STATE_ADDRESS_MAPS) }
	}

    pub fn lat_long(&self) -> MapInt32ToMutableLongitude {
		MapInt32ToMutableLongitude { proxy: self.proxy.root(STATE_LAT_LONG) }
	}

    pub fn random(&self) -> ScMutableUint64 {
		ScMutableUint64::new(self.proxy.root(STATE_RANDOM))
	}

    pub fn string_array_of_arrays(&self) -> ArrayOfMutableStringArray {
		ArrayOfMutableStringArray { proxy: self.proxy.root(STATE_STRING_ARRAY_OF_ARRAYS) }
	}

    pub fn string_array_of_maps(&self) -> ArrayOfMutableStringMap {
		ArrayOfMutableStringMap { proxy: self.proxy.root(STATE_STRING_ARRAY_OF_MAPS) }
	}

    pub fn string_map_of_arrays(&self) -> MapStringToMutableStringArray {
		MapStringToMutableStringArray { proxy: self.proxy.root(STATE_STRING_MAP_OF_ARRAYS) }
	}

    pub fn string_map_of_maps(&self) -> MapStringToMutableStringMap {
		MapStringToMutableStringMap { proxy: self.proxy.root(STATE_STRING_MAP_OF_MAPS) }
	}
}
