use deku::{DekuRead, DekuWrite};

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct Item {
    pub uid: u32,
    pub tid: u32,
    pub val: u32,
    pub count: u32,
}
