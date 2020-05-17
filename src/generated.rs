use crate::{Readable, VarInt, Writeable};
use anyhow::{bail, Context};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::TryFrom;
pub mod clientbound {
    use super::*;
    pub mod handshake {
        use super::*;
    }
    pub mod login {
        use super::*;
        #[derive(Debug, Clone)]
        pub struct Disconnect {
            pub reason: String,
        }
        impl Readable for Disconnect {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let reason = String::read(buffer).context("Failed to read field `reason`")?;
                Ok(Self { reason })
            }
        }
        #[derive(Debug, Clone)]
        pub struct EncryptionRequest {
            pub server_id: String,
            pub public_key: Vec<u8>,
            pub verify_token: Vec<u8>,
        }
        impl Readable for EncryptionRequest {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let server_id = String::read(buffer).context("Failed to read field `server_id`")?;
                let public_key = {
                    let length = usize::try_from(
                        VarInt::read(buffer).context("Failed to read field `public_key`")?,
                    )?;
                    if length > std::u16::MAX as usize {
                        bail!(
                            "array length `{}` exceeds maximum allowed array length",
                            length
                        );
                    }
                    let mut buf = Vec::with_capacity(length);
                    for _ in 0..length {
                        buf.push(u8::read(buffer).context("Failed to read field `public_key`")?);
                    }
                    buf
                };
                let verify_token = {
                    let length = usize::try_from(
                        VarInt::read(buffer).context("Failed to read field `verify_token`")?,
                    )?;
                    if length > std::u16::MAX as usize {
                        bail!(
                            "array length `{}` exceeds maximum allowed array length",
                            length
                        );
                    }
                    let mut buf = Vec::with_capacity(length);
                    for _ in 0..length {
                        buf.push(u8::read(buffer).context("Failed to read field `verify_token`")?);
                    }
                    buf
                };
                Ok(Self {
                    server_id,
                    public_key,
                    verify_token,
                })
            }
        }
        #[derive(Debug, Clone)]
        pub struct LoginSuccess {
            pub uuid: String,
            pub username: String,
        }
        impl Readable for LoginSuccess {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let uuid = String::read(buffer).context("Failed to read field `uuid`")?;
                let username = String::read(buffer).context("Failed to read field `username`")?;
                Ok(Self { uuid, username })
            }
        }
        #[derive(Debug, Clone)]
        pub struct SetCompression {
            pub threshold: VarInt,
        }
        impl Readable for SetCompression {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let threshold = VarInt::read(buffer).context("Failed to read field `threshold`")?;
                Ok(Self { threshold })
            }
        }
        #[derive(Debug, Clone)]
        pub struct LoginPluginRequest {
            pub message_id: VarInt,
            pub channel: String,
            pub data: Vec<u8>,
        }
        impl Readable for LoginPluginRequest {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let message_id =
                    VarInt::read(buffer).context("Failed to read field `message_id`")?;
                let channel = String::read(buffer).context("Failed to read field `channel`")?;
                let data = {
                    let length = buffer.remaining();
                    if length > std::u16::MAX as usize {
                        bail!(
                            "array length `{}` exceeds maximum allowed array length",
                            length
                        );
                    }
                    let mut buf = Vec::with_capacity(length);
                    for _ in 0..length {
                        buf.push(u8::read(buffer).context("Failed to read field `data`")?);
                    }
                    buf
                };
                Ok(Self {
                    message_id,
                    channel,
                    data,
                })
            }
        }
    }
    pub mod play {
        use super::*;
    }
    pub mod status {
        use super::*;
    }
}
pub mod serverbound {
    use super::*;
    pub mod handshake {
        use super::*;
    }
    pub mod login {
        use super::*;
        #[derive(Debug, Clone)]
        pub struct LoginStart {
            pub client_username: String,
        }
        impl Readable for LoginStart {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let client_username =
                    String::read(buffer).context("Failed to read field `client_username`")?;
                Ok(Self { client_username })
            }
        }
        #[derive(Debug, Clone)]
        pub struct EncryptionResponse {
            pub shared_secret: Vec<u8>,
            pub verify_token: Vec<u8>,
        }
        impl Readable for EncryptionResponse {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let shared_secret = {
                    let length = usize::try_from(
                        VarInt::read(buffer).context("Failed to read field `shared_secret`")?,
                    )?;
                    if length > std::u16::MAX as usize {
                        bail!(
                            "array length `{}` exceeds maximum allowed array length",
                            length
                        );
                    }
                    let mut buf = Vec::with_capacity(length);
                    for _ in 0..length {
                        buf.push(u8::read(buffer).context("Failed to read field `shared_secret`")?);
                    }
                    buf
                };
                let verify_token = {
                    let length = usize::try_from(
                        VarInt::read(buffer).context("Failed to read field `verify_token`")?,
                    )?;
                    if length > std::u16::MAX as usize {
                        bail!(
                            "array length `{}` exceeds maximum allowed array length",
                            length
                        );
                    }
                    let mut buf = Vec::with_capacity(length);
                    for _ in 0..length {
                        buf.push(u8::read(buffer).context("Failed to read field `verify_token`")?);
                    }
                    buf
                };
                Ok(Self {
                    shared_secret,
                    verify_token,
                })
            }
        }
        #[derive(Debug, Clone)]
        pub struct LoginPluginResponse {
            pub message_id: VarInt,
            pub data: Option<Vec<u8>>,
        }
        impl Readable for LoginPluginResponse {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let message_id =
                    VarInt::read(buffer).context("Failed to read field `message_id`")?;
                let data = {
                    let present = bool::read(buffer).context("Failed to read field `data`")?;
                    if present {
                        Some({
                            let length = buffer.remaining();
                            if length > std::u16::MAX as usize {
                                bail!(
                                    "array length `{}` exceeds maximum allowed array length",
                                    length
                                );
                            }
                            let mut buf = Vec::with_capacity(length);
                            for _ in 0..length {
                                buf.push(u8::read(buffer).context("Failed to read field `data`")?);
                            }
                            buf
                        })
                    } else {
                        None
                    }
                };
                Ok(Self { message_id, data })
            }
        }
    }
    pub mod play {
        use super::*;
    }
    pub mod status {
        use super::*;
    }
}
