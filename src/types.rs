//! Extension traits for `Bytes` and `BytesMut` which support Minecraft types.
use bytes::Buf;
use bytes::{Bytes, BytesMut};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("insufficient bytes left in buffer")]
    NotEnoughBytes,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait BytesExt {
    fn try_get_var_int(&mut self) -> Result<i32>;
    fn try_get_string(&mut self) -> Result<String>;
    fn try_get_bool(&mut self) -> Result<bool>;

    fn try_get_i8(&mut self) -> Result<i8>;
    fn try_get_i16(&mut self) -> Result<i16>;
    fn try_get_i32(&mut self) -> Result<i32>;
    fn try_get_i64(&mut self) -> Result<i64>;

    fn try_get_f32(&mut self) -> Result<f32>;
    fn try_get_f64(&mut self) -> Result<f64>;

    fn try_get_u8(&mut self) -> Result<u8>;
    fn try_get_u16(&mut self) -> Result<u16>;
    fn try_get_u32(&mut self) -> Result<u32>;
    fn try_get_u64(&mut self) -> Result<u64>;
}

pub trait BytesMutExt {
    fn put_var_int(&mut self, x: i32);
    fn put_string(&mut self, x: impl AsRef<str>);
    fn put_bool(&mut self, x: bool);
}

macro_rules! try_get_impl {
    ($this:ident, $size:expr, $method:ident) => {{
        if $this.remaining() < $size {
            return Err(Error::NotEnoughBytes);
        }

        return Ok($this.$method());
    }};
}

impl BytesExt for Bytes {
    fn try_get_var_int(&mut self) -> Result<i32> {
        unimplemented!()
    }

    fn try_get_string(&mut self) -> Result<String> {
        unimplemented!()
    }

    fn try_get_bool(&mut self) -> Result<bool> {
        unimplemented!()
    }

    fn try_get_i8(&mut self) -> Result<i8> {
        try_get_impl!(self, 1, get_i8);
    }

    fn try_get_i16(&mut self) -> Result<i16> {
        try_get_impl!(self, 2, get_i16);
    }

    fn try_get_i32(&mut self) -> Result<i32> {
        try_get_impl!(self, 4, get_i32);
    }

    fn try_get_i64(&mut self) -> Result<i64> {
        try_get_impl!(self, 8, get_i64);
    }

    fn try_get_f32(&mut self) -> Result<f32> {
        try_get_impl!(self, 4, get_f32);
    }

    fn try_get_f64(&mut self) -> Result<f64> {
        try_get_impl!(self, 8, get_f64);
    }

    fn try_get_u8(&mut self) -> Result<u8> {
        try_get_impl!(self, 1, get_u8);
    }

    fn try_get_u16(&mut self) -> Result<u16> {
        try_get_impl!(self, 2, get_u16);
    }

    fn try_get_u32(&mut self) -> Result<u32> {
        try_get_impl!(self, 4, get_u32);
    }

    fn try_get_u64(&mut self) -> Result<u64> {
        try_get_impl!(self, 8, get_u64);
    }
}

impl BytesMutExt for BytesMut {
    fn put_var_int(&mut self, _x: i32) {
        unimplemented!()
    }

    fn put_string(&mut self, _x: impl AsRef<str>) {
        unimplemented!()
    }

    fn put_bool(&mut self, _x: bool) {
        unimplemented!()
    }
}
