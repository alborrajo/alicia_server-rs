use deku::{DekuRead, DekuWrite};

use crate::{
    commands::{LengthPrefixedVec, shared::quest::Quest},
    impl_command_traits,
    packet::CommandId,
};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestQuestList {
    pub character_id: u32,
}
impl_command_traits!(RequestQuestList, CommandId::AcCmdCLRequestQuestList);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestQuestListOk {
    pub character_id: u32,
    pub quests: LengthPrefixedVec<2, Quest>,
}
impl_command_traits!(RequestQuestListOk, CommandId::AcCmdCLRequestQuestListOK);
