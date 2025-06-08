use deku::{DekuRead, DekuWrite};
use std::ffi::CString;

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Character {
  pub parts: Parts,
  pub appearance: Appearance
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Parts {
  pub char_id: u8,
  pub mouth_serial_id: u8,
  pub face_serial_id: u8,
  pub val0: u8
}
impl Default for Parts {
    fn default() -> Self {
        Self {char_id: 10, mouth_serial_id: 0, face_serial_id: 0, val0: 0}
    }
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Appearance {
  pub val0: u16,
  pub head_size: u16,
  pub height: u16,
  pub thigh_volume: u16,
  pub leg_volume: u16,
  pub val1: u16
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum Gender 
{
  Unspecified = 0x0,
  Boy = 0x1,
  Girl = 0x2
}
impl Default for Gender {
    fn default() -> Self {
        Gender::Unspecified
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum AgeGroup
{
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
