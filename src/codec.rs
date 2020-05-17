use crate::{Direction, Packet, Stage};
use aes::Aes128;
use bytes::buf::BufMutExt;
use bytes::{Buf, BytesMut};
use cfb8::stream_cipher::{NewStreamCipher, StreamCipher};
use cfb8::Cfb8;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Cursor, Read, Write};
use thiserror::Error;
use tokio_util::codec::{Decoder, Encoder};

type AesCfb8 = Cfb8<Aes128>;

/// Maximum possible size of a varint.
const MAX_VAR_INT_SIZE: usize = 5;
/// Maximum allowed length of a received packet.
const MAX_PACKET_LEN: usize = 1_048_576; // One MiB
/// Maximum possible size of a packet header.
const HEADER_SIZE: usize = MAX_VAR_INT_SIZE * 2;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Packet of length {0} (under compression threshold {1}) was sent compressed")]
    CompressedPacketTooSmall(usize, usize),
    #[error("Packet length {0} is too large")]
    PacketTooLarge(usize),
    #[error("Invalid packet ID {0} for stage {1:?}")]
    InvalidPacketId(u32, Stage),
}

/// Codec for encoding and decoding Minecraft packets.
pub struct MinecraftCodec {
    /// Direction of outgoing packets.
    outgoing_direction: Direction,
    /// The current stage of this codec.
    stage: Stage,
    /// The encrypter, if encryption is enabled.
    encrypter: Option<AesCfb8>,
    /// The decrypter, if encryption is enabled.
    decrypter: Option<AesCfb8>,
    /// The compression threshold, if compression is enabled.
    compression_threshold: Option<usize>,
    /// Cached buffer for writing header data.
    /// Using this avoids reallocations.
    header_buffer: BytesMut,
    /// Cached buffer into which we write decompressed
    /// data. Using this avoids reallocations.
    decompressed_buffer: Vec<u8>,
    /// Index into `src` of next byte to decrypt.
    decrypt_index: usize,
}

impl MinecraftCodec {
    /// Creates a new `MinecraftCodec`. The given [`Direction`]
    /// must be the direction of outgoing packets. (For example,
    /// if you are writing a server, then it should be `Clientbound`.)
    ///
    /// By default, encryption and compression are disabled.
    /// Call `enable_encryption` and `enable_compression` to enable them.
    /// Additionally, the [`Stage`] starts as `Handshake`. Call `set_stage`
    /// to update it.
    ///
    /// [`Direction`]: ../enum.Direction.html
    /// [`Stage`]: ../enum.Stage.html
    pub fn new(outgoing_direction: Direction) -> Self {
        Self {
            outgoing_direction,
            stage: Stage::Handshake,
            encrypter: None,
            decrypter: None,
            compression_threshold: None,
            header_buffer: BytesMut::with_capacity(HEADER_SIZE),
            decompressed_buffer: vec![],
            decrypt_index: 0,
        }
    }

    pub fn enable_compression(&mut self, threshold: usize) {
        self.compression_threshold = Some(threshold);
    }

    pub fn enable_encryption(&mut self, key: [u8; 16]) {
        // This is the toppoint of security: using the same IV
        // for every packet. Typical for Mojang.
        self.encrypter = Some(AesCfb8::new_var(&key, &key).unwrap());
        self.decrypter = Some(AesCfb8::new_var(&key, &key).unwrap());
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }
}

impl Encoder<Packet> for MinecraftCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, packet: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Reserve space for the packet header (at most 2 * 5 bytes, for length + data length).
        // `header` will contain the first 10 bytes of the buffer, while `dst`
        // still contains the rest.
        // "Data length" refers to the uncompressed size of the packet.
        // Since we cannot know the size of the header in advance, thanks to varints,
        // we reserve the maximum size and copy the header in with a correct offset.
        assert!(dst.is_empty());
        dst.reserve(HEADER_SIZE);

        // Zero out the header.
        dst.extend_from_slice(&[0u8; HEADER_SIZE]);

        let mut header = dst.split_to(HEADER_SIZE);
        assert!(dst.is_empty());

        // Write raw packet data to `dst`.
        let ty = packet.ty();
        dst.push_var_int(ty.get_id().0 as i32);
        packet.write_to(dst);

        // If compression is enabled, we follow a more complex course of action:
        // * Write the raw packet data to `dst`.
        // * If the data is less than the compression threshold, proceed as usual.
        // * Otherwise, we move forward into the buffer, allocating
        // another header and then writing the compressed bytes
        // to the capacity after that.
        let data_len: Option<usize> = if let Some(threshold) = self.compression_threshold {
            let data_len = dst.len();
            if data_len >= threshold {
                // Allocate new header
                dst.reserve(HEADER_SIZE);

                let uncompressed = dst.split_to(data_len);
                dst.extend_from_slice(&[0u8; HEADER_SIZE]);
                header = dst.split_to(HEADER_SIZE);

                assert!(dst.is_empty());
                // Compress data into `compressed`.
                let mut encoder = ZlibEncoder::new(dst.writer(), Compression::default());
                encoder.write_all(uncompressed.as_ref()).unwrap();

                Some(data_len)
            } else {
                Some(0) // Not compressed
            }
        } else {
            None
        };

        // Figure out the length of `data_length` encoded.
        let length_of_data_length = match data_len {
            Some(data_len) => {
                let mut temp = BytesMut::with_capacity(MAX_VAR_INT_SIZE);
                temp.push_var_int(data_len as i32);
                temp.len()
            }
            None => 0,
        };

        // Write header. We first write to a temporary buffer,
        // then copy this to the correct position in `header`,
        // trimming off the unused bytes.
        self.header_buffer
            .push_var_int((length_of_data_length + dst.len()) as i32);
        if let Some(data_len) = data_len {
            self.header_buffer.push_var_int(data_len as i32);
        }

        // Offset into `header` to write to.
        let header_offset = HEADER_SIZE - self.header_buffer.len();
        // Discard unused header bytes.
        header.advance(header_offset);
        header.clear();

        // Write into header.
        header.extend_from_slice(&self.header_buffer);
        self.header_buffer.clear();

        // Finally, merge `header` and `dst`.
        std::mem::swap(dst, &mut header);
        dst.unsplit(header);

        // If encryption is enabled, encrypt data in place.
        if let Some(crypter) = self.encrypter.as_mut() {
            crypter.encrypt(dst);
        }

        Ok(())
    }
}

impl Decoder for MinecraftCodec {
    type Item = Packet;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // If encryption is enabled, decrypt undecrypted data.
        if let Some(crypter) = self.decrypter.as_mut() {
            crypter.decrypt(&mut src[self.decrypt_index..]);
            self.decrypt_index = src.len();
        }

        // Conversion to `Cursor` is required because `Bytes` does
        // not implement `Buf`.
        let mut cursor = Cursor::new(src.as_ref());

        // Read header.
        let length = match cursor.try_get_var_int() {
            Ok(length) => length as usize,
            Err(TryGetError::NotEnoughBytes) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        if length > cursor.remaining() {
            // Full packet has not been read yet.
            return Ok(None);
        }

        // Prevent malicious clients from causing huge allocations.
        if length > MAX_PACKET_LEN {
            return Err(Error::PacketTooLarge(length).into());
        }

        // At this point, we know a full packet has been received.

        // Trim `cursor` and `src` to length of packet.
        let position = cursor.position() as usize;
        src.advance(position);
        cursor = Cursor::new(&src[..length]);

        // If compression is enabled:
        // * Read the data length field. If 0, continue as normal: the packet is not compressed.
        // * Decompress remaining bytes into `self.decompressed_buffer`.
        // * Update `cursor` to read from `self.decompressed_buffer`.
        if let Some(threshold) = self.compression_threshold {
            let data_length = cursor.try_get_var_int()?;

            if data_length != 0 {
                self.decompressed_buffer.clear();

                let mut decoder = ZlibDecoder::new(cursor);
                decoder.read_to_end(&mut self.decompressed_buffer)?;

                let actual_data_length = self.decompressed_buffer.len();
                if actual_data_length < threshold {
                    return Err(
                        Error::CompressedPacketTooSmall(actual_data_length, threshold).into(),
                    );
                }

                cursor = Cursor::new(&self.decompressed_buffer);
            }
        }

        // Read packet.
        let id = cursor.try_get_var_int()? as u32;
        // If we don't know this packet type, skip the packet.
        let packet_type = {
            match PacketType::get_from_id(PacketId(id, self.incoming_direction, self.stage)) {
                Ok(ty) => ty,
                Err(_) => {
                    // Advance buffer and stop.
                    src.advance(length);
                    self.decrypt_index = src.len();
                    return Ok(None);
                }
            }
        };

        let mut packet = packet_type.get_implementation();
        packet.read_from(&mut cursor)?;

        src.advance(length);
        self.decrypt_index = src.len();

        Ok(Some(packet))
    }
}
