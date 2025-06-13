use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::{entities::character::Character, impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuWrite, DekuRead)]
pub struct CreateNicknameNotify {}
impl_command_traits!(CreateNicknameNotify, CommandId::AcCmdCLCreateNicknameNotify);

#[derive(Debug, Default, DekuWrite, DekuRead)]
pub struct CreateNicknameCancel {
    pub error: u8,
}
impl_command_traits!(CreateNicknameCancel, CommandId::AcCmdCLCreateNicknameCancel);

#[derive(Debug, Default, DekuWrite, DekuRead)]
pub struct CreateNickname {
    pub nickname: CString,
    pub character: Character,
    pub unk0: u32,
}
impl_command_traits!(CreateNickname, CommandId::AcCmdCLCreateNickname);
