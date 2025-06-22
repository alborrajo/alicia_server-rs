use std::{
    ffi::CString,
    io::{Read, Seek, Write},
};

use deku::{
    DekuError, DekuRead, DekuReader, DekuWrite, DekuWriter, reader::Reader, writer::Writer,
};

use crate::{
    commands::{
        LengthPrefixedVec,
        shared::{
            address::Address,
            character::{
                AgeGroup, AnotherPlayerRelatedThing, Character, Gender, PlayerRelatedThing,
                YetAnotherPlayerRelatedThing,
            },
            horse::Horse,
            item::Item,
            win_file_time::WinFileTime,
        },
    },
    impl_command_traits,
    packet::CommandId,
};

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Login {
    pub constant0: u16,
    pub constant1: u16,
    pub login_id: CString,
    pub member_no: u32,
    pub auth_key: CString,
    pub val0: u8,
}
impl_command_traits!(Login, CommandId::AcCmdCLLogin);

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum LoginCancelReason {
    InvalidUser = 1,
    Duplicated = 2,
    InvalidVersion = 3,
    InvalidEquipment = 4,
    InvalidLoginId = 5,
    DisconnectYourself = 6,
}
impl Default for LoginCancelReason {
    fn default() -> Self {
        Self::InvalidUser
    }
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct LoginCancel {
    pub reason: LoginCancelReason,
}
impl_command_traits!(LoginCancel, CommandId::AcCmdCLLoginCancel);

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct LoginOk {
    pub lobby_time: WinFileTime,
    pub val0: u32,

    pub self_uid: u32,
    pub nick_name: CString,
    pub motd: CString,
    pub profile_gender: Gender,
    pub status: CString,

    pub character_equipment: LengthPrefixedVec<1, Item>,
    pub mount_equipment: LengthPrefixedVec<1, Item>,

    pub level: u16,
    pub carrots: u32,
    pub val1: u32,
    pub val2: u32,
    pub val3: u8,

    pub options: Options,

    pub age_group: AgeGroup,
    pub hide_age: u8, // TODO: Bool?

    pub val5: LengthPrefixedVec<1, Val5>,

    pub val6: CString,

    pub lobby_server_address: Address,
    pub scrambling_constant: u32,

    pub character: Character,
    pub horse: Horse,

    pub val7: Val7,

    pub bitfield: u32,

    pub val9: Val9,

    pub val10: u32,

    pub val11: Val11,

    pub val12: Val12,

    pub val13: Val13,

    pub val14: u32,
    pub val15: PlayerRelatedThing,
    pub val16: u8,

    pub val17: AnotherPlayerRelatedThing, // Seomthing with rental horse

    pub val18: u32,
    pub val19: u32,
    pub val20: u32,

    pub val21: YetAnotherPlayerRelatedThing,
}
impl_command_traits!(LoginOk, CommandId::AcCmdCLLoginOK);

#[derive(Default, Debug)]
pub struct Options {
    pub keyboard_options: Option<KeyboardOptions>,
    pub macro_options: Option<MacroOptions>,
    pub value_options: Option<u32>,
    // TODO: GamepadOptions
}
impl<'a> DekuReader<'a> for Options {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        ctx: (),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let mut options = Options::default();
        let option_type_mask = u32::from_reader_with_ctx(reader, ctx)?;
        if option_type_mask & 1 != 0 {
            options.keyboard_options = Some(KeyboardOptions::from_reader_with_ctx(reader, ctx)?);
        }
        if option_type_mask & (1 << 3) != 0 {
            options.macro_options = Some(MacroOptions::from_reader_with_ctx(reader, ctx)?);
        }
        if option_type_mask & (1 << 4) != 0 {
            options.value_options = Some(u32::from_reader_with_ctx(reader, ctx)?);
        }
        // TODO: GamepadOptions
        Ok(options)
    }
}
impl DekuWriter for Options {
    fn to_writer<W: Write + Seek>(&self, writer: &mut Writer<W>, ctx: ()) -> Result<(), DekuError> {
        let mut option_type_mask = 0;
        if self.keyboard_options.is_some() {
            option_type_mask |= 1
        };
        if self.macro_options.is_some() {
            option_type_mask |= 1 << 3
        };
        if self.value_options.is_some() {
            option_type_mask |= 1 << 4
        };
        // TODO: GamepadOptions
        option_type_mask.to_writer(writer, ctx)?;

        if let Some(ref keyboard_options) = self.keyboard_options {
            keyboard_options.to_writer(writer, ctx)?;
        }
        if let Some(ref macro_options) = self.macro_options {
            macro_options.to_writer(writer, ctx)?;
        }
        if let Some(ref value_options) = self.value_options {
            value_options.to_writer(writer, ctx)?;
        }
        // TODO: GamepadOptions
        Ok(())
    }
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct KeyboardOptions {
    pub bindings: LengthPrefixedVec<1, KeyboardOption>,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct KeyboardOption {
    pub index: u16,
    pub r#type: u8,
    pub key: u8,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct MacroOptions {
    pub macros: [CString; 8],
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val5 {
    pub val0: u16,
    pub val1: LengthPrefixedVec<1, Val5Val1>,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val5Val1 {
    pub val0: u32,
    pub val1: u32,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val7 {
    pub values: LengthPrefixedVec<1, Val7Value>,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val7Value {
    pub val0: u32,
    pub val1: u32,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val9 {
    pub val0: u16,
    pub val1: u16,
    pub val2: u16,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val11 {
    pub val0: u8,
    pub val1: u32,
    pub val2: u16,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val12 {
    pub values: LengthPrefixedVec<1, Val12Value>,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val12Value {
    pub val0: u8,
    pub val1: u8,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val13 {
    pub values: LengthPrefixedVec<1, Val13Value>,
}

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct Val13Value {
    pub val0: u16,
    pub val1: u8,
    pub val2: u8,
}
