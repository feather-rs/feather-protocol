//! Traits for reading/writing Minecraft-encoded values.

use anyhow::{bail, Context};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::{
    convert::{TryFrom, TryInto},
    num::TryFromIntError,
};
use thiserror::Error;

/// Trait implemented for types which can be written
/// to a buffer.
pub trait Writeable {
    /// Writes this value to the given buffer.
    fn write(&self, buffer: &mut BytesMut);
}

/// Trait implemented for types which can be read
/// from a buffer.
pub trait Readable {
    /// Reads this type from the given buffer.
    fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
    where
        Self: Sized;
}

/// Error when reading a value.
#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected end of input: failed to read value of type `{0}`")]
    UnexpectedEof(&'static str),
}

macro_rules! integer_impl {
    ($($int:ty, $read_fn:ident, $write_fn:ident),* $(,)?) => {
        $(
            impl Readable for $int {
                fn read(buffer: &mut Bytes) -> anyhow::Result<Self> {
                    if buffer.remaining() < std::mem::size_of::<Self>() {
                        bail!(Error::UnexpectedEof(std::any::type_name::<Self>()));
                    }

                    Ok(buffer.$read_fn())
                }
            }

            impl Writeable for $int {
                fn write(&self, buffer: &mut BytesMut) {
                    buffer.$write_fn(*self);
                }
            }
        )*
    }
}

integer_impl! {
    u8, get_u8, put_u8,
    u16, get_u16, put_u16,
    u32, get_u32, put_u32,
    u64, get_u64, put_u64,

    i8, get_i8, put_i8,
    i16, get_i16, put_i16,
    i32, get_i32, put_i32,
    i64, get_i64, put_i64,

    f32, get_f32, put_f32,
    f64, get_f64, put_f64,
}

/// A variable-length integer as defined by the Minecraft protocol.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl Readable for VarInt {
    fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut num_read = 0;
        let mut result = 0;

        loop {
            if !buffer.has_remaining() {
                bail!(Error::UnexpectedEof("VarInt"));
            }

            let read = u8::read(buffer)?;
            let value = i32::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7 * num_read).0;

            num_read += 1;

            if num_read > 5 {
                bail!(
                    "VarInt too long (max length: 5, value read so far: {})",
                    result
                );
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(VarInt(result))
    }
}

impl TryFrom<VarInt> for usize {
    type Error = TryFromIntError;
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl Writeable for VarInt {
    fn write(&self, buffer: &mut BytesMut) {
        let mut x = self.0;
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }

            buffer.put_u8(temp);

            if x == 0 {
                break;
            }
        }
    }
}

impl Readable for String {
    fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // Length is encoded as VarInt.
        // Following `length` bytes are the UTF8-encoded
        // string.

        let length = VarInt::read(buffer)
            .context("failed to read string length")?
            .0 as usize;

        // TODO: support custom length limits
        // Current max length is max value of a signed 16-bit int.
        let max_length = std::i16::MAX as usize;
        if length > max_length {
            bail!(
                "string length {} exceeds maximum allowed length of {}",
                length,
                max_length
            );
        }

        if buffer.remaining() < length {
            bail!(Error::UnexpectedEof("String"));
        }

        // Read string into buffer.
        let bytes = buffer.split_to(length);
        let s = std::str::from_utf8(&bytes).context("string contained invalid UTF8")?;

        Ok(s.to_owned())
    }
}

impl Writeable for bool {
    fn write(&self, buffer: &mut BytesMut) {
        let x = if *self { 1u8 } else { 0 };
        x.write(buffer);
    }
}

impl Readable for bool {
    fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let x = u8::read(buffer)?;

        if x == 0 {
            Ok(false)
        } else if x == 1 {
            Ok(true)
        } else {
            Err(anyhow::anyhow!("invalid boolean tag {}", x))
        }
    }
}
