#[cfg(feature = "codec")]
mod codec;
#[cfg(feature = "codec")]
pub use codec::{Error as CodecError, MinecraftCodec};

#[allow(warnings)]
mod generated;
mod io;

pub use generated::Packet;
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

/// Current state of the connection.
/// This state is updated during the login
/// sequence. See wiki.vg.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Stage {
    Handshake,
    Status,
    Login,
    Play,
}

/// Direction in which a packet is sent.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Clientbound,
    Serverbound,
}
