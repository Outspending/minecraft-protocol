use crate::{buffer::buffer::ByteBuf, FromNetwork, ToNetwork};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl ToNetwork for Position {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write_long(((self.x & 0x3FFFFFF) << 38) | ((self.z & 0x3FFFFFF) << 12) | (self.y & 0xFFF))
    }
}

impl FromNetwork for Position {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let value = buf.read_long();
        Self {
            x: value >> 38,
            y: value << 52 >> 52,
            z: value << 26 >> 38,
        }
    }
}