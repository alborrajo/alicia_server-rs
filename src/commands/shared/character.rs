use deku::{DekuRead, DekuWrite};
use postgres_from_row::FromRow;
use std::ffi::CString;

use crate::database::{U8Sql, U16Sql};

#[derive(Debug, Default, DekuRead, DekuWrite, Clone, FromRow)]
pub struct Character {
    #[from_row(flatten)]
    pub parts: Parts,
    #[from_row(flatten)]
    pub appearance: Appearance,
}

#[derive(Debug, DekuRead, DekuWrite, Clone, FromRow)]
pub struct Parts {
    #[from_row(from = "U8Sql")]
    pub char_id: u8,
    #[from_row(from = "U8Sql")]
    pub mouth_serial_id: u8,
    #[from_row(from = "U8Sql")]
    pub face_serial_id: u8,
    #[from_row(rename = "parts_val0")]
    #[from_row(from = "U8Sql")]
    pub val0: u8,
}
impl Default for Parts {
    fn default() -> Self {
        Self {
            char_id: 10.into(),
            mouth_serial_id: 0.into(),
            face_serial_id: 0.into(),
            val0: 0.into(),
        }
    }
}

#[derive(Debug, Default, DekuRead, DekuWrite, Clone, FromRow)]
pub struct Appearance {
    #[from_row(rename = "appearance_val0")]
    #[from_row(from = "U16Sql")]
    pub val0: u16,
    #[from_row(from = "U16Sql")]
    pub head_size: u16,
    #[from_row(from = "U16Sql")]
    pub height: u16,
    #[from_row(from = "U16Sql")]
    pub thigh_volume: u16,
    #[from_row(from = "U16Sql")]
    pub leg_volume: u16,
    #[from_row(rename = "appearance_val1")]
    #[from_row(from = "U16Sql")]
    pub val1: u16,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum Gender {
    Unspecified = 0x0,
    Boy = 0x1,
    Girl = 0x2,
}
impl Default for Gender {
    fn default() -> Self {
        Gender::Unspecified
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum AgeGroup {
    /// Age <12
    Kid = 0x0C,
    /// Age 13-15
    Teenager = 0x0D,
    /// Age 16-18
    Highschooler = 0x10,
    /// Age 19+
    Adult = 0x13,
}
impl Default for AgeGroup {
    fn default() -> Self {
        AgeGroup::Kid
    }
}
#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct PlayerRelatedThing {
    pub val0: u32,
    pub val1: u8,
    pub val2: u32,
    pub val3: CString,
    pub val4: u8,
    pub val5: u32,
    /// ignored by the client?
    pub val6: u8,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct AnotherPlayerRelatedThing {
    pub mount_uid: u32,
    pub val1: u32,
    pub val2: u32,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct YetAnotherPlayerRelatedThing {
    pub val0: u32,
    pub val1: u32,
    pub val2: CString,
    pub val3: u32,
}
