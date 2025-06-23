use std::{
    fmt::Debug,
    io::{Read, Seek, Write},
    net::Ipv4Addr,
};

use deku::{DekuError, DekuReader, DekuWriter, reader::Reader, writer::Writer};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Address {
    pub ip: Ipv4Addr,
    pub port: u16,
}
impl Default for Address {
    fn default() -> Self {
        Address {
            ip: Ipv4Addr::new(127, 0, 0, 1),
            port: 10030,
        }
    }
}
impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}
impl<'a> DekuReader<'a> for Address {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        ctx: (),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let mut octets = [0u8; 4];
        reader.read_bytes_const(&mut octets, deku::ctx::Order::Lsb0)?;
        let ip = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]);

        let port = u16::from_reader_with_ctx(reader, ctx)?;

        Ok(Address { ip, port })
    }
}
impl DekuWriter for Address {
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        _ctx: (),
    ) -> Result<(), DekuError> {
        writer.write_bytes(&self.ip.octets())?;
        writer.write_bytes(&self.port.to_le_bytes())?;
        Ok(())
    }
}
