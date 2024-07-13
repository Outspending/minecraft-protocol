use std::io::{Cursor, Write};

use thiserror::Error;

use crate::{
    compression::CompressionData,
    register_buffer,
    types::{VarInt, VarLong},
    FromNetwork, ToNetwork,
};

/// Errors that can occur when reading or writing to a buffer.
///
/// This enum is used as the error type for the `Buffer` trait.
///
/// # Examples
/// ```rust
/// use buffer::BufferError;
///
/// fn main() -> Result<(), BufferError> {
///    Err(BufferError::InsufficientData)
/// }
/// ```
///
/// # Variants
///
/// - `VarIntOverflow` - The VarInt is too large to be read.
/// - `InsufficientData` - There is not enough data in the buffer to read.
/// - `Utf8Error` - The data in the buffer is not valid UTF-8.
/// - `BadPacketId` - The packet ID is not valid.
/// - `BadPacketLength` - The packet length is not valid.
///
#[derive(Debug, Error)]
pub enum BufferError {
    #[error("VarInt overflow")]
    VarIntOverflow,
    #[error("Not enough data in buffer")]
    InsufficientData,
    #[error("Invalid UTF-8 sequence")]
    Utf8Error,
    #[error("Invalid packet ID")]
    BadPacketId,
    #[error("Invalid packet length")]
    BadPacketLength,
}

/// A type alias for a `Result` that uses `BufferError` as the error type.
///
/// # Examples
/// ```rust
/// use buffer::BufferResult;
///
/// fn main() -> BufferResult<()> {
///    Err(BufferError::InsufficientData)
/// }
/// ```
///
pub type BufferResult<T> = Result<T, BufferError>;

register_buffer! {
    Buffer,

    bool => (read_bool, write_bool),
    u8 => (read_byte, write_byte),
    u16 => (read_short, write_short),
    u32 => (read_int, write_int),
    u64 => (read_long, write_long),
    f32 => (read_float, write_float),
    f64 => (read_double, write_double),
    String => (read_string, write_string),
    VarInt => (read_varint, write_varint),
    VarLong => (read_varlong, write_varlong)
}

/// Represents a buffer that can be read from and written to.
///
/// This buffer has no data for packets. This can be used for a less complex buffer.
/// Instead of grabbing data from the buffer while creating the packet, the data is passed in.
///
/// # Examples
/// ```rust
/// use buffer::NormalBuffer;
///
/// let buffer = NormalBuffer::new(vec![0x01, 0x02, 0x03]);
/// ```
///
/// # Fields
///
/// - `buffer` - The buffer that contains the data.
#[derive(Debug, Clone)]
pub struct NormalBuffer {
    pub buffer: Cursor<Vec<u8>>,
}

impl Buffer for NormalBuffer {
    /// Reads a value from the `[NormalBuffer]` buffer. This function is used to read data from the buffer.
    ///
    /// Any type that implements the `FromNetwork` trait can be read from the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::NormalBuffer;
    ///
    /// let mut buffer = NormalBuffer::new(vec![0x01, 0x02, 0x03]);
    /// let value: u8 = buffer.read();
    /// ```
    fn read<T: FromNetwork>(&mut self) -> T {
        T::from_network(&mut self.buffer)
    }

    /// Writes a value to the `[NormalBuffer]` buffer. This function is used to write data to the buffer.
    ///
    /// Any type that implements the `ToNetwork` trait can be written to the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::NormalBuffer;
    ///
    /// let mut buffer = NormalBuffer::new(Vec::new());
    /// buffer.write(0x01);
    /// ```
    fn write<T: ToNetwork>(&mut self, buf: T) {
        self.buffer.write_all(&buf.to_network()).unwrap();
    }

    /// Returns a reference to the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::NormalBuffer;
    ///
    /// let buffer = NormalBuffer::new(vec![0x01, 0x02, 0x03]);
    ///
    /// assert_eq!(buffer.get_ref(), &[0x01, 0x02, 0x03]);
    /// ```
    ///
    /// # Returns
    /// A reference to the buffer.
    fn get_ref(&self) -> &Vec<u8> {
        self.buffer.get_ref()
    }

    /// Returns a mutable reference to the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::NormalBuffer;
    ///
    /// let mut buffer = NormalBuffer::new(vec![0x01, 0x02, 0x03]);
    /// buffer.get_mut().push(0x04);
    ///
    /// assert_eq!(buffer.get_ref(), &[0x01, 0x02, 0x03, 0x04]);
    /// ```
    ///
    /// # Returns
    /// A mutable reference to the buffer.
    fn get_mut(&mut self) -> &mut Vec<u8> {
        self.buffer.get_mut()
    }
}

impl NormalBuffer {
    /// Creates a new `NormalBuffer` with the given data.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::NormalBuffer;
    ///
    /// let buffer = NormalBuffer::new(vec![0x01, 0x02, 0x03]);
    ///
    /// assert_eq!(buffer.buffer.get_ref(), &[0x01, 0x02, 0x03]);
    /// ```
    pub const fn new(buffer: Vec<u8>) -> Self {
        Self {
            buffer: Cursor::new(buffer),
        }
    }
}

impl From<Vec<u8>> for NormalBuffer {
    /// Creates a new `NormalBuffer` with the given data.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::NormalBuffer;
    ///
    /// let buffer = NormalBuffer::from(vec![0x01, 0x02, 0x03]);
    ///
    /// assert_eq!(buffer.buffer.get_ref(), &[0x01, 0x02, 0x03]);
    /// ```
    fn from(buffer: Vec<u8>) -> Self {
        Self::new(buffer)
    }
}

/// A buffer that defines the data needed to read and write packets.
///
/// This struct is used to read and write packets to and from `[Buffer]`
///
/// # Examples
/// ```rust
/// use buffer::PacketBuffer;
///
/// let buffer = PacketBuffer::new(CompressionData::new(256, CompressionType::Zlib));
/// ```
///
/// # Fields
///
/// - `compression` - The data needed to compress / decompress packets.
/// - `packet_length` - The length of the packet.
/// - `data_length` - The length of the data.
/// - `packet_id` - The ID of the packet.
/// - `buffer` - The buffer that contains the data.
#[derive(Debug, Clone)]
pub struct PacketBuffer {
    pub packet_length: VarInt,
    pub data_length: VarInt,
    pub packet_id: VarInt,
    pub buffer: NormalBuffer,
}

impl Buffer for PacketBuffer {
    /// Reads a value from the `[PacketBuffer]` buffer. Although its just a wrapper for the `[NormalBuffer]` buffer.
    /// Its just calling the `read` function from the `[NormalBuffer]` buffer.
    ///
    /// Any type that implements the `FromNetwork` trait can be read from the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::PacketBuffer;
    ///
    /// let mut buffer = PacketBuffer::new(CompressionData::new(256, CompressionType::Zlib));
    /// let value: u8 = buffer.read();
    ///
    /// assert_eq!(value, 0x01);
    /// ```
    fn read<T: FromNetwork>(&mut self) -> T {
        self.buffer.read()
    }

    /// Writes a value to the `[PacketBuffer]` buffer. Although its just a wrapper for the `[NormalBuffer]` buffer.
    /// Its just calling the `write` function from the `[NormalBuffer]` buffer.
    ///
    /// Any type that implements the `ToNetwork` trait can be written to the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::PacketBuffer;
    ///
    /// let mut buffer = PacketBuffer::new(CompressionData::new(256, CompressionType::Zlib));
    ///
    /// buffer.write(0x01);
    /// ```
    fn write<T: ToNetwork>(&mut self, buf: T) {
        self.buffer.write(buf);
    }

    /// Returns a reference to the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::PacketBuffer;
    ///
    /// let buffer = PacketBuffer::new(CompressionData::new(256, CompressionType::Zlib), vec![0x01, 0x02, 0x03]);
    ///
    /// assert_eq!(buffer.get_ref(), &[0x01, 0x02, 0x03]);
    /// ```
    ///
    /// # Returns
    /// A reference to the buffer.
    fn get_ref(&self) -> &Vec<u8> {
        self.buffer.get_ref()
    }

    /// Returns a mutable reference to the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::PacketBuffer;
    ///
    /// let mut buffer = PacketBuffer::new(CompressionData::new(256, CompressionType::Zlib), vec![0x01, 0x02, 0x03]);
    /// buffer.get_mut().push(0x04);
    ///
    /// assert_eq!(buffer.get_mut(), &mut vec![0x01, 0x02, 0x03]);
    /// ```
    ///
    /// # Returns
    /// A mutable reference to the buffer.
    fn get_mut(&mut self) -> &mut Vec<u8> {
        self.buffer.get_mut()
    }
}

impl PacketBuffer {
    /// Creates a new `PacketBuffer` with the given data.
    ///
    /// Given the compression data, the packet length, the packet ID and the buffer.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::PacketBuffer;
    ///
    /// let buffer = PacketBuffer::new(CompressionData::new(256, CompressionType::Zlib), NormalBuffer::new(vec![0x01, 0x02, 0x03]));
    ///
    /// assert_eq!(buffer.get_ref(), &[0x01, 0x02, 0x03]);
    /// ```
    ///
    /// # Returns
    /// A new `PacketBuffer`. If the buffer had an error, it will return `None`.
    pub fn new(buffer: Vec<u8>, compression: &CompressionData) -> Option<Self> {
        if let Ok(data) = compression.grab_from_buffer(buffer, compression) {
            Some(data)
        } else {
            None
        }
    }
}
