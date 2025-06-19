use deku::{DekuRead, DekuWrite};

use crate::{
    commands::{LengthPrefixedVec, shared::quest::Quest},
    impl_command_traits,
    packet::CommandId,
};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestDailyQuestList {
    pub character_id: u32,
}
impl_command_traits!(
    RequestDailyQuestList,
    CommandId::AcCmdCLRequestDailyQuestList
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestDailyQuestListOk {
    pub character_id: u32,
    pub quests: LengthPrefixedVec<2, Quest>,
    pub val1: LengthPrefixedVec<2, Unk>,
}
impl_command_traits!(
    RequestDailyQuestListOk,
    CommandId::AcCmdCLRequestDailyQuestListOK
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Unk {
    pub val0: u16,
    pub val1: u32,
    pub val2: u8,
    pub val3: u8,
}
