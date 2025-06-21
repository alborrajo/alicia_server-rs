use deku::{DekuRead, DekuWrite};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct BreedingFailureCard {}
impl_command_traits!(BreedingFailureCard, CommandId::AcCmdCRBreedingFailureCard);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct BreedingFailureCardOk {
    pub unk0: u8,
}
impl_command_traits!(
    BreedingFailureCardOk,
    CommandId::AcCmdCRBreedingFailureCardOK
);
