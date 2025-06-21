use crate::{
    commands::shared::horse::{Appearance, Parts, Stats},
    impl_command_traits,
    packet::CommandId,
};
use deku::{DekuRead, DekuWrite};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct TryBreeding {
    pub own_horse_uid: u32,
    pub other_horse_uid: u32,
}
impl_command_traits!(TryBreeding, CommandId::AcCmdCRTryBreeding);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct TryBreedingOk {
    pub uid: u32,
    pub tid: u32,
    pub val: u32,
    pub count: u32,
    pub unk0: u8,
    pub parts: Parts,
    pub appearance: Appearance,
    pub stats: Stats,
    pub unk1: u32,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: u8,
    pub unk8: u8,
    pub unk9: u16,
    pub unk10: u8,
}
impl_command_traits!(TryBreedingOk, CommandId::AcCmdCRTryBreedingOK);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct TryBreedingCancel {
    pub unk0: u8,
    pub unk1: u32,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
}
impl_command_traits!(TryBreedingCancel, CommandId::AcCmdCRTryBreedingCancel);
