pub use r#enum::Packet;
mod r#enum {
    use super::{
        clientbound::{handshake::*, login::*, play::*, status::*},
        serverbound::{handshake::*, login::*, play::*, status::*},
        *,
    };
    #[derive(Debug, Clone)]
    pub enum Packet {
        Disconnect(Disconnect),
        EncryptionRequest(EncryptionRequest),
        LoginSuccess(LoginSuccess),
        SetCompression(SetCompression),
        LoginPluginRequest(LoginPluginRequest),
        LoginStart(LoginStart),
        EncryptionResponse(EncryptionResponse),
        LoginPluginResponse(LoginPluginResponse),
    }
    impl From<Disconnect> for Packet {
        fn from(packet: Disconnect) -> Self {
            Packet::Disconnect(packet)
        }
    }
    impl From<EncryptionRequest> for Packet {
        fn from(packet: EncryptionRequest) -> Self {
            Packet::EncryptionRequest(packet)
        }
    }
    impl From<LoginSuccess> for Packet {
        fn from(packet: LoginSuccess) -> Self {
            Packet::LoginSuccess(packet)
        }
    }
    impl From<SetCompression> for Packet {
        fn from(packet: SetCompression) -> Self {
            Packet::SetCompression(packet)
        }
    }
    impl From<LoginPluginRequest> for Packet {
        fn from(packet: LoginPluginRequest) -> Self {
            Packet::LoginPluginRequest(packet)
        }
    }
    impl From<LoginStart> for Packet {
        fn from(packet: LoginStart) -> Self {
            Packet::LoginStart(packet)
        }
    }
    impl From<EncryptionResponse> for Packet {
        fn from(packet: EncryptionResponse) -> Self {
            Packet::EncryptionResponse(packet)
        }
    }
    impl From<LoginPluginResponse> for Packet {
        fn from(packet: LoginPluginResponse) -> Self {
            Packet::LoginPluginResponse(packet)
        }
    }
}
use crate::{Readable, VarInt, Writeable};
use anyhow::{bail, Context};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::TryFrom;
pub mod serverbound {
    use super::*;
    pub mod status {
        use super::*;
    }
    pub mod play {
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
        impl Writeable for LoginStart {
            fn write(&self, buffer: &mut BytesMut) {
                (&self.client_username).write(buffer);
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
        impl Writeable for EncryptionResponse {
            fn write(&self, buffer: &mut BytesMut) {
                let length = VarInt::from((&self.shared_secret).len());
                length.write(buffer);
                for elem in (&self.shared_secret) {
                    elem.write(buffer);
                }
                let length = VarInt::from((&self.verify_token).len());
                length.write(buffer);
                for elem in (&self.verify_token) {
                    elem.write(buffer);
                }
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
        impl Writeable for LoginPluginResponse {
            fn write(&self, buffer: &mut BytesMut) {
                (&self.message_id).write(buffer);
                let tag = (&self.data).is_some();
                tag.write(buffer);
                if let Some(val) = (&self.data) {
                    for elem in val {
                        elem.write(buffer);
                    }
                }
            }
        }
    }
    pub mod handshake {
        use super::*;
    }
}
pub mod clientbound {
    use super::*;
    pub mod play {
        use super::*;
    }
    pub mod status {
        use super::*;
    }
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
        impl Writeable for Disconnect {
            fn write(&self, buffer: &mut BytesMut) {
                (&self.reason).write(buffer);
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
        impl Writeable for EncryptionRequest {
            fn write(&self, buffer: &mut BytesMut) {
                (&self.server_id).write(buffer);
                let length = VarInt::from((&self.public_key).len());
                length.write(buffer);
                for elem in (&self.public_key) {
                    elem.write(buffer);
                }
                let length = VarInt::from((&self.verify_token).len());
                length.write(buffer);
                for elem in (&self.verify_token) {
                    elem.write(buffer);
                }
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
        impl Writeable for LoginSuccess {
            fn write(&self, buffer: &mut BytesMut) {
                (&self.uuid).write(buffer);
                (&self.username).write(buffer);
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
        impl Writeable for SetCompression {
            fn write(&self, buffer: &mut BytesMut) {
                (&self.threshold).write(buffer);
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
        impl Writeable for LoginPluginRequest {
            fn write(&self, buffer: &mut BytesMut) {
                (&self.message_id).write(buffer);
                (&self.channel).write(buffer);
                for elem in (&self.data) {
                    elem.write(buffer);
                }
            }
        }
    }
}
