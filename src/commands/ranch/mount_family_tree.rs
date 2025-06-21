use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::{commands::LengthPrefixedVec, impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct MountFamilyTree {
    pub uid: u32,
}
impl_command_traits!(MountFamilyTree, CommandId::AcCmdCRMountFamilyTree);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct MountFamilyTreeCancel {}
impl_command_traits!(
    MountFamilyTreeCancel,
    CommandId::AcCmdCRMountFamilyTreeCancel
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct MountFamilyTreeOk {
    pub uid: u32,
    pub items: LengthPrefixedVec<1, MountFamilyTreeItem>,
}
impl_command_traits!(MountFamilyTreeOk, CommandId::AcCmdCRMountFamilyTreeOK);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct MountFamilyTreeItem {
    pub unk0: u8,
    pub unk1: CString,
    pub unk2: u8,
    pub unk3: u16,
}
