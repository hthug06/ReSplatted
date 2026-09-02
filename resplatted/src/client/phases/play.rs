use crate::client::core::MinecraftClient;
use log::debug;
use rand::RngExt;
use rand::rngs::SmallRng;
use resplatted_protocol::packet::common::c_disconnect::DisconnectPacket;
use resplatted_protocol::packet::common::c_keep_alive::CKeepAlivePacket;
use resplatted_protocol::packet::common::s_keep_alive::SKeepAlivePacket;
use resplatted_protocol::packet::play::s_chat_message::ChatMessagePacket;
use resplatted_protocol::packet::play::s_move_player_rot::MovePlayerRotPacket;
use resplatted_protocol::packet::{
    PacketRead,
    play::{
        c_sync_player_pos::SyncPlayerPos, s_accept_teleportation::AcceptTeleportationPacket,
        s_move_player_pos_rot::MovePlayerPosRotPacket,
    },
};
use std::io::{Cursor, Error, ErrorKind};
use std::sync::Arc;

impl MinecraftClient {
    /// Handle play phase between the client and the server
    pub async fn enter_game(&mut self, message: Option<Arc<String>>) -> std::io::Result<()> {
        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet id to know what packet we need to handle
            match raw_packet.id {
                id if id == CKeepAlivePacket::id(&self.context) => {
                    let packet = CKeepAlivePacket::read(
                        &mut Cursor::new(&raw_packet.payload),
                        &self.context,
                    )?;
                    debug!(
                        "Received Keep Alive packet with id: {}. Sending Keep Alive packet with the same id",
                        packet.id
                    );
                    self.writer
                        .write_and_send_packet(&SKeepAlivePacket { id: packet.id }, &self.context)
                        .await?;
                }
                // Disconnect packet
                id if id == DisconnectPacket::id(&self.context) => {
                    let packet = DisconnectPacket::read(
                        &mut Cursor::new(&raw_packet.payload),
                        &self.context,
                    )?;
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!("Disconnected by server in play phase: {}", packet.reason),
                    ));
                }
                id if id == SyncPlayerPos::id(&self.context) => {
                    let packet =
                        SyncPlayerPos::read(&mut Cursor::new(&raw_packet.payload), &self.context)?;
                    debug!(
                        "Received Sync Player Pos packet with teleport id: {}. Sending Accept Teleportation packet",
                        packet.teleport_id
                    );

                    self.writer
                        .write_and_send_packet(
                            &AcceptTeleportationPacket {
                                teleport_id: packet.teleport_id,
                            },
                            &self.context,
                        )
                        .await?;

                    self.writer
                        .write_and_send_packet(
                            &MovePlayerPosRotPacket {
                                x: packet.x,
                                feet_y: packet.y - 1.62,
                                z: packet.z,
                                yaw: packet.yaw,
                                pitch: packet.pitch,
                                flags: 0x01,
                            },
                            &self.context,
                        )
                        .await?;
                }
                // The time update packet.
                // This packet repeat every second, so we can use it to spam the chat lol and rotate the head to not get kicked
                0x71 => {
                    if let Some(message) = &message {
                        self.writer
                            .write_and_send_packet(
                                &ChatMessagePacket {
                                    message: Arc::clone(message),
                                },
                                &self.context,
                            )
                            .await?;
                    }

                    // TODO bot are kick for invalid movement
                    let mut rng: SmallRng = rand::make_rng();
                    self.writer
                        .write_and_send_packet(
                            &MovePlayerRotPacket {
                                yaw: rng.random_range(-179.0..=179.0),
                                pitch: rng.random_range(-89.0..=89.0),
                                flags: 0x01,
                            },
                            &self.context,
                        )
                        .await?;
                }

                // Error on the network or unimplemented packet
                _ => {
                    // If we want, we can stop the program here on an unimplemented packet.
                    // But right now, we just want to skip it so just a warn in the console is okay
                    /*return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Packet ID unknown in the play phase: 0x{:02X}",
                            raw_packet.id
                        ),
                    ));*/
                    debug!(
                        "Packet ID unknown in the play phase: 0x{:02X}",
                        raw_packet.id
                    );
                }
            }
        }
    }
}
