use deku::{DekuRead, DekuWrite};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct GetMessengerInfo {}
impl_command_traits!(GetMessengerInfo, CommandId::AcCmdCLGetMessengerInfo);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct GetMessengerInfoOk {
    pub code: u32,
    pub ip: u32,
    pub port: u16,
}
impl_command_traits!(GetMessengerInfoOk, CommandId::AcCmdCLGetMessengerInfoOK);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct GetMessengerInfoCancel {}
impl_command_traits!(
    GetMessengerInfoCancel,
    CommandId::AcCmdCLGetMessengerInfoCancel
);
