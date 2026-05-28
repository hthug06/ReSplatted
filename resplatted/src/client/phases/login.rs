use crate::client::core::MinecraftClient;
use crate::client::state::ProtocolState;
use log::{debug, info};
use resplatted_protocol::packet::{
    PacketRead,
    login::{
        c_disconnect::LoginDisconnectPacket, c_encryption_request::EncryptionRequestPacket,
        c_login_success::LoginSuccessPacket, c_set_compression::SetCompressionPacket,
        s_login_acknowledged::LoginAcknowledgedPacket, s_login_start::LoginStartPacket,
    },
};
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
                LoginDisconnectPacket::ID => {
                    let packet =
                        LoginDisconnectPacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!("Disconnected by server in login phase: {}", packet.reason),
                    ));
                }
                // Encryption request packet
                // It's sent when the server is in online mode.
                // This is a bot, not a premium account so we just stop the program
                EncryptionRequestPacket::ID => {
                    return Err(Error::new(
                        ErrorKind::PermissionDenied,
                        "Server is in Online mode.",
                    ));
                }
                // Login Success Packet$
                // Sent when the login phase is done.
                // We need to send the login acknowledged packet in return
                LoginSuccessPacket::ID => {
                    let packet = LoginSuccessPacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    debug!("Received Game Profile: {:?}", packet);

                    self.writer
                        .write_and_send_packet(&LoginAcknowledgedPacket)
                        .await?;

                    return Ok(ProtocolState::Configuration);
                }
                // Compression packet
                SetCompressionPacket::ID => {
                    let packet = SetCompressionPacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    self.reader.compression_threshold = Some(packet.threshold);
                    self.writer.compression_threshold = Some(packet.threshold);
                    debug!("Compression enabled with threshold: {}", packet.threshold);
                }
                // Error on the network or unimplemented packet
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Packet ID unknown in the login phase: 0x{:02X}",
                            raw_packet.id
                        ),
                    ));
                }
            }
        }
    }
}
