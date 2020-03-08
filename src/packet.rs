use bytes::{Bytes, BytesMut};
use smallbox::space::S32;
use smallbox::SmallBox;

pub type DynPacket = SmallBox<dyn Packet, S32>;

/// Represents a packet.
pub trait Packet: Send + Sync + 'static {
    fn id(&self) -> u32;
    fn name(&self) -> &'static str;
    fn write_to(self, buf: &mut BytesMut);
}

pub trait PacketReader: Send + Sync + 'static + Default {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket>;
}
