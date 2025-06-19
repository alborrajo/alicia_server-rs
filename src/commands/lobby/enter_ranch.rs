use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterRanch {
    pub character_id: u32,
    pub unk1: CString,
    pub unk2: u8,
}
impl_command_traits!(EnterRanch, CommandId::AcCmdCLEnterRanch);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterRanchOk {
    pub ranch_uid: u32,
    pub code: u32,
    pub ip: u32,
    pub port: u16,
}
impl_command_traits!(EnterRanchOk, CommandId::AcCmdCLEnterRanchOK);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterRanchCancel {
    pub unk0: u16,
}
impl_command_traits!(EnterRanchCancel, CommandId::AcCmdCLEnterRanchCancel);
