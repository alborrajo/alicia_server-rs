use deku::{DekuRead, DekuWrite};

use crate::{
    commands::{LengthPrefixedVec, shared::quest::Quest},
    impl_command_traits,
    packet::CommandId,
};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct AchievementCompleteList {
    pub character_id: u32,
}
impl_command_traits!(
    AchievementCompleteList,
    CommandId::AcCmdCLAchievementCompleteList
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct AchievementCompleteListOk {
    pub character_id: u32,
    pub achievements: LengthPrefixedVec<2, Quest>,
}
impl_command_traits!(
    AchievementCompleteListOk,
    CommandId::AcCmdCLAchievementCompleteListOK
);
