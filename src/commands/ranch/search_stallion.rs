use std::ffi::CString;

use crate::{
    commands::{
        LengthPrefixedVec,
        shared::horse::{Appearance, Parts, Stats},
    },
    impl_command_traits,
    packet::CommandId,
};
use deku::{DekuRead, DekuWrite};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct SearchStallion {
    pub unk0: u32,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: u8,
    pub unk8: u8,
    pub unk9: [LengthPrefixedVec<1, u32>; 3],
    pub unk10: u8,
}
impl_command_traits!(SearchStallion, CommandId::AcCmdCRSearchStallion);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct SearchStallionCancel;
impl_command_traits!(SearchStallionCancel, CommandId::AcCmdCRSearchStallionCancel);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct SearchStallionOk {
    pub unk0: u32,
    pub unk1: u32,
    pub stallions: LengthPrefixedVec<1, Stallion>,
}
impl_command_traits!(SearchStallionOk, CommandId::AcCmdCRSearchStallionOK);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Stallion {
    pub unk0: CString,
    pub uid: u32,
    pub tid: u32,
    pub name: CString,
    pub grade: u8,
    pub chance: u8,
    pub price: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub stats: Stats,
    pub parts: Parts,
    pub appearance: Appearance,
    pub unk11: u8,
    pub coat_bonus: u8,
}
