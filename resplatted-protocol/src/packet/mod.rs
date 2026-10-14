pub mod common;
pub mod configuration;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

use crate::io::ConnectionContext;
use crate::io::write::MinecraftWriteExt;
use std::io::Cursor;

#[derive(Debug)]
pub struct RawPacket {
    pub id: i32,
    pub payload: Vec<u8>,
}

/// Trait for reading the packet.
/// Server -> Client
pub trait PacketRead: Sized {
    /// The packet ID
    /// This can change depending on the client version
    fn id(ctx: &ConnectionContext) -> i32;
    /// Read the packet data from the buffer
    fn read(cursor: &mut Cursor<&[u8]>, ctx: &ConnectionContext) -> std::io::Result<Self>;
}

/// Trait for writing the packet
/// Client -> Server
pub trait PacketWrite {
    /// The packet ID
    /// This can change depending on the client version
    fn id(ctx: &ConnectionContext) -> i32;
    /// Write the packet into the buffer
    fn write(&self, buf: &mut Vec<u8>, ctx: &ConnectionContext) -> std::io::Result<()>;
}

/// Encode a packet into a BytesMut Buffer (ID + DATA)
pub fn encode_packet<P: PacketWrite>(
    packet: &P,
    buf: &mut Vec<u8>,
    ctx: &ConnectionContext,
) -> std::io::Result<()> {
    buf.clear();

    // ID
    buf.write_var_int(P::id(ctx));

    // Then DATA
    packet.write(buf, ctx)?;

    Ok(())
}

#[macro_export]
macro_rules! define_phase_packets {
    (
        $EnumName:ident {
            $( $Variant:ident($PacketType:ty) ),* $(,)?
        }
    ) => {
        pub enum $EnumName {
            $( $Variant($PacketType), )*
            Unknown(i32),
        }

        impl $EnumName {
            pub fn parse(raw: &RawPacket, ctx: &ConnectionContext) -> std::io::Result<Self> {
                let mut cursor = std::io::Cursor::new(raw.payload.as_slice());
                let id = raw.id;

                // Ensure the compiler checks sequentially using the packet's own id(ctx)
                $(
                    if id == <$PacketType>::id(ctx) {
                        return Ok(Self::$Variant(<$PacketType>::read(&mut cursor, ctx)?));
                    }
                )*

                Ok(Self::Unknown(id))
            }
        }
    };
}
