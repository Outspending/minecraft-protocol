use std::io::Write;

use flate2::write::ZlibEncoder;

use crate::{
    buffer::{Buffer, BufferResult, NormalBuffer, PacketBuffer},
    types::{encode_varint, VarInt},
    ToNetwork,
};

/// Defines the compression types that can be used to compress / decompress packets.
///
/// # Variants
///
/// - `None` - No compression is used.
/// - `Zlib` - Zlib compression is used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Zlib,
}

/// Represents the result of compressing / decompressing a packet.
///
/// This is a type alias for a `BufferResult` with a `PacketBuffer` containing the compressed / decompressed packet.
pub type CompressionResult<B: Buffer> = BufferResult<B>;

/// Contains the data needed to compress / decompress packets.
///
/// # Fields
///
/// - `threshold` - The threshold at which packets should be compressed.
/// - `compression_type` - The type of compression to use.
///
/// # Examples
/// ```rust
/// use buffer::CompressionData;
///
/// let data = CompressionData {
///    threshold: 256,
///   compression_type: CompressionType::Zlib,
/// };
///
/// assert_eq!(data.threshold, 256);
/// assert_eq!(data.compression_type, CompressionType::Zlib);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CompressionData {
    pub threshold: i32,
    pub compression_type: CompressionType,
}

impl CompressionData {
    /// Creates a new `CompressionData` with the given threshold and compression type.
    ///
    /// # Examples
    /// ```rust
    /// use buffer::{CompressionData, CompressionType};
    ///
    /// let data = CompressionData::new(256, CompressionType::Zlib);
    /// ```
    ///
    /// # Parameters
    /// - `threshold` - The threshold at which packets should be compressed.
    /// - `compression_type` - The type of compression to use.
    ///
    /// # Returns
    /// A new `CompressionData` instance.
    pub const fn new(threshold: i32, compression_type: CompressionType) -> Self {
        Self {
            threshold,
            compression_type,
        }
    }

    /// Grabs the compressed packet from the buffer.
    ///
    /// # Parameters
    /// - `buffer` - The buffer to grab the compressed packet from.
    ///
    /// # Returns
    /// The compressed packet in a `[CompressionResult]` format.
    pub fn grab_from_buffer(
        &self,
        buffer: Vec<u8>,
        data: &CompressionData,
    ) -> CompressionResult<PacketBuffer> {
        Ok(match self.compression_type {
            CompressionType::None => NormalCompression::decompress(buffer, data),
            CompressionType::Zlib => ZlibCompression::decompress(buffer, data),
        })
    }

    /// Compresses the given buffer.
    ///
    /// # Parameters
    /// - `buffer` - The buffer to compress.
    ///
    /// # Returns
    /// The compressed buffer in a `[CompressionResult]` format.
    pub fn to_buffer(
        &self,
        buffer: PacketBuffer,
        data: &CompressionData,
    ) -> CompressionResult<Vec<u8>> {
        match self.compression_type {
            CompressionType::None => NormalCompression::compress(buffer, data),
            CompressionType::Zlib => ZlibCompression::compress(buffer, data),
        }
    }
}

/// A trait that defines a compression algorithm type. This is used for values inside `[CompressionType]`.
///
/// # Examples
/// ```rust
/// use buffer::Compression;
///
/// struct CustomCompression;
///
/// impl Compression for CustomCompression {
///    fn compress<B: Buffer>(buffer: B) -> CompressionResult<B> {}
///    fn decompress<B: Buffer>(buffer: B) -> CompressionResult<B> {}
/// }
/// ```
///
/// # Type Parameters
/// - `B` - The buffer type to compress / decompress.
trait Compression {
    /// Compresses the given buffer. This is used for values inside `[CompressionType]`.
    ///
    /// # Parameters
    /// - `buffer` - The buffer to compress.
    fn compress(buffer: PacketBuffer, data: &CompressionData) -> CompressionResult<Vec<u8>>;

    /// Decompresses the given buffer. This is used for values inside `[CompressionType]`.
    ///
    /// # Parameters
    /// - `buffer` - The buffer to decompress.
    fn decompress(buffer: Vec<u8>, data: &CompressionData) -> PacketBuffer;
}

/// This struct represents the `[CompressionType::None]` variant.
///
/// This is used for a compression type that does not compress packets.
///
/// # Examples
/// ```rust
/// use buffer::NormalCompression;
///
/// let result = NormalCompression::compress(PacketBuffer::new(vec![0, 1, 2, 3]));
/// ```
struct NormalCompression;

impl Compression for NormalCompression {
    /// This compression algorithm doesn't actually compress anything. This is used for values inside `[CompressionType]`
    ///
    /// The format of the `None` compression is as follows:
    /// - Field Name | Field Type | Notes
    /// - Length     | VarInt     | Length of Packet ID + Data
    /// - Packet ID  | VarInt     |
    /// - Data       | Byte Array | The data of the packet.
    ///
    /// # Parameters
    /// - `buffer` - The buffer to compress.
    ///
    /// # Returns
    /// The compressed packet in a `[CompressionResult]` format.
    fn compress(buffer: PacketBuffer, _data: &CompressionData) -> CompressionResult<Vec<u8>> {
        Ok(buffer.get_ref().clone())
    }

    /// This decompression algorithm doesn't actually decompress anything. This is used for values inside `[CompressionType]`
    ///
    /// The format of the `None` compression is as follows:
    /// - Field Name | Field Type | Notes
    /// - Length     | VarInt     | Length of Packet ID + Data
    /// - Packet ID  | VarInt     |
    /// - Data       | Byte Array | The data of the packet.
    ///
    /// # Parameters
    /// - `buffer` - The buffer to decompress.
    ///
    /// # Returns
    /// The decompressed packet in a `[CompressionResult]` format.
    ///
    /// # Note
    /// The uncompressed packet does not contain the `data_length` field. Therefore, it's always set to `0`.
    /// This is because the `data_length` field is only used for compressed packets.
    fn decompress(buffer: Vec<u8>, data: &CompressionData) -> PacketBuffer {
        let mut normal_buffer = NormalBuffer::new(buffer);
        PacketBuffer {
            packet_length: normal_buffer.read_varint(),
            data_length: VarInt::from(0),
            packet_id: normal_buffer.read_varint(),
            buffer: normal_buffer,
        }
    }
}

/// This struct represents the `[CompressionType::Zlib]` variant.
///
/// This is used for a compression type that compresses packets using the Zlib algorithm.
///
/// # Examples
/// ```rust
/// use buffer::ZlibCompression;
///
/// let result = ZlibCompression::compress(PacketBuffer::new(vec![0, 1, 2, 3]));
/// ```
struct ZlibCompression;

impl Compression for ZlibCompression {
    /// Compresses the given buffer using the Zlib algorithm. This is used for values inside `[CompressionType]`
    ///
    /// # Parameters
    /// - `buffer` - The buffer to compress.
    ///
    /// # Returns
    /// The compressed packet in a `[CompressionResult]` format.
    fn compress(buffer: PacketBuffer, data: &CompressionData) -> CompressionResult<Vec<u8>> {
        let mut result = Vec::new();
        let buffer_data = buffer.get_ref().clone();
        let packet_id = buffer.packet_id;

        result.extend_from_slice(&encode_varint(0));

        if buffer_data.len() as i32 >= data.threshold {
            result.extend_from_slice(&encode_varint((packet_id.len() + buffer_data.len()) as i32));

            let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
            encoder.write_all(&packet_id.to_network()).unwrap();
            encoder.write_all(&buffer_data).unwrap();
            let compressed_data = encoder.finish().unwrap();

            result.extend_from_slice(&compressed_data);
        } else {
            result.extend_from_slice(&encode_varint(0));

            result.extend_from_slice(&packet_id.to_network());
            result.extend_from_slice(&buffer_data);
        }

        let packet_length = (result.len() - encode_varint(0).len()) as i32;
        let packet_length_encoded = encode_varint(packet_length);
        result[..packet_length_encoded.len()].copy_from_slice(&packet_length_encoded);

        Ok(result)
    }

    /// Decompresses the given buffer using the Zlib algorithm. This is used for values inside `[CompressionType]`
    ///
    /// # Parameters
    /// - `buffer` - The buffer to decompress.
    ///
    /// # Returns
    /// The decompressed packet in a `[CompressionResult]` format.
    fn decompress(buffer: Vec<u8>, data: &CompressionData) -> PacketBuffer {
        unimplemented!()
    }
}
