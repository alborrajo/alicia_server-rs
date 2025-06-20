use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::{commands::LengthPrefixedVec, impl_command_traits, packet::CommandId};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestStorage {
    pub val0: u8,
    pub val1: u16,
}
impl_command_traits!(RequestStorage, CommandId::AcCmdCRRequestStorage);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestStorageOk {
    pub val0: u8,
    pub val1: u16,
    pub val2: u16,
    /// Max 33 elements.
    pub val3: LengthPrefixedVec<1, Unk>,
}
impl_command_traits!(RequestStorageOk, CommandId::AcCmdCRRequestStorageOK);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct RequestStorageCancel {
    pub val0: u8,
    pub val1: u8,
}
impl_command_traits!(RequestStorageCancel, CommandId::AcCmdCRRequestStorageCancel);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Unk {
    pub uid: u32,
    pub val1: u32,
    pub val2: u8,
    pub val3: u32,
    pub val4: u32,
    pub val5: u32,
    pub val6: u32,
    pub sender: CString,
    pub message: CString,
    /// [0000'00][00'0000]'[0000'0000]'[0000]'[0000'0000'0000]
    /// [minute] [hour] [day] [month] [year]
    pub date_and_time: u32,
}
