use crate::client::core::MinecraftClient;
use crate::client::state::ProtocolState;
use resplatted_protocol::packet::PacketRead;
use resplatted_protocol::packet::configuration::c_disconnect::ConfigurationDisconnectPacket;
use resplatted_protocol::packet::configuration::client_information::ClientInformationPacket;
use std::io::{Cursor, Error, ErrorKind};

impl MinecraftClient {
    /// Handle login phase between the client and the server
    pub async fn configuration(&mut self) -> std::io::Result<ProtocolState> {
        // Send the client information packet
        self.writer
            .write_and_send_packet(&ClientInformationPacket::default())
            .await?;

        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet id to know what packet we need to handle
            match raw_packet.id {
                // Disconnect Packet
                0x02 => {
                    let packet =
                        ConfigurationDisconnectPacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!(
                            "Disconnected by server in the configuration phase: {}",
                            packet.reason
                        ),
                    ));
                }
                // Error on the network or unimplemented packet
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Packet ID unknown in the configuration phase: 0x{:02X}",
                            raw_packet.id
                        ),
                    ));
                }
            }
        }
    }
}
