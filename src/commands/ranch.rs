use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::commands::{
    LengthPrefixedVec,
    shared::{
        character::{
            AnotherPlayerRelatedThing, Character, Gender, PlayerRelatedThing,
            YetAnotherPlayerRelatedThing,
        },
        horse::Horse,
        item::Item,
    },
};

pub mod breeding_failure_card;
pub mod breeding_wishlist;
pub mod enter_breeding_market;
pub mod enter_ranch;
pub mod leave_breeding_market;
pub mod mount_family_tree;
pub mod ranch_chat;
pub mod ranch_cmd_action;
pub mod ranch_snapshot;
pub mod request_npc_dress_list;
pub mod request_storage;
pub mod search_stallion;
pub mod try_breeding;
pub mod update_mount_nickname;
pub mod wear_equipment;

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RanchHorse {
    pub ranch_index: u16,
    pub horse: Horse,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct RanchCharacter {
    pub uid: u32,
    pub name: CString,
    pub gender: Gender,
    pub unk0: u8,
    pub unk1: u8,
    pub description: CString,

    pub character: Character,
    pub mount: Horse,
    pub character_equipment: LengthPrefixedVec<1, Item>,

    pub player_related_thing: PlayerRelatedThing,

    pub ranch_index: u16,
    pub unk2: u8,
    pub unk3: u8,

    pub another_player_related_thing: AnotherPlayerRelatedThing,
    pub yet_another_player_related_thing: YetAnotherPlayerRelatedThing,

    pub unk4: u8,
    pub unk5: u8,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RanchUnk11 {
    pub unk0: u8,
    pub unk1: u8,
}
