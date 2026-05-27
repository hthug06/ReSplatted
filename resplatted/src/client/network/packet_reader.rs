use flate2::bufread::ZlibDecoder;
use resplatted_protocol::io::read::MinecraftReadExt;
use resplatted_protocol::packet::RawPacket;
use std::io::{Cursor, Read};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

/// The packet reader struct contain everything needed to read a packet,
/// the stream, the cipher and the compression_threshold
pub struct PacketReader {
    pub stream: BufReader<OwnedReadHalf>,
    pub compression_threshold: Option<i32>,
    // Later, encryption and compression will be here
    // pub cipher: Option<Aes128Cfb8Decryptor>,
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
        let decompressed_data = decompress_payload(&raw_buffer, self.compression_threshold)?;

        // Extract id and data
        let mut cursor = Cursor::new(&decompressed_data);
        let id = cursor.read_var_int()?;

        let position = cursor.position() as usize;
        let payload = decompressed_data[position..].to_vec();

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

/// Decompress the payload of a packet to read it
pub fn decompress_payload(
    raw_buffer: &[u8],
    compression_threshold: Option<i32>,
) -> std::io::Result<Vec<u8>> {
    if let Some(threshold) = compression_threshold {
        let mut cursor = Cursor::new(raw_buffer);
        let uncompressed_length = cursor.read_var_int()?;
        let pos = cursor.position() as usize;

        if uncompressed_length == 0 {
            Ok(raw_buffer[pos..].to_vec())
        } else {
            if uncompressed_length < threshold {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Badly compressed packet - size {} below threshold {}",
                        uncompressed_length, threshold
                    ),
                ));
            }

            // Security check (copied from Minecraft)
            if uncompressed_length > 8_388_608 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Badly compressed packet - size exceeds protocol maximum",
                ));
            }

            let compressed_data = &raw_buffer[pos..];
            let mut decoder = ZlibDecoder::new(compressed_data);
            let mut decompressed_data = vec![0u8; uncompressed_length as usize];
            decoder.read_exact(&mut decompressed_data)?;

            Ok(decompressed_data)
        }
    } else {
        Ok(raw_buffer.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{BufMut, BytesMut};
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use resplatted_protocol::io::write::MinecraftWriteExt;
    use std::io::Write;

    #[test]
    fn test_decompression_disabled() {
        let fake_data = vec![0x01, 0x02, 0x03];
        let result = decompress_payload(&fake_data, None).unwrap();
        assert_eq!(result, fake_data);
    }

    #[test]
    fn test_valid_compression() {
        let threshold = 50;
        let original_data = vec![0xAA; 100];

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&original_data).unwrap();
        let compressed_data = encoder.finish().unwrap();

        let mut network_buffer = BytesMut::new();
        network_buffer.write_var_int(original_data.len() as i32);
        network_buffer.put_slice(&compressed_data);

        let result = decompress_payload(&network_buffer, Some(threshold)).unwrap();
        assert_eq!(result, original_data);
    }

    /// Try an uncompressed packet with the compression active (uncompressed_length == 0)
    #[test]
    fn test_decode_compression_active_uncompressed_packet() {
        let mut network_buffer = BytesMut::new();
        network_buffer.write_var_int(0); // Data Length = 0
        network_buffer.put_slice(&[0xDE, 0xAD, 0xBE, 0xEF]); // Payload brut

        let result = decompress_payload(&network_buffer, Some(256)).unwrap();

        assert_eq!(
            result,
            vec![0xDE, 0xAD, 0xBE, 0xEF],
            "Should return bytes without any changes"
        );
    }

    /// Packet with a size below the threshold
    #[test]
    fn test_decode_rejects_below_threshold() {
        let threshold = 256;
        let original_data = vec![0xAA; 50]; // 50 bytes (< 256)

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&original_data).unwrap();
        let compressed_data = encoder.finish().unwrap();

        let mut network_buffer = BytesMut::new();
        network_buffer.write_var_int(50); // Data Length = 50
        network_buffer.put_slice(&compressed_data);

        let result = decompress_payload(&network_buffer, Some(threshold));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
    }

    // 3. Paquet dépassant 8 388 608 bytes (Zip Bomb)
    /// Packet size > 8 388 608 bytes (Zip Bomb)
    #[test]
    fn test_decode_rejects_zip_bomb() {
        let mut network_buffer = BytesMut::new();
        network_buffer.write_var_int(10_000_000);
        network_buffer.put_slice(&[0x78, 0x9C, 0x01, 0x02]);

        let result = decompress_payload(&network_buffer, Some(256));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
    }

    // Corrupt data / invalid zlib
    #[test]
    fn test_decode_invalid_zlib_data() {
        let mut network_buffer = BytesMut::new();
        network_buffer.write_var_int(500); // valid size (> 256)
        // send random byte that does not respect zlib
        network_buffer.put_slice(&[0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x11]);

        let result = decompress_payload(&network_buffer, Some(256));

        assert!(result.is_err());
    }

    /// threshold = 0 -> everything compress
    #[test]
    fn test_decode_threshold_zero() {
        let original_data = vec![0xBB; 5]; // length = 5

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&original_data).unwrap();
        let compressed_data = encoder.finish().unwrap();

        let mut network_buffer = BytesMut::new();
        network_buffer.write_var_int(5); // Data Length = 5
        network_buffer.put_slice(&compressed_data);

        let result = decompress_payload(&network_buffer, Some(0)).unwrap();

        assert_eq!(
            result, original_data,
            "Should decompress every packet because threshold = 0 "
        );
    }
}
