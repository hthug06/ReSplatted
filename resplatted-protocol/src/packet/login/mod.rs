use super::common::c_disconnect::DisconnectPacket;
use crate::define_phase_packets;
use crate::io::ConnectionContext;
use crate::packet::{PacketRead, RawPacket};

mod c_encryption_request;
mod c_login_success;
mod c_set_compression;
pub mod s_login_acknowledged;
pub mod s_login_start;

define_phase_packets!(LoginPacket {
    Disconnect(DisconnectPacket),
    EncryptionRequest(c_encryption_request::EncryptionRequestPacket),
    LoginSuccess(c_login_success::LoginSuccessPacket),
    SetCompression(c_set_compression::SetCompressionPacket),
});
