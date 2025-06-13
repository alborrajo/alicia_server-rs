use deku::{DekuRead, DekuWrite};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct RequestLeagueInfo {}
impl_command_traits!(RequestLeagueInfo, CommandId::AcCmdCLRequestLeagueInfo);

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct RequestLeagueInfoCancel {}
impl_command_traits!(
    RequestLeagueInfoCancel,
    CommandId::AcCmdCLRequestLeagueInfoCancel
);

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct RequestLeagueInfoOk {
    pub unk0: u8,
    pub unk1: u8,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u8,
    pub unk9: u8,
    pub unk10: u32,
    pub unk11: u8,
    pub unk12: u8,
    pub unk13: u8,
}
impl_command_traits!(RequestLeagueInfoOk, CommandId::AcCmdCLRequestLeagueInfoOK);
