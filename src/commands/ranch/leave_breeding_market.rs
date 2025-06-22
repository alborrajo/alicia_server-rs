use deku::{DekuRead, DekuWrite};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct LeaveBreedingMarket {}
impl_command_traits!(LeaveBreedingMarket, CommandId::AcCmdCRLeaveBreedingMarket);
