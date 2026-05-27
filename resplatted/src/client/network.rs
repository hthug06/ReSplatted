use bytes::{BufMut, BytesMut};
use resplatted_protocol::io::read::MinecraftReadExt;
use resplatted_protocol::io::write::MinecraftWriteExt;
use resplatted_protocol::packet::{PacketWrite, RawPacket, encode_packet};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

/// The packet reader struct contain everything needed to read a packet,
/// the stream, the cipher and the compression_threshold
pub struct PacketReader {
    pub stream: BufReader<OwnedReadHalf>,
    // Later, encryption and compression will be here
    // pub cipher: Option<Aes128Cfb8Decryptor>,
    // pub compression_threshold: Option<i32>,
}

impl PacketReader {
    /// Read the next packet (with decompression and decryption)
    pub async fn read_packet(&mut self) -> std::io::Result<RawPacket> {
        // Cipher activate : decrypt

        // Read size
        let length = self.read_async_var_int().await? as usize;

        // Put all the packet in a buffer
        let mut raw_buffer = vec![0u8; length];
        self.stream.read_exact(&mut raw_buffer).await?;

        // Decrypt everything

        // If compression active, decompress

        // Extract id and data
        let mut cursor = Cursor::new(&raw_buffer);
        let id = cursor.read_var_int()?;

        let position = cursor.position() as usize;
        let payload = raw_buffer[position..].to_vec();

        Ok(RawPacket { id, payload })
    }

    /// Read a byte with decryption (used to read varint)
    async fn read_byte(&mut self) -> std::io::Result<u8> {
        let mut buf = [0u8; 1];
        self.stream.read_exact(&mut buf).await?;

        // decrypt here (later)

        Ok(buf[0])
    }

    /// read a varint byte by byte (async)
    async fn read_async_var_int(&mut self) -> std::io::Result<i32> {
        let mut value: i32 = 0;
        let mut position: i32 = 0;

        loop {
            let current_byte = self.read_byte().await?;
            value |= ((current_byte & 0x7F) as i32) << position;
            if (current_byte & 0x80) == 0 {
                break;
            }
            position += 7;
            if position >= 32 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt too Big",
                ));
            }
        }
        Ok(value)
    }
}

/// Packet writer, with the stream, the cipher and the compression_threshold
pub struct PacketWriter {
    pub stream: OwnedWriteHalf,
    // For the Future
    // pub cipher: Option<Aes128Cfb8Encryptor>,
    // pub compression_threshold: Option<i32>,
}

impl PacketWriter {
    /// Send a packet (with encryption and compression if needed)
    pub async fn write_and_send_packet<P: PacketWrite>(
        &mut self,
        packet: &P,
    ) -> std::io::Result<()> {
        // get ID + DATA
        let raw_payload = encode_packet(packet)?;

        // Create the buffer that we're going to send
        let mut final_buffer = BytesMut::new();

        // In the future, the compression will be here
        // https://minecraft.wiki/w/Java_Edition_protocol/Packets#Packet_format

        // Here this is without compression
        // First, the id + data size
        final_buffer.write_var_int(raw_payload.len() as i32);
        // Then the id + data
        final_buffer.put_slice(&raw_payload);

        // The encryption will be here
        // https://minecraft.wiki/w/Java_Edition_protocol/Encryption

        // Packet is ready, send it to the network
        self.stream.write_all(&final_buffer).await?;
        Ok(())
    }
}
