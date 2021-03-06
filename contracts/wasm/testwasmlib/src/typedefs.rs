// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]

use wasmlib::*;
use crate::*;

#[derive(Clone)]
pub struct ArrayOfImmutableAddress {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableAddress {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_address(&self, index: u32) -> ScImmutableAddress {
        ScImmutableAddress::new(self.proxy.index(index))
    }
}

pub type ImmutableAddressArray = ArrayOfImmutableAddress;

#[derive(Clone)]
pub struct ArrayOfMutableAddress {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableAddress {
	pub fn append_address(&self) -> ScMutableAddress {
		ScMutableAddress::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_address(&self, index: u32) -> ScMutableAddress {
        ScMutableAddress::new(self.proxy.index(index))
    }
}

pub type MutableAddressArray = ArrayOfMutableAddress;

#[derive(Clone)]
pub struct MapAddressToImmutableAddress {
	pub(crate) proxy: Proxy,
}

impl MapAddressToImmutableAddress {
    pub fn get_address(&self, key: &ScAddress) -> ScImmutableAddress {
        ScImmutableAddress::new(self.proxy.key(&address_to_bytes(key)))
    }
}

pub type ImmutableAddressMap = MapAddressToImmutableAddress;

#[derive(Clone)]
pub struct MapAddressToMutableAddress {
	pub(crate) proxy: Proxy,
}

impl MapAddressToMutableAddress {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_address(&self, key: &ScAddress) -> ScMutableAddress {
        ScMutableAddress::new(self.proxy.key(&address_to_bytes(key)))
    }
}

pub type MutableAddressMap = MapAddressToMutableAddress;

#[derive(Clone)]
pub struct ArrayOfImmutableAgentID {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableAgentID {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_agent_id(&self, index: u32) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.proxy.index(index))
    }
}

pub type ImmutableAgentIDArray = ArrayOfImmutableAgentID;

#[derive(Clone)]
pub struct ArrayOfMutableAgentID {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableAgentID {
	pub fn append_agent_id(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_agent_id(&self, index: u32) -> ScMutableAgentID {
        ScMutableAgentID::new(self.proxy.index(index))
    }
}

pub type MutableAgentIDArray = ArrayOfMutableAgentID;

#[derive(Clone)]
pub struct MapAgentIDToImmutableAgentID {
	pub(crate) proxy: Proxy,
}

impl MapAgentIDToImmutableAgentID {
    pub fn get_agent_id(&self, key: &ScAgentID) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.proxy.key(&agent_id_to_bytes(key)))
    }
}

pub type ImmutableAgentIDMap = MapAgentIDToImmutableAgentID;

#[derive(Clone)]
pub struct MapAgentIDToMutableAgentID {
	pub(crate) proxy: Proxy,
}

impl MapAgentIDToMutableAgentID {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_agent_id(&self, key: &ScAgentID) -> ScMutableAgentID {
        ScMutableAgentID::new(self.proxy.key(&agent_id_to_bytes(key)))
    }
}

pub type MutableAgentIDMap = MapAgentIDToMutableAgentID;

#[derive(Clone)]
pub struct ArrayOfImmutableBytes {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableBytes {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_bytes(&self, index: u32) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.index(index))
    }
}

pub type ImmutableBytesArray = ArrayOfImmutableBytes;

#[derive(Clone)]
pub struct ArrayOfMutableBytes {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableBytes {
	pub fn append_bytes(&self) -> ScMutableBytes {
		ScMutableBytes::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_bytes(&self, index: u32) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.index(index))
    }
}

pub type MutableBytesArray = ArrayOfMutableBytes;

#[derive(Clone)]
pub struct MapBytesToImmutableBytes {
	pub(crate) proxy: Proxy,
}

impl MapBytesToImmutableBytes {
    pub fn get_bytes(&self, key: &[u8]) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.key(&bytes_to_bytes(key)))
    }
}

pub type ImmutableBytesMap = MapBytesToImmutableBytes;

#[derive(Clone)]
pub struct MapBytesToMutableBytes {
	pub(crate) proxy: Proxy,
}

impl MapBytesToMutableBytes {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_bytes(&self, key: &[u8]) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.key(&bytes_to_bytes(key)))
    }
}

pub type MutableBytesMap = MapBytesToMutableBytes;

#[derive(Clone)]
pub struct ArrayOfImmutableChainID {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableChainID {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_chain_id(&self, index: u32) -> ScImmutableChainID {
        ScImmutableChainID::new(self.proxy.index(index))
    }
}

pub type ImmutableChainIDArray = ArrayOfImmutableChainID;

#[derive(Clone)]
pub struct ArrayOfMutableChainID {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableChainID {
	pub fn append_chain_id(&self) -> ScMutableChainID {
		ScMutableChainID::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_chain_id(&self, index: u32) -> ScMutableChainID {
        ScMutableChainID::new(self.proxy.index(index))
    }
}

pub type MutableChainIDArray = ArrayOfMutableChainID;

#[derive(Clone)]
pub struct MapChainIDToImmutableChainID {
	pub(crate) proxy: Proxy,
}

impl MapChainIDToImmutableChainID {
    pub fn get_chain_id(&self, key: &ScChainID) -> ScImmutableChainID {
        ScImmutableChainID::new(self.proxy.key(&chain_id_to_bytes(key)))
    }
}

pub type ImmutableChainIDMap = MapChainIDToImmutableChainID;

#[derive(Clone)]
pub struct MapChainIDToMutableChainID {
	pub(crate) proxy: Proxy,
}

impl MapChainIDToMutableChainID {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_chain_id(&self, key: &ScChainID) -> ScMutableChainID {
        ScMutableChainID::new(self.proxy.key(&chain_id_to_bytes(key)))
    }
}

pub type MutableChainIDMap = MapChainIDToMutableChainID;

#[derive(Clone)]
pub struct ArrayOfImmutableColor {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableColor {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_color(&self, index: u32) -> ScImmutableColor {
        ScImmutableColor::new(self.proxy.index(index))
    }
}

pub type ImmutableColorArray = ArrayOfImmutableColor;

#[derive(Clone)]
pub struct ArrayOfMutableColor {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableColor {
	pub fn append_color(&self) -> ScMutableColor {
		ScMutableColor::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_color(&self, index: u32) -> ScMutableColor {
        ScMutableColor::new(self.proxy.index(index))
    }
}

pub type MutableColorArray = ArrayOfMutableColor;

#[derive(Clone)]
pub struct MapColorToImmutableColor {
	pub(crate) proxy: Proxy,
}

impl MapColorToImmutableColor {
    pub fn get_color(&self, key: &ScColor) -> ScImmutableColor {
        ScImmutableColor::new(self.proxy.key(&color_to_bytes(key)))
    }
}

pub type ImmutableColorMap = MapColorToImmutableColor;

#[derive(Clone)]
pub struct MapColorToMutableColor {
	pub(crate) proxy: Proxy,
}

impl MapColorToMutableColor {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_color(&self, key: &ScColor) -> ScMutableColor {
        ScMutableColor::new(self.proxy.key(&color_to_bytes(key)))
    }
}

pub type MutableColorMap = MapColorToMutableColor;

#[derive(Clone)]
pub struct ArrayOfImmutableHash {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableHash {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_hash(&self, index: u32) -> ScImmutableHash {
        ScImmutableHash::new(self.proxy.index(index))
    }
}

pub type ImmutableHashArray = ArrayOfImmutableHash;

#[derive(Clone)]
pub struct ArrayOfMutableHash {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableHash {
	pub fn append_hash(&self) -> ScMutableHash {
		ScMutableHash::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_hash(&self, index: u32) -> ScMutableHash {
        ScMutableHash::new(self.proxy.index(index))
    }
}

pub type MutableHashArray = ArrayOfMutableHash;

#[derive(Clone)]
pub struct MapHashToImmutableHash {
	pub(crate) proxy: Proxy,
}

impl MapHashToImmutableHash {
    pub fn get_hash(&self, key: &ScHash) -> ScImmutableHash {
        ScImmutableHash::new(self.proxy.key(&hash_to_bytes(key)))
    }
}

pub type ImmutableHashMap = MapHashToImmutableHash;

#[derive(Clone)]
pub struct MapHashToMutableHash {
	pub(crate) proxy: Proxy,
}

impl MapHashToMutableHash {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_hash(&self, key: &ScHash) -> ScMutableHash {
        ScMutableHash::new(self.proxy.key(&hash_to_bytes(key)))
    }
}

pub type MutableHashMap = MapHashToMutableHash;

#[derive(Clone)]
pub struct ArrayOfImmutableHname {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableHname {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_hname(&self, index: u32) -> ScImmutableHname {
        ScImmutableHname::new(self.proxy.index(index))
    }
}

pub type ImmutableHnameArray = ArrayOfImmutableHname;

#[derive(Clone)]
pub struct ArrayOfMutableHname {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableHname {
	pub fn append_hname(&self) -> ScMutableHname {
		ScMutableHname::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_hname(&self, index: u32) -> ScMutableHname {
        ScMutableHname::new(self.proxy.index(index))
    }
}

pub type MutableHnameArray = ArrayOfMutableHname;

#[derive(Clone)]
pub struct MapHnameToImmutableHname {
	pub(crate) proxy: Proxy,
}

impl MapHnameToImmutableHname {
    pub fn get_hname(&self, key: ScHname) -> ScImmutableHname {
        ScImmutableHname::new(self.proxy.key(&hname_to_bytes(key)))
    }
}

pub type ImmutableHnameMap = MapHnameToImmutableHname;

#[derive(Clone)]
pub struct MapHnameToMutableHname {
	pub(crate) proxy: Proxy,
}

impl MapHnameToMutableHname {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_hname(&self, key: ScHname) -> ScMutableHname {
        ScMutableHname::new(self.proxy.key(&hname_to_bytes(key)))
    }
}

pub type MutableHnameMap = MapHnameToMutableHname;

#[derive(Clone)]
pub struct ArrayOfImmutableInt32 {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableInt32 {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_int32(&self, index: u32) -> ScImmutableInt32 {
        ScImmutableInt32::new(self.proxy.index(index))
    }
}

pub type ImmutableInt32Array = ArrayOfImmutableInt32;

#[derive(Clone)]
pub struct ArrayOfMutableInt32 {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableInt32 {
	pub fn append_int32(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_int32(&self, index: u32) -> ScMutableInt32 {
        ScMutableInt32::new(self.proxy.index(index))
    }
}

pub type MutableInt32Array = ArrayOfMutableInt32;

#[derive(Clone)]
pub struct MapInt32ToImmutableInt32 {
	pub(crate) proxy: Proxy,
}

impl MapInt32ToImmutableInt32 {
    pub fn get_int32(&self, key: i32) -> ScImmutableInt32 {
        ScImmutableInt32::new(self.proxy.key(&int32_to_bytes(key)))
    }
}

pub type ImmutableInt32Map = MapInt32ToImmutableInt32;

#[derive(Clone)]
pub struct MapInt32ToMutableInt32 {
	pub(crate) proxy: Proxy,
}

impl MapInt32ToMutableInt32 {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_int32(&self, key: i32) -> ScMutableInt32 {
        ScMutableInt32::new(self.proxy.key(&int32_to_bytes(key)))
    }
}

pub type MutableInt32Map = MapInt32ToMutableInt32;

#[derive(Clone)]
pub struct ArrayOfImmutableInt64 {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableInt64 {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_int64(&self, index: u32) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.proxy.index(index))
    }
}

pub type ImmutableInt64Array = ArrayOfImmutableInt64;

#[derive(Clone)]
pub struct ArrayOfMutableInt64 {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableInt64 {
	pub fn append_int64(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_int64(&self, index: u32) -> ScMutableInt64 {
        ScMutableInt64::new(self.proxy.index(index))
    }
}

pub type MutableInt64Array = ArrayOfMutableInt64;

#[derive(Clone)]
pub struct MapInt64ToImmutableInt64 {
	pub(crate) proxy: Proxy,
}

impl MapInt64ToImmutableInt64 {
    pub fn get_int64(&self, key: i64) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.proxy.key(&int64_to_bytes(key)))
    }
}

pub type ImmutableInt64Map = MapInt64ToImmutableInt64;

#[derive(Clone)]
pub struct MapInt64ToMutableInt64 {
	pub(crate) proxy: Proxy,
}

impl MapInt64ToMutableInt64 {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_int64(&self, key: i64) -> ScMutableInt64 {
        ScMutableInt64::new(self.proxy.key(&int64_to_bytes(key)))
    }
}

pub type MutableInt64Map = MapInt64ToMutableInt64;

#[derive(Clone)]
pub struct MapInt32ToImmutableLocation {
	pub(crate) proxy: Proxy,
}

impl MapInt32ToImmutableLocation {
    pub fn get_location(&self, key: i32) -> ImmutableLocation {
        ImmutableLocation { proxy: self.proxy.key(&int32_to_bytes(key)) }
    }
}

pub type ImmutableLongitude = MapInt32ToImmutableLocation;

#[derive(Clone)]
pub struct MapInt32ToMutableLocation {
	pub(crate) proxy: Proxy,
}

impl MapInt32ToMutableLocation {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_location(&self, key: i32) -> MutableLocation {
        MutableLocation { proxy: self.proxy.key(&int32_to_bytes(key)) }
    }
}

pub type MutableLongitude = MapInt32ToMutableLocation;

#[derive(Clone)]
pub struct ArrayOfImmutableRequestID {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableRequestID {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_request_id(&self, index: u32) -> ScImmutableRequestID {
        ScImmutableRequestID::new(self.proxy.index(index))
    }
}

pub type ImmutableRequestIDArray = ArrayOfImmutableRequestID;

#[derive(Clone)]
pub struct ArrayOfMutableRequestID {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableRequestID {
	pub fn append_request_id(&self) -> ScMutableRequestID {
		ScMutableRequestID::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_request_id(&self, index: u32) -> ScMutableRequestID {
        ScMutableRequestID::new(self.proxy.index(index))
    }
}

pub type MutableRequestIDArray = ArrayOfMutableRequestID;

#[derive(Clone)]
pub struct MapRequestIDToImmutableRequestID {
	pub(crate) proxy: Proxy,
}

impl MapRequestIDToImmutableRequestID {
    pub fn get_request_id(&self, key: &ScRequestID) -> ScImmutableRequestID {
        ScImmutableRequestID::new(self.proxy.key(&request_id_to_bytes(key)))
    }
}

pub type ImmutableRequestIDMap = MapRequestIDToImmutableRequestID;

#[derive(Clone)]
pub struct MapRequestIDToMutableRequestID {
	pub(crate) proxy: Proxy,
}

impl MapRequestIDToMutableRequestID {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_request_id(&self, key: &ScRequestID) -> ScMutableRequestID {
        ScMutableRequestID::new(self.proxy.key(&request_id_to_bytes(key)))
    }
}

pub type MutableRequestIDMap = MapRequestIDToMutableRequestID;

#[derive(Clone)]
pub struct ArrayOfImmutableString {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableString {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_string(&self, index: u32) -> ScImmutableString {
        ScImmutableString::new(self.proxy.index(index))
    }
}

pub type ImmutableStringArray = ArrayOfImmutableString;

#[derive(Clone)]
pub struct ArrayOfMutableString {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableString {
	pub fn append_string(&self) -> ScMutableString {
		ScMutableString::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_string(&self, index: u32) -> ScMutableString {
        ScMutableString::new(self.proxy.index(index))
    }
}

pub type MutableStringArray = ArrayOfMutableString;

#[derive(Clone)]
pub struct MapStringToImmutableString {
	pub(crate) proxy: Proxy,
}

impl MapStringToImmutableString {
    pub fn get_string(&self, key: &str) -> ScImmutableString {
        ScImmutableString::new(self.proxy.key(&string_to_bytes(key)))
    }
}

pub type ImmutableStringMap = MapStringToImmutableString;

#[derive(Clone)]
pub struct MapStringToMutableString {
	pub(crate) proxy: Proxy,
}

impl MapStringToMutableString {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_string(&self, key: &str) -> ScMutableString {
        ScMutableString::new(self.proxy.key(&string_to_bytes(key)))
    }
}

pub type MutableStringMap = MapStringToMutableString;

#[derive(Clone)]
pub struct ArrayOfImmutableUint64 {
	pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableUint64 {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_uint64(&self, index: u32) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.index(index))
    }
}

pub type ImmutableUint64Array = ArrayOfImmutableUint64;

#[derive(Clone)]
pub struct ArrayOfMutableUint64 {
	pub(crate) proxy: Proxy,
}

impl ArrayOfMutableUint64 {
	pub fn append_uint64(&self) -> ScMutableUint64 {
		ScMutableUint64::new(self.proxy.append())
	}

	pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_uint64(&self, index: u32) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.index(index))
    }
}

pub type MutableUint64Array = ArrayOfMutableUint64;

#[derive(Clone)]
pub struct MapUint64ToImmutableUint64 {
	pub(crate) proxy: Proxy,
}

impl MapUint64ToImmutableUint64 {
    pub fn get_uint64(&self, key: u64) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.key(&uint64_to_bytes(key)))
    }
}

pub type ImmutableUint64Map = MapUint64ToImmutableUint64;

#[derive(Clone)]
pub struct MapUint64ToMutableUint64 {
	pub(crate) proxy: Proxy,
}

impl MapUint64ToMutableUint64 {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_uint64(&self, key: u64) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.key(&uint64_to_bytes(key)))
    }
}

pub type MutableUint64Map = MapUint64ToMutableUint64;
