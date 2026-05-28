use crate::client::core::MinecraftClient;
use crate::client::state::ProtocolState;
use log::{info, warn};
use resplatted_protocol::packet::PacketRead;
use resplatted_protocol::packet::play::c_disconnect::PlayDisconnectPacket;
use std::io::{Cursor, Error, ErrorKind};

impl MinecraftClient {
    /// Handle play phase between the client and the server
    pub async fn enter_game(&mut self) -> std::io::Result<ProtocolState> {
        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet id to know what packet we need to handle
            match raw_packet.id {
                // Disconnect packet
                PlayDisconnectPacket::ID => {
                    let packet = PlayDisconnectPacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!("Disconnected by server in play phase: {}", packet.reason),
                    ));
                }
                // Error on the network or unimplemented packet
                _ => {
                    // If we want, we can stop the program here on an unimplemented packet.
                    // But right now, we just want to skip it so just a warn in the console is okay
                    /*return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Packet ID unknown in the configuration phase: 0x{:02X}",
                            raw_packet.id
                        ),
                    ));*/
                    warn!(
                        "Packet ID unknown in the play phase: 0x{:02X}",
                        raw_packet.id
                    );
                }
            }
        }
    }
}
