// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use crate::coreblob::*;
use crate::host::*;

pub struct MapStringToImmutableBytes {
    pub(crate) obj_id: i32,
}

impl MapStringToImmutableBytes {
    pub fn get_bytes(&self, key: &str) -> ScImmutableBytes {
        ScImmutableBytes::new(self.obj_id, key.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableStoreBlobParams {
    pub(crate) id: i32,
}

impl ImmutableStoreBlobParams {
    pub fn blobs(&self) -> MapStringToImmutableBytes {
        MapStringToImmutableBytes { obj_id: self.id }
    }
}

pub struct MapStringToMutableBytes {
    pub(crate) obj_id: i32,
}

impl MapStringToMutableBytes {
    pub fn clear(&self) {
        clear(self.obj_id)
    }

    pub fn get_bytes(&self, key: &str) -> ScMutableBytes {
        ScMutableBytes::new(self.obj_id, key.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableStoreBlobParams {
    pub(crate) id: i32,
}

impl MutableStoreBlobParams {
    pub fn blobs(&self) -> MapStringToMutableBytes {
        MapStringToMutableBytes { obj_id: self.id }
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableGetBlobFieldParams {
    pub(crate) id: i32,
}

impl ImmutableGetBlobFieldParams {
    pub fn field(&self) -> ScImmutableString {
        ScImmutableString::new(self.id, PARAM_FIELD.get_key_id())
    }

    pub fn hash(&self) -> ScImmutableHash {
        ScImmutableHash::new(self.id, PARAM_HASH.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableGetBlobFieldParams {
    pub(crate) id: i32,
}

impl MutableGetBlobFieldParams {
    pub fn field(&self) -> ScMutableString {
        ScMutableString::new(self.id, PARAM_FIELD.get_key_id())
    }

    pub fn hash(&self) -> ScMutableHash {
        ScMutableHash::new(self.id, PARAM_HASH.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableGetBlobInfoParams {
    pub(crate) id: i32,
}

impl ImmutableGetBlobInfoParams {
    pub fn hash(&self) -> ScImmutableHash {
        ScImmutableHash::new(self.id, PARAM_HASH.get_key_id())
    }
}

#[derive(Clone, Copy)]
pub struct MutableGetBlobInfoParams {
    pub(crate) id: i32,
}

impl MutableGetBlobInfoParams {
    pub fn hash(&self) -> ScMutableHash {
        ScMutableHash::new(self.id, PARAM_HASH.get_key_id())
    }
}
