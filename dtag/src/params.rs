// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use wasmlib::host::*;

use crate::*;
use crate::keys::*;

#[derive(Clone, Copy)]
pub struct ImmutableAssignImageParams {
    pub(crate) id: i32,
}

impl ImmutableAssignImageParams {
    pub fn image_id(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, idx_map(IDX_PARAM_IMAGE_ID))
    }
}

#[derive(Clone, Copy)]
pub struct MutableAssignImageParams {
    pub(crate) id: i32,
}

impl MutableAssignImageParams {
    pub fn image_id(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, idx_map(IDX_PARAM_IMAGE_ID))
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableCreateGameParams {
    pub(crate) id: i32,
}

impl ImmutableCreateGameParams {
    pub fn description(&self) -> ScImmutableString {
        ScImmutableString::new(self.id, idx_map(IDX_PARAM_DESCRIPTION))
    }

    pub fn number_of_images(&self) -> ScImmutableInt64 {
        ScImmutableInt64::new(self.id, idx_map(IDX_PARAM_NUMBER_OF_IMAGES))
    }
}

#[derive(Clone, Copy)]
pub struct MutableCreateGameParams {
    pub(crate) id: i32,
}

impl MutableCreateGameParams {
    pub fn description(&self) -> ScMutableString {
        ScMutableString::new(self.id, idx_map(IDX_PARAM_DESCRIPTION))
    }

    pub fn number_of_images(&self) -> ScMutableInt64 {
        ScMutableInt64::new(self.id, idx_map(IDX_PARAM_NUMBER_OF_IMAGES))
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableInitParams {
    pub(crate) id: i32,
}

impl ImmutableInitParams {
    pub fn owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.id, idx_map(IDX_PARAM_OWNER))
    }
}

#[derive(Clone, Copy)]
pub struct MutableInitParams {
    pub(crate) id: i32,
}

impl MutableInitParams {
    pub fn owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.id, idx_map(IDX_PARAM_OWNER))
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableSendTagParams {
    pub(crate) id: i32,
}

impl ImmutableSendTagParams {
    pub fn tag(&self) -> ImmutableTag {
        ImmutableTag { obj_id: self.id, key_id: idx_map(IDX_PARAM_TAG) }
    }
}

#[derive(Clone, Copy)]
pub struct MutableSendTagParams {
    pub(crate) id: i32,
}

impl MutableSendTagParams {
    pub fn tag(&self) -> MutableTag {
        MutableTag { obj_id: self.id, key_id: idx_map(IDX_PARAM_TAG) }
    }
}
