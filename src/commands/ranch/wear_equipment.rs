use deku::{DekuRead, DekuWrite};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct WearEquipment {
    pub item_uid: u32,
    pub member: u8,
}
impl_command_traits!(WearEquipment, CommandId::AcCmdCRWearEquipment);

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct WearEquipmentCancel {
    pub unk0: u32,
    pub unk1: u8,
}
impl_command_traits!(WearEquipmentCancel, CommandId::AcCmdCRWearEquipmentCancel);

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct WearEquipmentOk {
    pub item_uid: u32,
    pub member: u8,
}
impl_command_traits!(WearEquipmentOk, CommandId::AcCmdCRWearEquipmentOK);
