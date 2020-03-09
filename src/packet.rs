use crate::ProtocolVersion;
use bytes::{Bytes, BytesMut};
use smallbox::space::S32;
use smallbox::SmallBox;

pub type DynPacket = SmallBox<dyn Packet, S32>;

/// Represents a packet.
pub trait Packet: Send + Sync + 'static {
    fn id(&self, version: ProtocolVersion) -> u32;
    fn name(&self) -> &'static str;
    fn write_to(self, buf: &mut BytesMut, version: ProtocolVersion);
}

pub trait PacketReader: Send + Sync + 'static + Default {
    fn read_from(buf: &mut Bytes, version: ProtocolVersion) -> anyhow::Result<DynPacket>;
}
