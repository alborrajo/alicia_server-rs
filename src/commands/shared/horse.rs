use std::ffi::CString;

use deku::{DekuRead, DekuWrite};

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Horse {
    pub uid: u32,
    pub tid: u32,
    pub name: CString,

    pub parts: Parts,
    pub appearance: Appearance,
    pub stats: Stats,
    pub rating: u32,
    pub class: u8,
    pub class_progress: u8,
    pub grade: u8,
    pub growth_points: u16,

    pub vals0: Vals0,
    pub vals1: Vals1,
    pub mastery: Mastery,

    pub val16: u32,
    pub val17: u32,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Parts {
    pub skin_id: u8,
    pub mane_id: u8,
    pub tail_id: u8,
    pub face_id: u8
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Appearance {
    pub scale: u8,
    pub leg_length: u8,
    pub leg_volume: u8,
    pub body_length: u8,
    pub body_volume: u8
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Stats {
    pub agility: u32,
    pub control: u32,
    pub speed: u32,
    pub strength: u32,
    pub spirit: u32
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Vals0 {
    pub stamina: u16,
    pub attractiveness: u16,

    pub hunger: u16,
    pub val0: u16,

    pub val1: u16,
    pub val2: u16,

    pub val3: u16,
    pub val4: u16,

    pub val5: u16,
    pub val6: u16,

    pub val7: u16,
    pub val8: u16,

    pub val9: u16,
    pub val10: u16,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Vals1 {
    pub val0: u8,
    pub val1: u32,
    pub date_of_birth: u32,

    pub val3: u8,
    pub val4: u8,
    pub class_progression: u32,
    pub val5: u32,

    pub potential_level: u8,
    pub has_potential: u8,
    pub potential_value: u8,
    pub val9: u8,

    pub luck: u8,
    pub has_luck: u8,
    pub val12: u8,

    pub fatigue: u16,
    pub val14: u16,
    pub emblem: u16,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Mastery {
    pub spur_magic_count: u32,
    pub jump_count: u32,
    pub sliding_time: u32,
    pub gliding_distance: u32 // Divided by 10?
}