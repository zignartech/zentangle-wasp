// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use wasmlib::host::*;

#[derive(Clone)]
pub struct Bet {
    pub amount   : i64, 
    pub image_id : i32, 
    pub player   : ScAgentID,  // player placing the bet
}

impl Bet {
    pub fn from_bytes(bytes: &[u8]) -> Bet {
        let mut decode = BytesDecoder::new(bytes);
        Bet {
            amount   : decode.int64(),
            image_id : decode.int32(),
            player   : decode.agent_id(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut encode = BytesEncoder::new();
		encode.int64(self.amount);
		encode.int32(self.image_id);
		encode.agent_id(&self.player);
        return encode.data();
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableBet {
    pub(crate) obj_id: i32,
    pub(crate) key_id: Key32,
}

impl ImmutableBet {
    pub fn exists(&self) -> bool {
        exists(self.obj_id, self.key_id, TYPE_BYTES)
    }

    pub fn value(&self) -> Bet {
        Bet::from_bytes(&get_bytes(self.obj_id, self.key_id, TYPE_BYTES))
    }
}

#[derive(Clone, Copy)]
pub struct MutableBet {
    pub(crate) obj_id: i32,
    pub(crate) key_id: Key32,
}

impl MutableBet {
    pub fn delete(&self) {
        del_key(self.obj_id, self.key_id, TYPE_BYTES);
    }

    pub fn exists(&self) -> bool {
        exists(self.obj_id, self.key_id, TYPE_BYTES)
    }

    pub fn set_value(&self, value: &Bet) {
        set_bytes(self.obj_id, self.key_id, TYPE_BYTES, &value.to_bytes());
    }

    pub fn value(&self) -> Bet {
        Bet::from_bytes(&get_bytes(self.obj_id, self.key_id, TYPE_BYTES))
    }
}

#[derive(Clone)]
pub struct Player {
    pub n_double_boosts  : i64,  // Number of 2x boost used in the round
    pub n_tags           : i64,  // Number of tags made by the player in the current round
    pub n_tripple_boosts : i64,  // Number of 3x boosts used in the round
    pub player_id        : ScAgentID,  // The player
}

impl Player {
    pub fn from_bytes(bytes: &[u8]) -> Player {
        let mut decode = BytesDecoder::new(bytes);
        Player {
            n_double_boosts  : decode.int64(),
            n_tags           : decode.int64(),
            n_tripple_boosts : decode.int64(),
            player_id        : decode.agent_id(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut encode = BytesEncoder::new();
		encode.int64(self.n_double_boosts);
		encode.int64(self.n_tags);
		encode.int64(self.n_tripple_boosts);
		encode.agent_id(&self.player_id);
        return encode.data();
    }
}

#[derive(Clone, Copy)]
pub struct ImmutablePlayer {
    pub(crate) obj_id: i32,
    pub(crate) key_id: Key32,
}

impl ImmutablePlayer {
    pub fn exists(&self) -> bool {
        exists(self.obj_id, self.key_id, TYPE_BYTES)
    }

    pub fn value(&self) -> Player {
        Player::from_bytes(&get_bytes(self.obj_id, self.key_id, TYPE_BYTES))
    }
}

#[derive(Clone, Copy)]
pub struct MutablePlayer {
    pub(crate) obj_id: i32,
    pub(crate) key_id: Key32,
}

impl MutablePlayer {
    pub fn delete(&self) {
        del_key(self.obj_id, self.key_id, TYPE_BYTES);
    }

    pub fn exists(&self) -> bool {
        exists(self.obj_id, self.key_id, TYPE_BYTES)
    }

    pub fn set_value(&self, value: &Player) {
        set_bytes(self.obj_id, self.key_id, TYPE_BYTES, &value.to_bytes());
    }

    pub fn value(&self) -> Player {
        Player::from_bytes(&get_bytes(self.obj_id, self.key_id, TYPE_BYTES))
    }
}

#[derive(Clone)]
pub struct TaggedImage {
    pub boost    : String,  // if the tags will be boosted or not
    pub h        : String,  // heights of the Tags
    pub image_id : i32, 
    pub player   : ScAgentID,  // player that has tagged this image
    pub w        : String,  // widths of the Tags
    pub x        : String,  // x top-left positions of the Tags
    pub y        : String,  // y top-left positions of the Tags
}

impl TaggedImage {
    pub fn from_bytes(bytes: &[u8]) -> TaggedImage {
        let mut decode = BytesDecoder::new(bytes);
        TaggedImage {
            boost    : decode.string(),
            h        : decode.string(),
            image_id : decode.int32(),
            player   : decode.agent_id(),
            w        : decode.string(),
            x        : decode.string(),
            y        : decode.string(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut encode = BytesEncoder::new();
		encode.string(&self.boost);
		encode.string(&self.h);
		encode.int32(self.image_id);
		encode.agent_id(&self.player);
		encode.string(&self.w);
		encode.string(&self.x);
		encode.string(&self.y);
        return encode.data();
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableTaggedImage {
    pub(crate) obj_id: i32,
    pub(crate) key_id: Key32,
}

impl ImmutableTaggedImage {
    pub fn exists(&self) -> bool {
        exists(self.obj_id, self.key_id, TYPE_BYTES)
    }

    pub fn value(&self) -> TaggedImage {
        TaggedImage::from_bytes(&get_bytes(self.obj_id, self.key_id, TYPE_BYTES))
    }
}

#[derive(Clone, Copy)]
pub struct MutableTaggedImage {
    pub(crate) obj_id: i32,
    pub(crate) key_id: Key32,
}

impl MutableTaggedImage {
    pub fn delete(&self) {
        del_key(self.obj_id, self.key_id, TYPE_BYTES);
    }

    pub fn exists(&self) -> bool {
        exists(self.obj_id, self.key_id, TYPE_BYTES)
    }

    pub fn set_value(&self, value: &TaggedImage) {
        set_bytes(self.obj_id, self.key_id, TYPE_BYTES, &value.to_bytes());
    }

    pub fn value(&self) -> TaggedImage {
        TaggedImage::from_bytes(&get_bytes(self.obj_id, self.key_id, TYPE_BYTES))
    }
}

#[derive(Clone)]
pub struct ValidTag {
    pub play_tag_id  : i32,  // Identifier to distinguish different tags in the same play
    pub player       : ScAgentID,  // player placing the bet
    pub tagged_image : i32, 
}

impl ValidTag {
    pub fn from_bytes(bytes: &[u8]) -> ValidTag {
        let mut decode = BytesDecoder::new(bytes);
        ValidTag {
            play_tag_id  : decode.int32(),
            player       : decode.agent_id(),
            tagged_image : decode.int32(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut encode = BytesEncoder::new();
		encode.int32(self.play_tag_id);
		encode.agent_id(&self.player);
		encode.int32(self.tagged_image);
        return encode.data();
    }
}

#[derive(Clone, Copy)]
pub struct ImmutableValidTag {
    pub(crate) obj_id: i32,
    pub(crate) key_id: Key32,
}

impl ImmutableValidTag {
    pub fn exists(&self) -> bool {
        exists(self.obj_id, self.key_id, TYPE_BYTES)
    }

    pub fn value(&self) -> ValidTag {
        ValidTag::from_bytes(&get_bytes(self.obj_id, self.key_id, TYPE_BYTES))
    }
}

#[derive(Clone, Copy)]
pub struct MutableValidTag {
    pub(crate) obj_id: i32,
    pub(crate) key_id: Key32,
}

impl MutableValidTag {
    pub fn delete(&self) {
        del_key(self.obj_id, self.key_id, TYPE_BYTES);
    }

    pub fn exists(&self) -> bool {
        exists(self.obj_id, self.key_id, TYPE_BYTES)
    }

    pub fn set_value(&self, value: &ValidTag) {
        set_bytes(self.obj_id, self.key_id, TYPE_BYTES, &value.to_bytes());
    }

    pub fn value(&self) -> ValidTag {
        ValidTag::from_bytes(&get_bytes(self.obj_id, self.key_id, TYPE_BYTES))
    }
}
