use deku::{DekuRead, DekuWrite};

use crate::{
    commands::{LengthPrefixedVec, shared::item::Item},
    impl_command_traits,
    packet::CommandId,
};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestNpcDressList {
    pub ranch_uid: u32,
}
impl_command_traits!(RequestNpcDressList, CommandId::AcCmdCRRequestNpcDressList);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestNpcDressListOk {
    pub ranch_uid: u32,

    // Max size 10
    pub dress_list: LengthPrefixedVec<1, Item>,
}
impl_command_traits!(
    RequestNpcDressListOk,
    CommandId::AcCmdCRRequestNpcDressListOK
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestNpcDressListCancel {}
impl_command_traits!(
    RequestNpcDressListCancel,
    CommandId::AcCmdCRRequestNpcDressListCancel
);
