use crate::client::core::MinecraftClient;
use log::debug;
use resplatted_protocol::packet::{
    PacketRead,
    play::{
        c_disconnect::PlayDisconnectPacket, c_keep_alive::CPlayKeepAlivePacket,
        c_sync_player_pos::SyncPlayerPos, s_accept_teleportation::AcceptTeleportationPacket,
        s_keep_alive::SPlayKeepAlivePacket, s_move_player_pos_rot::MovePlayerPosRotPacket,
    },
};
use std::io::{Cursor, Error, ErrorKind};

impl MinecraftClient {
    /// Handle play phase between the client and the server
    pub async fn enter_game(&mut self) -> std::io::Result<()> {
        // read loop
        loop {
            // read the raw packet
            let raw_packet = self.reader.read_packet().await?;

            // Match the packet id to know what packet we need to handle
            match raw_packet.id {
                CPlayKeepAlivePacket::ID => {
                    let packet = CPlayKeepAlivePacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    debug!(
                        "Received Keep Alive packet with id: {}. Sending Keep Alive packet with the same id",
                        packet.id
                    );
                    self.writer
                        .write_and_send_packet(&SPlayKeepAlivePacket { id: packet.id })
                        .await?;
                }
                // Disconnect packet
                PlayDisconnectPacket::ID => {
                    let packet = PlayDisconnectPacket::read(&mut Cursor::new(&raw_packet.payload))?;
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        format!("Disconnected by server in play phase: {}", packet.reason),
                    ));
                }
                SyncPlayerPos::ID => {
                    let packet = SyncPlayerPos::read(&mut Cursor::new(&raw_packet.payload))?;
                    debug!(
                        "Received Sync Player Pos packet with teleport id: {}. Sending Accept Teleportation packet",
                        packet.teleport_id
                    );

                    self.writer
                        .write_and_send_packet(&AcceptTeleportationPacket {
                            teleport_id: packet.teleport_id,
                        })
                        .await?;

                    self.writer
                        .write_and_send_packet(&MovePlayerPosRotPacket {
                            x: packet.x,
                            feet_y: packet.y - 1.62,
                            z: packet.z,
                            yaw: packet.yaw,
                            pitch: packet.pitch,
                            flags: 0x01,
                        })
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
