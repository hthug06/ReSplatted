use crate::client::core::MinecraftClient;
use crate::client::state::ProtocolState;
use log::info;
use resplatted_protocol::packet::PacketRead;
use resplatted_protocol::packet::login::c_disconnect::LoginDisconnectPacket;
use resplatted_protocol::packet::login::c_set_compression::SetCompressionPacket;
use resplatted_protocol::packet::login::s_login_start::LoginStartPacket;
use std::io::{Cursor, Error, ErrorKind};

impl MinecraftClient {
    /// Handle login phase between the client and the server
    pub async fn login(&mut self, username: &str) -> std::io::Result<ProtocolState> {
        // login function so send LoginStartPacket
        self.writer
            .write_and_send_packet(&LoginStartPacket::new(username.to_string()))
            .await?;

        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet id to know what packet we need to handle
            match raw_packet.id {
                // Disconnect packet
                0x00 => {
                    let packet =
                        LoginDisconnectPacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!("Disconnected by server: {}", packet.reason),
                    ));
                }
                // Compression packet
                0x03 => {
                    let packet = SetCompressionPacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    self.reader.compression_threshold = Some(packet.threshold);
                    self.writer.compression_threshold = Some(packet.threshold);
                    info!("Compression enabled with threshold: {}", packet.threshold);
                }
                // Error on the network or unimplemented packet
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Packet ID unknow in the login phase: 0x{:02X}",
                            raw_packet.id
                        ),
                    ));
                }
            }
        }
    }
}
