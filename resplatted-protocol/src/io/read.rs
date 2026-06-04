use std::io::{Error, ErrorKind, Read};

/// trait for reading primitive and custom type from a Minecraft packet
/// Read simple type like u8, or more complex like VariInt
pub trait MinecraftReadExt {
    /// Read a Minecraft VarInt
    fn read_var_int(&mut self) -> std::io::Result<i32>;
    /// Read an utf-8 String
    fn read_string(&mut self) -> std::io::Result<String>;
    /// Read an u16.
    /// Same as i16, but without the & 65536
    fn read_u16(&mut self) -> std::io::Result<u16>;

    /// Read an i32 (int)
    fn read_i32(&mut self) -> std::io::Result<i32>;

    /// Read an i64 (long)
    fn read_i64(&mut self) -> std::io::Result<i64>;

    /// Read a f32 (float)
    fn read_f32(&mut self) -> std::io::Result<f32>;

    /// Read a f64 (double)
    fn read_f64(&mut self) -> std::io::Result<f64>;
}

/// Implement the MinecraftReadExt trait for any type that implements std::io::Read
/// That way, we can use it for any reader, like Cursor<&[u8]>, BufReader<TcpStream>, etc.
impl<R: Read> MinecraftReadExt for R {
    /// Read a Minecraft VarInt
    /// Copied from https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong So it's should be safe.
    /// BUT just in case, we created tests
    fn read_var_int(&mut self) -> std::io::Result<i32> {
        let mut value: i32 = 0;
        let mut position: i32 = 0;
        let mut buf = [0u8; 1]; // Read bytes by bytes

        loop {
            self.read_exact(&mut buf)?;
            let current_byte = buf[0];

            value |= ((current_byte & 0x7F) as i32) << position;

            if (current_byte & 0x80) == 0 {
                break;
            }

            position += 7;
            if position >= 32 {
                return Err(Error::new(ErrorKind::InvalidData, "VarInt is too big"));
            }
        }

        Ok(value)
    }

    fn read_string(&mut self) -> std::io::Result<String> {
        // String size
        let length = self.read_var_int()? as usize;

        if length > 32_767 * 4 {
            // basic security
            return Err(Error::new(ErrorKind::InvalidData, "String too big"));
        }

        // Read the string entirely
        let mut bytes = vec![0u8; length];
        self.read_exact(&mut bytes)?;

        String::from_utf8(bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
    }

    fn read_u16(&mut self) -> std::io::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_i32(&mut self) -> std::io::Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_i64(&mut self) -> std::io::Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    fn read_f32(&mut self) -> std::io::Result<f32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    fn read_f64(&mut self) -> std::io::Result<f64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }
}

#[cfg(test)]
/// Test for the MinecraftReadExt trait.
/// We can see if the function are good because this is the base we were going to read packet
mod tests {
    use super::*;
    use std::io::Cursor;

    // VarInt Tests

    /// Test if a VarInt can be 0
    #[test]
    fn test_read_var_int_zero() {
        let data = vec![0x00];
        let mut cursor = Cursor::new(data);
        assert_eq!(cursor.read_var_int().unwrap(), 0);
    }

    /// Test if a VarInt can be 255.
    /// Enough for all value (0 - i32::MAX)
    #[test]
    fn test_read_var_int_255() {
        let data = vec![0xFF, 0x01];
        let mut cursor = Cursor::new(data);
        assert_eq!(cursor.read_var_int().unwrap(), 255);
    }

    /// Test negative VarInt
    #[test]
    fn test_read_var_int_negative() {
        let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F];
        let mut cursor = Cursor::new(data);
        assert_eq!(cursor.read_var_int().unwrap(), -1);
    }

    /// Test if we get the right error when the VarInt is too big
    #[test]
    fn test_read_var_int_too_big() {
        // 6 bytes here (max is 5)
        let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        let mut cursor = Cursor::new(data);

        let result = cursor.read_var_int();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "VarInt is too big");
    }

    // String tests

    /// Simply read hello from a buffer
    #[test]
    fn test_read_string_simple() {
        let data = vec![0x05, b'H', b'e', b'l', b'l', b'o'];
        let mut cursor = Cursor::new(data);

        assert_eq!(cursor.read_string().unwrap(), "Hello");
    }

    /// Read a complex string with UTF-8 characters (like é and emoji)
    #[test]
    fn test_read_string_complex_utf8() {
        let target_string = "Aé🔥";
        let bytes = target_string.as_bytes();

        let mut data = vec![bytes.len() as u8];
        data.extend_from_slice(bytes);

        let mut cursor = Cursor::new(data);
        assert_eq!(cursor.read_string().unwrap(), "Aé🔥");
    }

    /// Read a string that is too big and see if we get an error
    #[test]
    fn test_read_string_too_big() {
        // 200 000 in VarInt = [0xC0, 0x9A, 0x0C]
        let data = vec![0xC0, 0x9A, 0x0C, b'A'];
        let mut cursor = Cursor::new(data);

        let result = cursor.read_string();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "String too big");
    }

    #[test]
    fn test_read_string_with_data_after() {
        let data = vec![0x05, b'H', b'e', b'l', b'l', b'o', 0xC0, 0x9A, 0x0C];
        let mut cursor = Cursor::new(data);

        assert_eq!(cursor.read_string().unwrap(), "Hello");
        assert_eq!(cursor.read_var_int().unwrap(), 200_000);
    }

    // test u16

    /// Simply read a u16
    #[test]
    fn test_read_u16_max_port() {
        // try to read an u16
        // here, if we read an i16, it should be -1, but an u16 should be  65535
        let data = vec![0xFF, 0xFF];
        let mut cursor = Cursor::new(data);

        let result = cursor.read_u16().unwrap();

        assert_eq!(result, 65535);
    }
}
