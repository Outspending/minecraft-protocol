use std::{
    io::{Cursor, Read},
    ops::Deref,
};

use crate::{
    handle_primitive_read, handle_primitive_type, register_varnum, FromNetwork, ToNetwork,
};

impl ToNetwork for bool {
    fn to_network(&self) -> Vec<u8> {
        (*self as u8).to_network()
    }
}

impl FromNetwork for bool {
    fn from_network(buffer: &mut Cursor<Vec<u8>>) -> Self {
        u8::from_network(buffer) != 0
    }
}

impl ToNetwork for u8 {
    fn to_network(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl FromNetwork for u8 {
    fn from_network(buffer: &mut Cursor<Vec<u8>>) -> Self {
        buffer.get_ref()[buffer.position() as usize]
    }
}

impl ToNetwork for String {
    fn to_network(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let length = VarInt::from(self.len() as i32);

        bytes.extend_from_slice(&length.to_network());
        bytes.extend_from_slice(self.as_bytes());
        bytes
    }
}

impl FromNetwork for String {
    fn from_network(buffer: &mut Cursor<Vec<u8>>) -> Self {
        let length = *VarInt::from_network(buffer) as usize;
        let bytes = &buffer.get_ref()[buffer.position() as usize..];
        let string = String::from_utf8(bytes[..length].to_vec()).unwrap();

        buffer.set_position(buffer.position() + length as u64);
        string
    }
}

handle_primitive_type!(u16, 2);
handle_primitive_type!(u32, 4);
handle_primitive_type!(u64, 8);
handle_primitive_type!(f32, 4);
handle_primitive_type!(f64, 8);

register_varnum!(VarInt, i32, u32, 5);
register_varnum!(VarLong, i64, u64, 10);

pub(crate) fn encode_varint(mut value: i32) -> Vec<u8> {
    let mut bytes = Vec::new();

    loop {
        let mut byte = (value & 0b01111111) as u8;
        value >>= 7;

        if value != 0 {
            byte |= 0b10000000;
        }

        bytes.push(byte);

        if value == 0 {
            break;
        }
    }

    bytes
}
