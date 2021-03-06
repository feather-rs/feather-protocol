mod error;
mod message;
mod packet;
mod packets;
mod provider;
mod types;

pub use error::Error;
pub use message::Message;
pub use packet::{DynPacket, Packet, PacketReader};
pub use provider::{Provider, RawChunkPalette, RawChunkSection};
pub use types::{BlockPosition, BytesExt, BytesMutExt, Node, Slot};

/// Protocol version.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolVersion {
    V1_13_2,
    V1_14_4,
    V1_15_0,
    V1_15_1,
    V1_15_2,
}
