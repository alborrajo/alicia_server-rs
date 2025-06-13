use deku::{DekuRead, DekuWrite};

use crate::{
    commands::LengthPrefixedVec,
    entities::{horse::Horse, item::Item},
    impl_command_traits,
    packet::CommandId,
};

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct ShowInventory {}
impl_command_traits!(ShowInventory, CommandId::AcCmdCLShowInventory);

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct ShowInventoryOk {
    pub items: LengthPrefixedVec<1, Item>,
    pub horses: LengthPrefixedVec<1, Horse>,
}
impl_command_traits!(ShowInventoryOk, CommandId::AcCmdCLShowInventoryOK);
