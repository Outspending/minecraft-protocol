use crate::{ByteBuf, FromNetwork, ToNetwork};
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::ops::Deref;

macro_rules! register_varnum {
    ( $name:ident, $type:ty, $working_type:ty, $max_size:literal ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub $type);

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }

        impl FromNetwork for $name {
            fn from_network(buf: &mut ByteBuf) -> Self {
                let mut result = 0u64;
                let mut shift = 0;

                loop {
                    let mut byte = [0u8];
                    buf.read(&mut byte).unwrap();
                    let byte = byte[0];

                    result |= ((byte & 0x7F) as u64) << shift;

                    if byte & 0x80 == 0 {
                        break;
                    }
                    shift += 7;
                }

                $name::from(result as $type)
            }
        }

        impl ToNetwork for $name {
            fn to_network(&self, buf: &mut ByteBuf) {
                let mut value = self.0 as $working_type;
                while value >= 0x80 {
                    buf.write(&[(value as u8) | 0x80]).unwrap();
                    value >>= 7;
                }
                buf.write(&[value as u8]).unwrap();
            }
        }

        impl $name {
            pub fn get_size_in_bytes(&self) -> usize {
                let mut value = self.0 as $working_type;
                let mut size = 0;
                while value >= 0x80 {
                    size += 1;
                    value >>= 7;
                }
                size + 1
            }
        }
    };
}

register_varnum!(VarInt, i32, u32, 5);
register_varnum!(VarLong, i64, u64, 10);
