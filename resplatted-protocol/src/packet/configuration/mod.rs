use crate::define_phase_packets;
use crate::io::ConnectionContext;
use crate::packet::common::c_disconnect::DisconnectPacket;
use crate::packet::common::c_keep_alive::CKeepAlivePacket;
use crate::packet::{PacketRead, RawPacket};

mod c_finish_configuration;
mod c_know_pack;
mod c_plugin_message;
pub mod s_client_information;
pub mod s_finish_configuration;
pub mod s_known_pack;

define_phase_packets!(ConfigurationPacket {
    PluginMessage(c_plugin_message::PluginMessagePacket),
    Disconnect(DisconnectPacket),
    FinishConfiguration(c_finish_configuration::CFinishConfigurationPacket),
    KeepAlive(CKeepAlivePacket),
    KnownPack(c_know_pack::CKnownPackPacket),
});
