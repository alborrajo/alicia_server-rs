use std::ffi::CString;

use deku::{DekuRead, DekuWrite};
use postgres_from_row::FromRow;

use crate::database::{CStringSql, U8Sql, U16Sql, U32Sql};

#[derive(Debug, Default, Clone, DekuRead, DekuWrite, FromRow)]
pub struct Horse {
    #[from_row(from = "U32Sql")]
    pub uid: u32,
    #[from_row(from = "U32Sql")]
    pub tid: u32,
    #[from_row(from = "CStringSql")]
    pub name: CString,

    #[from_row(flatten)]
    pub parts: Parts,
    #[from_row(flatten)]
    pub appearance: Appearance,
    #[from_row(flatten)]
    pub stats: Stats,
    #[from_row(from = "U32Sql")]
    pub rating: u32,
    #[from_row(from = "U8Sql")]
    pub class: u8,
    #[from_row(from = "U8Sql")]
    pub class_progress: u8,
    #[from_row(from = "U8Sql")]
    pub grade: u8,
    #[from_row(from = "U16Sql")]
    pub growth_points: u16,

    #[from_row(flatten)]
    pub vals0: Vals0,
    #[from_row(flatten)]
    pub vals1: Vals1,
    #[from_row(flatten)]
    pub mastery: Mastery,

    #[from_row(from = "U32Sql")]
    pub val16: u32,
    #[from_row(from = "U32Sql")]
    pub val17: u32,
}

#[derive(Debug, Clone, DekuRead, DekuWrite, FromRow)]
pub struct Parts {
    #[from_row(from = "U8Sql")]
    pub skin_id: u8,
    #[from_row(from = "U8Sql")]
    pub mane_id: u8,
    #[from_row(from = "U8Sql")]
    pub tail_id: u8,
    #[from_row(from = "U8Sql")]
    pub face_id: u8,
}
impl Default for Parts {
    fn default() -> Self {
        Self {
            skin_id: 1,
            mane_id: 0,
            tail_id: 0,
            face_id: 0,
        }
    }
}
impl Parts {
    pub fn random() -> Self {
        Self {
            skin_id: rand::random_range(1..=15),
            mane_id: rand::random_range(0..=40),
            tail_id: rand::random_range(0..=30),
            face_id: 0,
        }
    }
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite, FromRow)]
pub struct Appearance {
    #[from_row(from = "U8Sql")]
    pub scale: u8,
    #[from_row(from = "U8Sql")]
    pub leg_length: u8,
    #[from_row(from = "U8Sql")]
    pub leg_volume: u8,
    #[from_row(from = "U8Sql")]
    pub body_length: u8,
    #[from_row(from = "U8Sql")]
    pub body_volume: u8,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite, FromRow)]
pub struct Stats {
    #[from_row(from = "U32Sql")]
    pub agility: u32,
    #[from_row(from = "U32Sql")]
    pub control: u32,
    #[from_row(from = "U32Sql")]
    pub speed: u32,
    #[from_row(from = "U32Sql")]
    pub strength: u32,
    #[from_row(from = "U32Sql")]
    pub spirit: u32,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite, FromRow)]
pub struct Vals0 {
    #[from_row(from = "U16Sql")]
    pub stamina: u16,
    #[from_row(from = "U16Sql")]
    pub attractiveness: u16,

    #[from_row(from = "U16Sql")]
    pub hunger: u16,
    #[from_row(from = "U16Sql", rename = "vals0_val0")]
    pub val0: u16,

    #[from_row(from = "U16Sql", rename = "vals0_val1")]
    pub val1: u16,
    #[from_row(from = "U16Sql", rename = "vals0_val2")]
    pub val2: u16,

    #[from_row(from = "U16Sql", rename = "vals0_val3")]
    pub val3: u16,
    #[from_row(from = "U16Sql", rename = "vals0_val4")]
    pub val4: u16,

    #[from_row(from = "U16Sql", rename = "vals0_val5")]
    pub val5: u16,
    #[from_row(from = "U16Sql", rename = "vals0_val6")]
    pub val6: u16,

    #[from_row(from = "U16Sql", rename = "vals0_val7")]
    pub val7: u16,
    #[from_row(from = "U16Sql", rename = "vals0_val8")]
    pub val8: u16,

    #[from_row(from = "U16Sql", rename = "vals0_val9")]
    pub val9: u16,
    #[from_row(from = "U16Sql", rename = "vals0_val10")]
    pub val10: u16,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite, FromRow)]
pub struct Vals1 {
    #[from_row(from = "U8Sql", rename = "vals1_val0")]
    pub val0: u8,
    #[from_row(from = "U32Sql", rename = "vals1_val1")]
    pub val1: u32,
    #[from_row(from = "U32Sql")]
    pub date_of_birth: u32,

    #[from_row(from = "U8Sql", rename = "vals1_val3")]
    pub val3: u8,
    #[from_row(from = "U8Sql", rename = "vals1_val4")]
    pub val4: u8,
    #[from_row(from = "U32Sql")]
    pub class_progression: u32,
    #[from_row(from = "U32Sql", rename = "vals1_val5")]
    pub val5: u32,

    #[from_row(from = "U8Sql")]
    pub potential_level: u8,
    #[from_row(from = "U8Sql")]
    pub has_potential: u8,
    #[from_row(from = "U8Sql")]
    pub potential_value: u8,
    #[from_row(from = "U8Sql", rename = "vals1_val9")]
    pub val9: u8,

    #[from_row(from = "U8Sql")]
    pub luck: u8,
    #[from_row(from = "U8Sql")]
    pub has_luck: u8,
    #[from_row(from = "U8Sql", rename = "vals1_val12")]
    pub val12: u8,

    #[from_row(from = "U16Sql")]
    pub fatigue: u16,
    #[from_row(from = "U16Sql", rename = "vals1_val14")]
    pub val14: u16,
    #[from_row(from = "U16Sql")]
    pub emblem: u16,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite, FromRow)]
pub struct Mastery {
    #[from_row(from = "U32Sql")]
    pub spur_magic_count: u32,
    #[from_row(from = "U32Sql")]
    pub jump_count: u32,
    #[from_row(from = "U32Sql")]
    pub sliding_time: u32,
    #[from_row(from = "U32Sql")]
    pub gliding_distance: u32, // Divided by 10?
}
