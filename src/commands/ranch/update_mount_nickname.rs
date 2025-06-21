use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct UpdateMountNickname {
    pub uid: u32,
    pub nickname: CString,
    pub unk1: u32,
}
impl_command_traits!(UpdateMountNickname, CommandId::AcCmdCRUpdateMountNickname);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct UpdateMountNicknameCancel {
    pub unk0: u8,
}
impl_command_traits!(
    UpdateMountNicknameCancel,
    CommandId::AcCmdCRUpdateMountNicknameCancel
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct UpdateMountNicknameOk {
    pub uid: u32,
    pub nickname: CString,
    pub unk1: u32,
    pub unk2: u32,
}
impl_command_traits!(
    UpdateMountNicknameOk,
    CommandId::AcCmdCRUpdateMountNicknameOK
);
