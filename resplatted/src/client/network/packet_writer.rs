use flate2::{Compression, write::ZlibEncoder};
use resplatted_protocol::{
    io::write::MinecraftWriteExt,
    packet::{PacketWrite, encode_packet},
};
use std::io::{ErrorKind, Write};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

/// Packet writer, with the stream, the cipher and the compression_threshold
pub struct PacketWriter {
    pub stream: OwnedWriteHalf,
    pub compression_threshold: Option<i32>,
    // For the Future
    // pub cipher: Option<Aes128Cfb8Encryptor>,
    pub raw_payload_buffer: Vec<u8>,
    pub compress_buffer: Vec<u8>,
    pub final_buffer: Vec<u8>,
}

impl PacketWriter {
    /// Send a packet (with encryption and compression if needed)
    pub async fn write_and_send_packet<P: PacketWrite>(
        &mut self,
        packet: &P,
    ) -> std::io::Result<()> {
        // encode ID + DATA
        encode_packet(packet, &mut self.raw_payload_buffer)?;

        // Compress here before the final buffer, so if the compression fail no ram is allocated
        compress_payload(
            &self.raw_payload_buffer,
            self.compression_threshold,
            &mut self.compress_buffer,
        )?;

        // Clear the previous buffer
        self.final_buffer.clear();

        // Create final the packet
        // Packet size
        self.final_buffer
            .write_var_int(self.compress_buffer.len() as i32);
        // Then the id + data
        self.final_buffer.extend_from_slice(&self.compress_buffer);

        // The encryption will be here
        // https://minecraft.wiki/w/Java_Edition_protocol/Encryption

        // Packet is ready, send it to the network
        self.stream.write_all(&self.final_buffer).await?;
        Ok(())
    }
}

/// Compress the packet payload.
///
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Packet_format
pub fn compress_payload(
    raw_payload: &[u8],
    compression_threshold: Option<i32>,
    out_buffer: &mut Vec<u8>,
) -> std::io::Result<()> {
    out_buffer.clear();

    let uncompressed_length = raw_payload.len();
    if uncompressed_length > 8_388_608 {
        return Err(std::io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "Packet too big (is {}, should be less than 8388608)",
                uncompressed_length
            ),
        ));
    }

    if let Some(threshold) = compression_threshold {
        // Nothing to compress
        if (uncompressed_length as i32) < threshold {
            out_buffer.write_var_int(0);
            out_buffer.extend_from_slice(raw_payload);
        }
        // Compress
        else {
            out_buffer.write_var_int(uncompressed_length as i32);
            // Write directly into the out_buffer
            let mut encoder = ZlibEncoder::new(&mut *out_buffer, Compression::default());
            encoder.write_all(raw_payload)?;
            encoder.finish()?;
        }
    }
    // Compression not active
    else {
        out_buffer.extend_from_slice(raw_payload);
    }

    Ok(())
}
