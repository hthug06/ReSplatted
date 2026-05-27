pub mod handshake;
pub mod status;

use crate::io::write::MinecraftWriteExt;
use bytes::{BufMut, BytesMut};
use std::io::Cursor;

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

/// Encode a packet into a buffer. This will write the ID, the data and the size of the packet
/// Not in the PacketWrite because after, the compression and encryption can be passed in argument
pub fn encode_packet<P: PacketWrite>(packet: &P) -> std::io::Result<BytesMut> {
    let mut payload = BytesMut::new();

    // First the ID
    payload.write_var_int(P::ID);

    // Then the data
    packet.write(&mut payload)?;

    // Finally wrap it
    let mut final_packet = BytesMut::new();
    final_packet.write_var_int(payload.len() as i32);
    final_packet.put_slice(&payload);

    Ok(final_packet)
}
