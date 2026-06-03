/// Write every primitive type (i.., f..., u.., bool) into a buffer
pub trait WritePrimitive {
    fn write_to(self, buf: &mut Vec<u8>);
}

impl WritePrimitive for bool {
    fn write_to(self, buf: &mut Vec<u8>) {
        buf.push(if self { 1 } else { 0 });
    }
}

impl WritePrimitive for f32 {
    fn write_to(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl WritePrimitive for f64 {
    fn write_to(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl WritePrimitive for i8 {
    fn write_to(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl WritePrimitive for i64 {
    fn write_to(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl WritePrimitive for u8 {
    fn write_to(self, buf: &mut Vec<u8>) {
        buf.push(self);
    }
}

impl WritePrimitive for u16 {
    fn write_to(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}