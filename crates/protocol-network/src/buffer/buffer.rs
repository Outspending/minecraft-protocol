use simdnbt::owned::Nbt;
use uuid::Uuid;

use crate::identifier::Identifier;
use crate::{FromNetwork, ToNetwork};
use std::io::Cursor;
use std::io::{Read, Write};
use std::ops::Deref;

use super::varnum::{VarInt, VarLong};

macro_rules! register_buffer {
    {
        $name:ident,
        $(
            $type:ty => ($write:ident, $read:ident)
        ),*
    } => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub(crate) buf: Cursor<Vec<u8>>
        }

        impl Read for $name {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                self.buf.read(buf)
            }
        }

        impl Write for $name {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                self.buf.write(buf)
            }

            fn flush(&mut self) -> std::io::Result<()> {
                self.buf.flush()
            }
        }

        impl $name {
            $(
                pub fn $write(&mut self, value: $type) {
                    value.to_network(self);
                }

                pub fn $read(&mut self) -> $type {
                    <$type>::from_network(self)
                }
            )*
        }
    };
}

impl Deref for ByteBuf {
    type Target = Cursor<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl From<Vec<u8>> for ByteBuf {
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}

impl ByteBuf {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            buf: Cursor::new(buf),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            buf: Cursor::new(Vec::new()),
        }
    }

    pub fn get_ref(&self) -> &Vec<u8> {
        self.buf.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut Vec<u8> {
        self.buf.get_mut()
    }

    pub fn len(&self) -> usize {
        self.buf.get_ref().len()
    }

    pub fn get_cursor(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.buf
    }

    pub fn set_position(&mut self, position: u64) {
        self.get_cursor().set_position(position);
    }

    pub fn get_rest(&mut self) -> Vec<u8> {
        let mut rest = Vec::new();
        self.buf.read_to_end(&mut rest).unwrap();
        rest
    }

    pub fn write_to<T: ToNetwork>(&mut self, value: T) {
        value.to_network(self);
    }

    pub fn read_from<T: FromNetwork>(&mut self) -> T {
        T::from_network(self)
    }
}

register_buffer! {
    ByteBuf,

    bool => (write_bool, read_bool),
    u8 => (write_ubyte, read_ubyte),
    i8 => (write_byte, read_byte),
    u16 => (write_ushort, read_ushort),
    i16 => (write_short, read_short),
    u32 => (write_uint, read_uint),
    i32 => (write_int, read_int),
    u64 => (write_ulong, read_ulong),
    i64 => (write_long, read_long),
    f32 => (write_float, read_float),
    f64 => (write_double, read_double),
    String => (write_string, read_string),
    Uuid => (write_uuid, read_uuid),
    VarInt => (write_varint, read_varint),
    VarLong => (write_varlong, read_varlong),
    Nbt => (write_nbt, read_nbt),
    Identifier => (write_identifier, read_identifier)
}
