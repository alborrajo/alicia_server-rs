use crate::{commands::LengthPrefixedVec, impl_command_traits, packet::CommandId};
use deku::{DekuRead, DekuWrite};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterBreedingMarket;
impl_command_traits!(EnterBreedingMarket, CommandId::AcCmdCREnterBreedingMarket);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterBreedingMarketCancel;
impl_command_traits!(
    EnterBreedingMarketCancel,
    CommandId::AcCmdCREnterBreedingMarketCancel
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct EnterBreedingMarketOk {
    pub available_horses: LengthPrefixedVec<1, AvailableHorse>,
}
impl_command_traits!(
    EnterBreedingMarketOk,
    CommandId::AcCmdCREnterBreedingMarketOK
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct AvailableHorse {
    pub uid: u32,
    pub tid: u32,
    pub success: u8,
    pub unk1: u32,
    pub unk2: u8,
    pub coat_bonus: u8,
}
