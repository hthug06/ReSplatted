use crate::client::core::MinecraftClient;
use log::debug;
use resplatted_protocol::io::ProtocolState;
use resplatted_protocol::packet::common::c_disconnect::DisconnectPacket;
use resplatted_protocol::packet::common::c_keep_alive::CKeepAlivePacket;
use resplatted_protocol::packet::common::s_keep_alive::SKeepAlivePacket;
use resplatted_protocol::{
    io::read::MinecraftReadExt,
    packet::{
        PacketRead,
        configuration::{
            c_finish_configuration::CFinishConfigurationPacket, c_know_pack::CKnownPackPacket,
            c_plugin_message::PluginMessagePacket, s_client_information::ClientInformationPacket,
            s_finish_configuration::SFinishConfigurationPacket, s_known_pack::SKnownPackPacket,
        },
    },
};
use std::io::{Cursor, Error, ErrorKind};

impl MinecraftClient {
    /// Handle configuration phase between the client and the server
    pub async fn configuration(&mut self) -> std::io::Result<()> {
        // Send the client information packet
        self.writer
            .write_and_send_packet(&ClientInformationPacket::default(), &self.context)
            .await?;

        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet id to know what packet we need to handle
            match raw_packet.id {
                // Custom payload, not very interesting
                id if id == PluginMessagePacket::id(&self.context) => {
                    let packet = PluginMessagePacket::read(
                        &mut Cursor::new(&raw_packet.payload),
                        &self.context,
                    )?;

                    if packet.channel == "minecraft:brand" {
                        let mut cursor = Cursor::new(&packet.data);
                        if let Ok(brand) = cursor.read_string() {
                            debug!("Server brand is: {}", brand);
                        }
                    }
                }
                // Disconnect Packet
                id if id == DisconnectPacket::id(&self.context) => {
                    let packet = DisconnectPacket::read(
                        &mut Cursor::new(&raw_packet.payload),
                        &self.context,
                    )?;
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!(
                            "Disconnected by server in the configuration phase: {}",
                            packet.reason
                        ),
                    ));
                }
                id if id == CFinishConfigurationPacket::id(&self.context) => {
                    debug!(
                        "Received Finish Configuration packet. Replying with Acknowledge Finish Configuration Packet"
                    );
                    // Need to answer the pack we already have on the disk (none)
                    self.writer
                        .write_and_send_packet(&SFinishConfigurationPacket, &self.context)
                        .await?;

                    self.context.state = ProtocolState::Play;
                    return Ok(());
                }
                id if id == CKeepAlivePacket::id(&self.context) => {
                    let packet = CKeepAlivePacket::read(
                        &mut Cursor::new(&raw_packet.payload),
                        &self.context,
                    )?;
                    self.writer
                        .write_and_send_packet(&SKeepAlivePacket { id: packet.id }, &self.context)
                        .await?;
                }
                id if id == CKnownPackPacket::id(&self.context) => {
                    debug!("Received Select Know Pack packet. Replying with Know Pack Packet");
                    // Need to answer the pack we already have on the disk (none)
                    self.writer
                        .write_and_send_packet(&SKnownPackPacket, &self.context)
                        .await?;
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
