use deku::{DekuRead, DekuWrite};

use crate::{commands::RestOfBuffer, impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RanchCmdAction {
    unk0: u16,
    snapshot: RestOfBuffer,
}
impl_command_traits!(RanchCmdAction, CommandId::AcCmdCRRanchCmdAction);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RanchCmdActionNotify {
    unk0: u16,
    unk1: u16,
    unk2: u8,
}
impl_command_traits!(RanchCmdActionNotify, CommandId::AcCmdCRRanchCmdActionNotify);
