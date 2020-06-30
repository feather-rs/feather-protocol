//! Traits for reading/writing Minecraft-encoded values.

use anyhow::{bail, Context};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    convert::{TryFrom, TryInto},
    io::{Cursor, Read},
    num::TryFromIntError,
};
use thiserror::Error;

/// Trait implemented for types which can be read
/// from a buffer.
pub trait Readable {
    /// Reads this type from the given buffer.
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

/// Trait implemented for types which can be written
/// to a buffer.
pub trait Writeable {
    /// Writes this value to the given buffer.
    fn write(&self, buffer: &mut Vec<u8>);
}

/// Error when reading a value.
#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected end of input: failed to read value of type `{0}`")]
    UnexpectedEof(&'static str),
}

macro_rules! integer_impl {
    ($($int:ty, $read_fn:tt, $write_fn:tt),* $(,)?) => {
        $(
            impl Readable for $int {
                fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
                    buffer.$read_fn().map_err(anyhow::Error::from)
                }
            }

            impl Writeable for $int {
                fn write(&self, buffer: &mut Vec<u8>) {
                    buffer.$write_fn(*self);
                }
            }
        )*
    }
}

integer_impl! {
    u8, read_u8, write_u8,
}

/// A variable-length integer as defined by the Minecraft protocol.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl Readable for VarInt {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut num_read = 0;
        let mut result = 0;

        loop {
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

impl From<usize> for VarInt {
    fn from(x: usize) -> Self {
        VarInt(x as i32)
    }
}

impl Writeable for VarInt {
    fn write(&self, buffer: &mut Vec<u8>) {
        let mut x = self.0;
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }

            buffer.write_u8(temp);

            if x == 0 {
                break;
            }
        }
    }
}

impl Readable for String {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
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

        // Read string into buffer.
        let mut temp = Vec::with_capacity(length);
        buffer
            .read_exact(&mut temp)
            .map_err(|_| Error::UnexpectedEof("String"))?;
        let s = std::str::from_utf8(&temp).context("string contained invalid UTF8")?;

        Ok(s.to_owned())
    }
}

impl Writeable for String {
    fn write(&self, buffer: &mut Vec<u8>) {
        VarInt(self.len() as i32).write(buffer);
        buffer.extend_from_slice(self.as_bytes());
    }
}

impl Readable for bool {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
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

impl Writeable for bool {
    fn write(&self, buffer: &mut Vec<u8>) {
        let x = if *self { 1u8 } else { 0 };
        x.write(buffer);
    }
}
