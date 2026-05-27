use bytes::{BufMut, BytesMut};
use resplatted_protocol::io::write::MinecraftWriteExt;
use resplatted_protocol::packet::{PacketWrite, encode_packet};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

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
