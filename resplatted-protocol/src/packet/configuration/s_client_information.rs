use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;

/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_(configuration)
pub struct ClientInformationPacket {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listing: bool,
    pub particles_status: ParticlesStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticlesStatus {
    All = 0,
    Decreased = 1,
    Minimal = 2,
}

impl Default for ClientInformationPacket {
    fn default() -> Self {
        Self {
            locale: "en_us".to_string(),
            view_distance: 2,
            chat_mode: ChatMode::Enabled,
            chat_colors: true,
            displayed_skin_parts: 0b0111111,
            main_hand: MainHand::Right,
            enable_text_filtering: false,
            allow_server_listing: false,
            particles_status: ParticlesStatus::All,
        }
    }
}

impl PacketWrite for ClientInformationPacket {
    const ID: i32 = 0x00;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.write_string(self.locale.as_str())?;
        buf.write_primitive_type(self.view_distance);
        buf.write_var_int(self.chat_mode as i32);
        buf.write_primitive_type(self.chat_colors);
        buf.write_primitive_type(self.displayed_skin_parts);
        buf.write_var_int(self.main_hand as i32);
        buf.write_primitive_type(self.enable_text_filtering);
        buf.write_primitive_type(self.allow_server_listing);
        buf.write_var_int(self.particles_status as i32);
        Ok(())
    }
}
