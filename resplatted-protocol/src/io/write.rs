use crate::io::write_primitive::WritePrimitive;
use uuid::Uuid;

pub trait MinecraftWriteExt {
    fn write_var_int(&mut self, value: i32);
    fn write_string(&mut self, text: &str) -> std::io::Result<()>;
    fn write_primitive_type<T: WritePrimitive>(&mut self, value: T);
    fn write_uuid(&mut self, uuid: Uuid);
}

impl MinecraftWriteExt for Vec<u8> {
    /// Write a Minecraft VarInt Into the Vec
    fn write_var_int(&mut self, value: i32) {
        // Cast to u32 to perform a logical bit shift rather than an arithmetic one
        let mut val = value as u32;
        loop {
            if (val & !0x7F) == 0 {
                self.push(val as u8);
                break;
            }
            self.push(((val & 0x7F) | 0x80) as u8);
            val >>= 7;
        }
    }

    /// Write a String (length + text). Also, it's UTF-8
    fn write_string(&mut self, text: &str) -> std::io::Result<()> {
        let bytes = text.as_bytes(); // UTF-8

        if bytes.len() > 32_767 * 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "String too big",
            ));
        }

        // first size
        self.write_var_int(bytes.len() as i32);
        // then text (bytes)
        self.extend_from_slice(bytes);
        Ok(())
    }

    /// Write a primitive (i.., u.., f.., bool) type into the buffer
    fn write_primitive_type<T: WritePrimitive>(&mut self, value: T) {
        value.write_to(self);
    }

    /// Write a UUID in the buffer
    fn write_uuid(&mut self, uuid: Uuid) {
        self.extend_from_slice(uuid.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Try to write a VarInt with the value 0
    #[test]
    fn test_write_varint_zero() {
        let mut buf = Vec::new();
        buf.write_var_int(0);
        assert_eq!(buf, &[0x00]);
    }

    /// Try to write a VarInt with the Value 255
    #[test]
    fn test_write_varint_255() {
        let mut buf = Vec::new();
        buf.write_var_int(255);
        assert_eq!(buf, &[0xFF, 0x01]);
    }

    /// Try to write a VarInt with the max value of an i32
    #[test]
    fn test_write_varint_max() {
        let mut buf = Vec::new();
        buf.write_var_int(2147483647); // i32::MAX
        assert_eq!(buf, &[0xFF, 0xFF, 0xFF, 0xFF, 0x07]);
    }

    /// Try to write a negative VarInt
    #[test]
    fn test_write_varint_negative() {
        let mut buf = Vec::new();
        buf.write_var_int(-1);
        assert_eq!(buf, &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
    }

    /// Write a string into the buffer (size + text)
    #[test]
    fn test_write_string_simple() {
        let mut buf = Vec::new();
        buf.write_string("Hello").unwrap();
        assert_eq!(buf, &[0x05, b'H', b'e', b'l', b'l', b'o']);
    }

    /// Write a complex string (with é and emoji) into the buffer
    #[test]
    fn test_write_string_complex_utf8() {
        let mut buf = Vec::new();
        buf.write_string("Aé🔥").unwrap();

        // "Aé🔥" is 3 char but 7 bytes in UTF-8.
        assert_eq!(buf[0], 0x07);
        // So size should be 8
        assert_eq!(buf.len(), 8);
    }

    /// Attempt to write a String that's too big
    #[test]
    fn test_write_string_too_big() {
        let mut buf = Vec::new();
        // Big string
        // Testing with an emoji because it takes the most byte
        // Here, if we want to try with just a letter, we should repeat it (32_767 * 4) +1
        let big_string = "🔥".repeat(32_768);

        let result = buf.write_string(&big_string);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "String too big");
    }
}
