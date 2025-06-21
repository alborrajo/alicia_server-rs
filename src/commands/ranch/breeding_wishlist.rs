use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

use crate::{
    commands::{
        LengthPrefixedVec,
        shared::horse::{Appearance, Parts, Stats},
    },
    impl_command_traits,
    packet::CommandId,
};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct BreedingWishlist {}
impl_command_traits!(BreedingWishlist, CommandId::AcCmdCRBreedingWishlist);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct BreedingWishlistCancel {}
impl_command_traits!(
    BreedingWishlistCancel,
    CommandId::AcCmdCRBreedingWishlistCancel
);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct BreedingWishlistOk {
    pub wishlist: LengthPrefixedVec<1, WishlistElement>,
}
impl_command_traits!(BreedingWishlistOk, CommandId::AcCmdCRBreedingWishlistOK);

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct WishlistElement {
    pub unk0: CString,
    pub uid: u32,
    pub tid: u32,
    pub unk1: u8,
    pub unk2: CString,
    pub unk3: u8,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub stats: Stats,
    pub parts: Parts,
    pub appearance: Appearance,
    pub unk9: u8,
    pub unk10: u8,
    pub unk11: u8,
}
