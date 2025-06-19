use deku::{DekuRead, DekuWrite};

use crate::{
    commands::{LengthPrefixedVec, shared::quest::Quest},
    impl_command_traits,
    packet::CommandId,
};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestSpecialEventList {
    pub unk0: u32,
}
impl_command_traits!(
    RequestSpecialEventList,
    CommandId::AcCmdCLRequestSpecialEventList
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestSpecialEventListOk {
    pub unk0: u32,
    pub quests: LengthPrefixedVec<2, Quest>,
    pub events: LengthPrefixedVec<2, Event>,
}
impl_command_traits!(
    RequestSpecialEventListOk,
    CommandId::AcCmdCLRequestSpecialEventListOK
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Event {
    pub unk0: u16,
    pub unk1: u32,
}
