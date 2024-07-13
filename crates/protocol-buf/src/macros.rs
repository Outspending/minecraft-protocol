#[macro_export]
macro_rules! register_buffer {
    {
        $buf_name:ident,
        $( $buf_type:ty => ($read:ident, $write:ident) ),*
    } => {
        pub trait $buf_name {
            fn write<T: ToNetwork>(&mut self, buf: T);
            fn read<T: FromNetwork>(&mut self) -> T;

            fn get_ref(&self) -> &Vec<u8>;
            fn get_mut(&mut self) -> &mut Vec<u8>;

            $(
                fn $read(&mut self) -> $buf_type {
                    self.read::<$buf_type>()
                }

                fn $write(&mut self, value: $buf_type) {
                    self.write::<$buf_type>(value);
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! register_varnum {
    ($name:ident, $varnum_type:ty, $working_type:ty, $max_size:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name {
            pub value: $varnum_type,
        }

        impl Deref for $name {
            type Target = $varnum_type;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl From<$varnum_type> for $name {
            fn from(value: $varnum_type) -> Self {
                Self { value }
            }
        }

        impl ToNetwork for $name {
            fn to_network(&self) -> Vec<u8> {
                let mut value = self.value as $working_type;
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
        }

        impl FromNetwork for $name {
            fn from_network(buffer: &mut Cursor<Vec<u8>>) -> Self {
                let mut value = 0;
                let mut size = 0;

                loop {
                    let byte = buffer.get_ref()[buffer.position() as usize];
                    buffer.set_position(buffer.position() + 1);

                    value |= ((byte & 0b01111111) as $working_type) << (7 * size);
                    size += 1;

                    if size > $max_size {
                        panic!("VarInt too large");
                    }

                    if byte & 0b10000000 == 0 {
                        break;
                    }
                }

                Self {
                    value: value as $varnum_type,
                }
            }
        }

        impl $name {
            pub fn len(&self) -> usize {
                let mut value = self.value as $working_type;
                let mut len = 0;

                loop {
                    value >>= 7;
                    len += 1;

                    if value == 0 {
                        break;
                    }
                }

                len
            }
        }
    };
}

#[macro_export]
macro_rules! handle_primitive_read {
    ($buffer:expr, $type:ty, $bytes:literal) => {{
        let mut bytes = [0; $bytes];
        $buffer
            .read_exact(&mut bytes)
            .expect("Failed to read bytes");
        <$type>::from_be_bytes(bytes)
    }};
}

#[macro_export]
macro_rules! handle_primitive_type {
    ($type:ty, $size:literal) => {
        impl ToNetwork for $type {
            fn to_network(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }
        }

        impl FromNetwork for $type {
            fn from_network(buffer: &mut Cursor<Vec<u8>>) -> Self {
                handle_primitive_read!(buffer, $type, $size)
            }
        }
    };
}
