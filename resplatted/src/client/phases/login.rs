use crate::client::core::MinecraftClient;
use log::debug;
use resplatted_protocol::io::ProtocolState;
use resplatted_protocol::packet::login::LoginPacket;
use resplatted_protocol::packet::login::{
    s_login_acknowledged::LoginAcknowledgedPacket, s_login_start::LoginStartPacket,
};
use std::io::{Error, ErrorKind};

impl MinecraftClient {
    /// Handle the login phase between the client and the server
    pub async fn login(&mut self, username: &str) -> std::io::Result<()> {
        // login function so send LoginStartPacket
        self.writer
            .write_and_send_packet(&LoginStartPacket::new(username.to_string()), &self.context)
            .await?;

        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet id to know what packet we need to handle
            match LoginPacket::parse(&raw_packet, &self.context)? {
                LoginPacket::Disconnect(packet) => {
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!("Disconnected by server in login phase: {}", packet.reason),
                    ));
                }
                // Encryption request packet
                // It's sent when the server is in online mode.
                // This is a bot, not a premium account, so we just stop the program
                LoginPacket::EncryptionRequest(_) => {
                    return Err(Error::new(
                        ErrorKind::PermissionDenied,
                        "Server is in Online mode.",
                    ));
                }
                LoginPacket::LoginSuccess(packet) => {
                    debug!("Received Game Profile: {:?}", packet);

                    self.writer
                        .write_and_send_packet(&LoginAcknowledgedPacket, &self.context)
                        .await?;

                    self.context.state = ProtocolState::Configuration;
                    return Ok(());
                }
                // Login Success Packet$
                // Sent when the login phase is done.
                // We need to send the login acknowledged packet in return
                LoginPacket::SetCompression(packet) => {
                    self.reader.compression_threshold = Some(packet.threshold);
                    self.writer.compression_threshold = Some(packet.threshold);
                    debug!("Compression enabled with threshold: {}", packet.threshold);
                }
                // Error on the network or unimplemented packet
                LoginPacket::Unknown(id) => {
                    debug!("Packet ID unknown in the login phase: 0x{:02X}", id);
                }
            };
        }
    }
}
