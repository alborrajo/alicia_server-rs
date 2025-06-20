use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::{
    commands::{
        LengthPrefixedVec,
        ranch::{RanchCharacter, RanchHorse, RanchUnk11},
    },
    impl_command_traits,
    packet::CommandId,
};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterRanch {
    pub character_uid: u32,
    pub otp: u32,
    pub ranch_uid: u32,
}
impl_command_traits!(EnterRanch, CommandId::AcCmdCREnterRanch);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterRanchCancel {}
impl_command_traits!(EnterRanchCancel, CommandId::AcCmdCREnterRanchCancel);

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct EnterRanchNotify {
    pub character: RanchCharacter,
}
impl_command_traits!(EnterRanchNotify, CommandId::AcCmdCREnterRanchNotify);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterRanchOk {
    pub ranch_id: u32,
    pub unk0: CString,
    pub ranch_name: CString,

    // Indexes across both lists cant be shared.
    // If the horse list takes indexes 0, 1 and 2
    // the player list must use indexes 3, 4 and 5.
    pub horses: LengthPrefixedVec<1, RanchHorse>,
    pub character: LengthPrefixedVec<1, RanchCharacter>,

    pub unk1: u64,
    pub unk2: u32,
    pub unk3: u32,

    pub unk4: LengthPrefixedVec<1, Unk4>,

    pub unk5: u8,
    pub unk6: u32,
    pub unk7: u32, // bitset

    pub unk8: u32,
    pub unk9: u32,

    pub unk10: [Unk10; 3],

    pub unk11: RanchUnk11,

    pub unk12: u32,
}
impl_command_traits!(EnterRanchOk, CommandId::AcCmdCREnterRanchOK);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Unk4 {
    pub unk0: u32,
    pub unk1: u16,
    pub unk2: u32,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Unk10 {
    pub horse_tid: u32,
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u8,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
}
