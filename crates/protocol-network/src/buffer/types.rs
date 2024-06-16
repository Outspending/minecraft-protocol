use std::io::{Read, Write};

use simdnbt::owned::Nbt;
use uuid::Uuid;

use crate::{identifier::Identifier, FromNetwork, ToNetwork};

use super::{buffer::ByteBuf, varnum::VarInt};

impl ToNetwork for bool {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&[if *self { 1 } else { 0 }]).unwrap();
    }
}

impl FromNetwork for bool {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 1];
        buf.read(&mut buffer).unwrap();
        buffer[0] != 0
    }
}

impl ToNetwork for u8 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&[*self]).unwrap();
    }
}

impl FromNetwork for u8 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 1];
        buf.read(&mut buffer).unwrap();
        buffer[0]
    }
}

impl ToNetwork for i8 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&[*self as u8]).unwrap();
    }
}

impl FromNetwork for i8 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 1];
        buf.read(&mut buffer).unwrap();
        buffer[0] as i8
    }
}

impl ToNetwork for i16 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork for i16 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 2];
        buf.read(&mut buffer).unwrap();
        i16::from_be_bytes(buffer)
    }
}

impl ToNetwork for u16 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork for u16 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 2];
        buf.read(&mut buffer).unwrap();
        u16::from_be_bytes(buffer)
    }
}

impl ToNetwork for i32 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork for i32 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 4];
        buf.read(&mut buffer).unwrap();
        i32::from_be_bytes(buffer)
    }
}

impl ToNetwork for u32 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork for u32 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 4];
        buf.read(&mut buffer).unwrap();
        u32::from_be_bytes(buffer)
    }
}

impl ToNetwork for i64 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork for i64 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 8];
        buf.read(&mut buffer).unwrap();
        i64::from_be_bytes(buffer)
    }
}

impl ToNetwork for u64 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork for u64 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 8];
        buf.read(&mut buffer).unwrap();
        u64::from_be_bytes(buffer)
    }
}

impl ToNetwork for f32 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork for f32 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 4];
        buf.read(&mut buffer).unwrap();
        f32::from_be_bytes(buffer)
    }
}

impl ToNetwork for f64 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork for f64 {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 8];
        buf.read(&mut buffer).unwrap();
        f64::from_be_bytes(buffer)
    }
}

impl ToNetwork for String {
    fn to_network(&self, buf: &mut ByteBuf) {
        let bytes = self.as_bytes();
        let length = VarInt::from(bytes.len() as i32);

        buf.write_varint(length);
        buf.write(bytes).unwrap();
    }
}

impl FromNetwork for String {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let length = VarInt::from_network(buf).0 as usize;
        let mut bytes = vec![0_u8; length];

        buf.read(&mut bytes).unwrap();
        String::from_utf8(bytes).unwrap()
    }
}

impl ToNetwork for Uuid {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write(self.as_bytes()).unwrap();
    }
}

impl FromNetwork for Uuid {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let mut buffer = [0_u8; 16];
        buf.read(&mut buffer).unwrap();
        Uuid::from_bytes(buffer)
    }
}

impl<T: ToNetwork> ToNetwork for Vec<T> {
    fn to_network(&self, buf: &mut ByteBuf) {
        let length = VarInt::from(self.len() as i32);
        buf.write_varint(length);

        for value in self {
            value.to_network(buf);
        }
    }
}

impl<T: FromNetwork> FromNetwork for Vec<T> {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let length = *buf.read_varint() as usize;
        let mut values = Vec::with_capacity(length);

        for _ in 0..length {
            values.push(T::from_network(buf));
        }

        values
    }
}


impl ToNetwork for Nbt {
    fn to_network(&self, buf: &mut ByteBuf) {
        self.write_unnamed(buf.get_mut());
        buf.set_position(buf.len() as u64 + 1);
    }
}

impl FromNetwork for Nbt {
    fn from_network(buf: &mut ByteBuf) -> Self {
        todo!()
    }
}

impl<'a> ToNetwork for Identifier<'a> {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write_string(format!("{}", self));
    }
}

impl<'a> FromNetwork for Identifier<'a> {
    fn from_network(buf: &mut ByteBuf) -> Self {
        todo!()
    }
}

impl<T: ToNetwork> ToNetwork for Option<T> {
    fn to_network(&self, buf: &mut ByteBuf) {
        match self {
            Some(value) => {
                value.to_network(buf);
            },
            None => ()
        }
    }
}

impl<T: FromNetwork> FromNetwork for Option<T> {
    fn from_network(buf: &mut ByteBuf) -> Self {
        todo!()
    }
}