use deku::prelude::*;

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Quest {
    pub tid: u16,
    pub member0: u32,
    pub member1: u8,
    pub member2: u32,
    pub member3: u8,
    pub member4: u8,
}
