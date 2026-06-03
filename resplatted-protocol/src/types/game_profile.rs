use crate::io::{read::MinecraftReadExt, write::MinecraftWriteExt};
use std::io::{Error, Read};
use uuid::Uuid;

/// A game profile with infos of the player
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Profile
#[derive(Debug)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<GameProfileProperties>,
}

#[derive(Debug)]
pub struct GameProfileProperties {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl GameProfile {
    /// Read a GameProfile from flux
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        // first the Uuid
        let mut uuid_bytes = [0u8; 16];
        reader.read_exact(&mut uuid_bytes)?;
        let uuid = Uuid::from_bytes(uuid_bytes);

        // Then the username
        let username = reader.read_string()?;

        // After that, the properties (skins, textures...)
        // This is a Prefixed Array, the prefix is the size
        let property_count = reader.read_var_int()?;

        // 16 should be more than okay because 99% of the time their just 'textures'
        // + texture is for premium server, so in cracked server there is no properties 99% of the time
        if !(0..=16).contains(&property_count) {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Nombre de propriétés GameProfile invalide : {}",
                    property_count
                ),
            ));
        }

        let mut properties: Vec<GameProfileProperties> =
            Vec::with_capacity(property_count as usize);

        for _ in 0..property_count {
            let prop_name = reader.read_string()?;
            let prop_value = reader.read_string()?;

            // read 'is signed'
            let mut is_signed_buf = [0u8; 1];
            reader.read_exact(&mut is_signed_buf)?;
            let is_signed = is_signed_buf[0] != 0;

            let signature = if is_signed {
                Some(reader.read_string()?)
            } else {
                None
            };

            properties.push(GameProfileProperties {
                name: prop_name,
                value: prop_value,
                signature,
            });
        }

        Ok(Self {
            uuid,
            username,
            properties,
        })
    }

    /// Write a gameprofile into a buffer
    pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        buf.write_uuid(self.uuid);
        buf.write_string(&self.username)?;

        buf.write_var_int(self.properties.len() as i32);
        for prop in &self.properties {
            buf.write_string(&prop.name)?;
            buf.write_string(&prop.value)?;

            if let Some(sig) = &prop.signature {
                buf.write_primitive_type(1u8); // is_signed = true
                buf.write_string(sig)?;
            } else {
                buf.write_primitive_type(0u8); // is_signed = false
            }
        }
        Ok(())
    }
}
