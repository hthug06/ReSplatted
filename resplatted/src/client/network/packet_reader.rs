use flate2::bufread::ZlibDecoder;
use resplatted_protocol::io::read::MinecraftReadExt;
use resplatted_protocol::packet::RawPacket;
use std::io::Cursor;
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
        let decompressed_data = if let Some(threshold) = self.compression_threshold {
            // if the program is here, compression is activated
            let mut cursor = Cursor::new(&raw_buffer);
            let uncompressed_length = cursor.read_var_int()?;
            let pos = cursor.position() as usize;

            if uncompressed_length == 0 {
                raw_buffer[pos..].to_vec()
            } else {
                if uncompressed_length < threshold {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!(
                            "Badly compressed packet - size of {} is below server threshold of {}",
                            uncompressed_length, threshold
                        ),
                    ));
                }

                // Protection (copied from Minecraft)
                if uncompressed_length > 8_388_608 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!(
                            "Badly compressed packet - size of {} is larger than protocol maximum of 8388608",
                            uncompressed_length
                        ),
                    ));
                }

                let compressed_data = &raw_buffer[pos..];
                let mut decoder = ZlibDecoder::new(compressed_data);

                let mut decompressed_data = vec![0u8; uncompressed_length as usize];
                decoder.read_exact(&mut decompressed_data)?;

                decompressed_data
            }
        } else {
            // None compression isn't active or will never be active
            raw_buffer
        };

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
