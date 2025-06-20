use std::io::{Read, Seek, Write};

use deku::{
    DekuError, DekuRead, DekuReader, DekuWrite, DekuWriter, reader::Reader, writer::Writer,
};

use crate::{impl_command_traits, packet::CommandId};

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RanchSnapshot {
    pub snapshot: Snapshot,
}
impl_command_traits!(RanchSnapshot, CommandId::AcCmdCRRanchSnapshot);

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RanchSnapshotNotify {
    pub ranch_index: u16,
    pub snapshot: Snapshot,
}
impl_command_traits!(RanchSnapshotNotify, CommandId::AcCmdCRRanchSnapshotNotify);

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct FullSpatial {
    pub member0: u16,
    pub member1: u32,
    pub member2: u16,
    pub member3: [u8; 12],
    pub member4: [u8; 16],
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct PartialSpatial {
    pub member0: u16,
    pub member1: u32,
    pub member2: u16,
    pub member3: [u8; 12],
    pub member4: [u8; 16],
}

#[derive(Debug, Clone)]
pub enum Snapshot {
    Full(FullSpatial),
    Partial(PartialSpatial),
}
impl<'a> DekuReader<'a, ()> for Snapshot {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        ctx: (),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let snapshot_type = u8::from_reader_with_ctx(reader, ctx)?;
        match snapshot_type {
            0 => {
                let full_snapshot = FullSpatial::from_reader_with_ctx(reader, ctx)?;
                Ok(Snapshot::Full(full_snapshot))
            }
            1 => {
                let partial_snapshot = PartialSpatial::from_reader_with_ctx(reader, ctx)?;
                Ok(Snapshot::Partial(partial_snapshot))
            }
            _ => Err(DekuError::InvalidParam(
                format!("Unknown snapshot type: {}", snapshot_type).into(),
            )),
        }
    }
}
impl DekuWriter for Snapshot {
    fn to_writer<W: Write + Seek>(&self, writer: &mut Writer<W>, ctx: ()) -> Result<(), DekuError> {
        match self {
            Snapshot::Full(full_snapshot) => {
                0u8.to_writer(writer, ctx)?;
                full_snapshot.to_writer(writer, ctx)
            }
            Snapshot::Partial(partial_snapshot) => {
                1u8.to_writer(writer, ctx)?;
                partial_snapshot.to_writer(writer, ctx)
            }
        }
    }
}
