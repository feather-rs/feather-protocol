#[allow(warnings)]
mod generated;
mod io;

pub use io::{Readable, Writeable};

pub(crate) use io::VarInt;

/// Denotes a type which may be treated as a packet.
///
/// If you want to store arbitrary packets (e.g. for sending
/// over a channel), use [`Packet`](crate::Packet) instead,
/// as it does not require boxing.
pub trait PacketTrait: Readable + Writeable {
    /// Returns the ID of this packet.
    fn id() -> u32
    where
        Self: Sized;
}
