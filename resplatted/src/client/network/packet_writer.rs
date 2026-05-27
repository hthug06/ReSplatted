use bytes::{BufMut, BytesMut};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use resplatted_protocol::io::write::MinecraftWriteExt;
use resplatted_protocol::packet::{PacketWrite, encode_packet};
use std::io::{ErrorKind, Write};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

/// Packet writer, with the stream, the cipher and the compression_threshold
pub struct PacketWriter {
    pub stream: OwnedWriteHalf,
    pub compression_threshold: Option<i32>,
    // For the Future
    // pub cipher: Option<Aes128Cfb8Encryptor>,
}

impl PacketWriter {
    /// Send a packet (with encryption and compression if needed)
    pub async fn write_and_send_packet<P: PacketWrite>(
        &mut self,
        packet: &P,
    ) -> std::io::Result<()> {
        // get ID + DATA
        let raw_payload = encode_packet(packet)?;

        // Compress here before the final buffer, so if the compression fail no ram is allocated
        let processed_payload = compress_payload(&raw_payload, self.compression_threshold)?;

        // Create the buffer that we're going to send
        let mut final_buffer = BytesMut::new();

        // Create final the packet
        // Packet size
        final_buffer.write_var_int(processed_payload.len() as i32);
        // Then the id + data
        final_buffer.put_slice(&processed_payload);

        // The encryption will be here
        // https://minecraft.wiki/w/Java_Edition_protocol/Encryption

        // Packet is ready, send it to the network
        self.stream.write_all(&final_buffer).await?;
        Ok(())
    }
}

/// Compress the packet payload.
///
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Packet_format
pub fn compress_payload(
    raw_payload: &[u8],
    compression_threshold: Option<i32>,
) -> std::io::Result<BytesMut> {
    let mut data_buffer = BytesMut::new();

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
            data_buffer.write_var_int(0);
            data_buffer.put_slice(raw_payload);
        }
        // Compress
        else {
            data_buffer.write_var_int(uncompressed_length as i32);

            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(raw_payload)?;
            let compressed_payload = encoder.finish()?;

            data_buffer.put_slice(&compressed_payload);
        }
    }
    // Compression not active
    else {
        data_buffer.put_slice(raw_payload);
    }

    Ok(data_buffer)
}
