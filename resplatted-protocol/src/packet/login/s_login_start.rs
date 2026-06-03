use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;
use uuid::Uuid;

pub struct LoginStartPacket {
    pub username: String,
    pub uuid: Uuid,
}

impl LoginStartPacket {
    pub fn new(username: String) -> LoginStartPacket {
        Self {
            username,
            uuid: Uuid::new_v4(),
        }
    }
}
impl PacketWrite for LoginStartPacket {
    const ID: i32 = 0x00;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.write_string(&self.username)?;
        buf.write_uuid(self.uuid);

        Ok(())
    }
}
