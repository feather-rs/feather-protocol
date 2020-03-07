mod error;
mod message;
mod packet;
mod provider;
mod types;

pub use error::Error;
pub use message::Message;
pub use packet::{DynPacket, Packet, PacketReader};
pub use provider::{Provider, RawChunkPalette, RawChunkSection};

/// Protocol version.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolVersion {
    V1_13_2,
    V1_14_4,
    V1_15_2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnumRepr {
    Integer(i64),
    String(String),
}
