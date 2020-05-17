use crate::{Readable, VarInt, Writeable};
use anyhow::Context;
use bytes::{Bytes, BytesMut};
pub mod serverbound {
    use super::*;
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
                let client_username = String::read(buffer)
                    .context("Failed to read field `client_username` of struct `login_start`")?;
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
                let shared_secret = Vec::<u8>::read(buffer).context(
                    "Failed to read field `shared_secret` of struct `encryption_response`",
                )?;
                let verify_token = Vec::<u8>::read(buffer).context(
                    "Failed to read field `verify_token` of struct `encryption_response`",
                )?;
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
                let message_id = VarInt::read(buffer).context(
                    "Failed to read field `message_id` of struct `login_plugin_response`",
                )?;
                let data = Option::<Vec<u8>>::read(buffer)
                    .context("Failed to read field `data` of struct `login_plugin_response`")?;
                Ok(Self { message_id, data })
            }
        }
    }
    pub mod status {
        use super::*;
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
                let reason = String::read(buffer)
                    .context("Failed to read field `reason` of struct `disconnect`")?;
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
                let server_id = String::read(buffer)
                    .context("Failed to read field `server_id` of struct `encryption_request`")?;
                let public_key = Vec::<u8>::read(buffer)
                    .context("Failed to read field `public_key` of struct `encryption_request`")?;
                let verify_token = Vec::<u8>::read(buffer).context(
                    "Failed to read field `verify_token` of struct `encryption_request`",
                )?;
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
                let uuid = String::read(buffer)
                    .context("Failed to read field `uuid` of struct `login_success`")?;
                let username = String::read(buffer)
                    .context("Failed to read field `username` of struct `login_success`")?;
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
                let threshold = VarInt::read(buffer)
                    .context("Failed to read field `threshold` of struct `set_compression`")?;
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
                let message_id = VarInt::read(buffer).context(
                    "Failed to read field `message_id` of struct `login_plugin_request`",
                )?;
                let channel = String::read(buffer)
                    .context("Failed to read field `channel` of struct `login_plugin_request`")?;
                let data = Vec::<u8>::read(buffer)
                    .context("Failed to read field `data` of struct `login_plugin_request`")?;
                Ok(Self {
                    message_id,
                    channel,
                    data,
                })
            }
        }
    }
    pub mod status {
        use super::*;
    }
    pub mod handshake {
        use super::*;
    }
}
