use crate::client::{core::MinecraftClient, state::ProtocolState};
use log::debug;
use resplatted_protocol::{
    io::read::MinecraftReadExt,
    packet::{
        PacketRead,
        configuration::{
            c_disconnect::ConfigurationDisconnectPacket,
            c_finish_configuration::CFinishConfigurationPacket,
            c_keep_alive::CConfigurationKeepAlivePacket, c_know_pack::CKnownPackPacket,
            c_plugin_message::PluginMessagePacket, s_client_information::ClientInformationPacket,
            s_finish_configuration::SFinishConfigurationPacket,
            s_keep_alive::SConfigurationKeepAlivePacket, s_known_pack::SKnownPackPacket,
        },
    },
};
use std::io::{Cursor, Error, ErrorKind};

impl MinecraftClient {
    /// Handle configuration phase between the client and the server
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
                // Custom payload, not very interesting
                PluginMessagePacket::ID => {
                    let packet = PluginMessagePacket::read(&mut Cursor::new(&raw_packet.payload))?;

                    if packet.channel == "minecraft:brand" {
                        let mut cursor = Cursor::new(&packet.data);
                        if let Ok(brand) = cursor.read_string() {
                            debug!("Server brand is: {}", brand);
                        }
                    }
                }
                // Disconnect Packet
                ConfigurationDisconnectPacket::ID => {
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
                CFinishConfigurationPacket::ID => {
                    debug!(
                        "Received Finish Configuration packet. Replying with Acknowledge Finish Configuration Packet"
                    );
                    // Need to answer the pack we already have on the disk (none)
                    self.writer
                        .write_and_send_packet(&SFinishConfigurationPacket)
                        .await?;

                    return Ok(ProtocolState::Play);
                }
                CConfigurationKeepAlivePacket::ID => {
                    let packet =
                        CConfigurationKeepAlivePacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    self.writer
                        .write_and_send_packet(&SConfigurationKeepAlivePacket { id: packet.id })
                        .await?;
                }
                CKnownPackPacket::ID => {
                    debug!("Received Select Know Pack packet. Replying with Know Pack Packet");
                    // Need to answer the pack we already have on the disk (none)
                    self.writer.write_and_send_packet(&SKnownPackPacket).await?;
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
                    debug!(
                        "Packet ID unknown in the configuration phase: 0x{:02X}",
                        raw_packet.id
                    );
                }
            }
        }
    }
}
