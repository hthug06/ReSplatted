use crate::define_phase_packets;
use crate::io::ConnectionContext;
use crate::packet::common::c_disconnect::DisconnectPacket;
use crate::packet::common::c_keep_alive::CKeepAlivePacket;
use crate::packet::{PacketRead, RawPacket};

mod c_set_time;
mod c_sync_player_pos;
pub mod s_accept_teleportation;
pub mod s_chat_message;
pub mod s_move_player_pos_rot;
pub mod s_move_player_rot;
pub mod s_player_loaded;

define_phase_packets!(PlayPacket {
    KeepAlive(CKeepAlivePacket),
    Disconnect(DisconnectPacket),
    SyncPlayerPos(c_sync_player_pos::SyncPlayerPosPacket),
    SetTime(c_set_time::SetTimePacket),
});
