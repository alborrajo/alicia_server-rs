use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RanchChat {
    pub message: CString,
    pub unk0: u8,
    pub unk1: u8,
}
impl_command_traits!(RanchChat, CommandId::AcCmdCRRanchChat);

#[derive(Clone, Debug, Default, DekuRead, DekuWrite)]
pub struct RanchChatNotify {
    pub author: CString,
    pub message: CString,
    pub is_blue: u8,
    pub unk1: u8,
}
impl_command_traits!(RanchChatNotify, CommandId::AcCmdCRRanchChatNotify);
