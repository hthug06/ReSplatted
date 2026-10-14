use crate::client::core::MinecraftClient;
use log::debug;
use resplatted_protocol::io::ProtocolState;
use resplatted_protocol::io::read::MinecraftReadExt;
use resplatted_protocol::packet::common::s_keep_alive::SKeepAlivePacket;
use resplatted_protocol::packet::configuration::ConfigurationPacket;
use resplatted_protocol::packet::configuration::s_client_information::ClientInformationPacket;
use resplatted_protocol::packet::configuration::s_finish_configuration::SFinishConfigurationPacket;
use resplatted_protocol::packet::configuration::s_known_pack::SKnownPackPacket;
use std::io::{Cursor, Error, ErrorKind};

impl MinecraftClient {
    /// Handle the configuration phase between the client and the server
    pub async fn configuration(&mut self) -> std::io::Result<()> {
        // Send the client information packet
        self.writer
            .write_and_send_packet(&ClientInformationPacket::default(), &self.context)
            .await?;

        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet to know what packet we need to handle
            match ConfigurationPacket::parse(&raw_packet, &self.context)? {
                // Custom payload, not very interesting
                ConfigurationPacket::PluginMessage(packet) => {
                    if packet.channel == "minecraft:brand" {
                        let mut cursor = Cursor::new(&packet.data);
                        if let Ok(brand) = cursor.read_string() {
                            debug!("Server brand is: {}", brand);
                        }
                    }
                }
                ConfigurationPacket::Disconnect(packet) => {
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!(
                            "Disconnected by server in the configuration phase: {}",
                            packet.reason
                        ),
                    ));
                }
                ConfigurationPacket::FinishConfiguration(_) => {
                    debug!(
                        "Received Finish Configuration packet. Replying with Acknowledge Finish Configuration Packet"
                    );
                    self.writer
                        .write_and_send_packet(&SFinishConfigurationPacket, &self.context)
                        .await?;

                    self.context.state = ProtocolState::Play;
                    return Ok(());
                }
                ConfigurationPacket::KeepAlive(packet) => {
                    self.writer
                        .write_and_send_packet(&SKeepAlivePacket { id: packet.id }, &self.context)
                        .await?;
                }
                ConfigurationPacket::KnownPack(_) => {
                    debug!("Received Select Know Pack packet. Replying with Know Pack Packet");
                    // Need to answer the pack we already have on the disk (none)
                    self.writer
                        .write_and_send_packet(&SKnownPackPacket, &self.context)
                        .await?;
                }
                // Error on the network or unimplemented packet
                ConfigurationPacket::Unknown(id) => {
                    debug!("Packet ID unknown in the configuration phase: 0x{:02X}", id);
                }
            }
        }
    }
}
