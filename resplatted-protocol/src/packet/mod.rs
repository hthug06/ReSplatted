pub mod handshake;
pub mod status;

use crate::io::write::MinecraftWriteExt;
use bytes::BytesMut;
use std::io::Cursor;

pub struct RawPacket {
    pub id: i32,
    pub payload: Vec<u8>,
}

/// Trait for reading the packet.
/// Server -> Client
pub trait PacketRead: Sized {
    const ID: i32;
    /// Read the packet data from the buffer
    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self>;
}

/// Trait for writing the packet
/// Client -> Server
pub trait PacketWrite {
    const ID: i32;
    /// Write the packet into the buffer
    fn write(&self, buf: &mut BytesMut) -> std::io::Result<()>;
}

/// Encode a packet into a BytesMut Buffer (ID + DATA)
pub fn encode_packet<P: PacketWrite>(packet: &P) -> std::io::Result<BytesMut> {
    let mut payload = BytesMut::with_capacity(128);

    // ID
    payload.write_var_int(P::ID);

    // Then DATA
    packet.write(&mut payload)?;

    Ok(payload)
}
