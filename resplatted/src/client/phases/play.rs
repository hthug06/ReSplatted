use crate::client::core::MinecraftClient;
use log::debug;
use rand::RngExt;
use rand::rngs::SmallRng;
use resplatted_protocol::packet::common::s_keep_alive::SKeepAlivePacket;
use resplatted_protocol::packet::play::PlayPacket;
use resplatted_protocol::packet::play::s_chat_message::ChatMessagePacket;
use resplatted_protocol::packet::play::s_move_player_rot::MovePlayerRotPacket;
use resplatted_protocol::packet::play::{
    s_accept_teleportation::AcceptTeleportationPacket,
    s_move_player_pos_rot::MovePlayerPosRotPacket,
};
use std::io::{Error, ErrorKind};
use std::sync::Arc;

impl MinecraftClient {
    /// Handle play phase between the client and the server
    pub async fn enter_game(&mut self, message: Option<Arc<String>>) -> std::io::Result<()> {
        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet to know what packet we need to handle
            match PlayPacket::parse(&raw_packet, &self.context)? {
                PlayPacket::KeepAlive(packet) => {
                    debug!(
                        "Received Keep Alive packet with id: {}. Sending Keep Alive packet with the same id",
                        packet.id
                    );
                    self.writer
                        .write_and_send_packet(&SKeepAlivePacket { id: packet.id }, &self.context)
                        .await?;
                }
                PlayPacket::Disconnect(packet) => {
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!("Disconnected by server in play phase: {}", packet.reason),
                    ));
                }
                PlayPacket::SyncPlayerPos(packet) => {
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
                PlayPacket::SetTime(_) => {
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

                    let mut rng: SmallRng = rand::make_rng();
                    self.writer
                        .write_and_send_packet(
                            &MovePlayerRotPacket {
                                yaw: rng.random_range(-10.0..=10.0),
                                pitch: rng.random_range(-10.0..=10.0),
                                flags: 0x01,
                            },
                            &self.context,
                        )
                        .await?;
                }
                // Error on the network or unimplemented packet
                PlayPacket::Unknown(id) => {
                    debug!("Packet ID unknown in the play phase: 0x{:02X}", id);
                }
            }
        }
    }
}
