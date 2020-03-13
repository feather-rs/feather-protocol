//! This module handles the _integration_ of protocols. In effect,
//! it analyzes the various parsed protocols versions, then determines
//! an inheritance graph where lower versions inherit packets from the
//! version above them. In this manner, we can generate only the packet
//! structs which have changed for a given version instead of generating
//! all packets for every single version.
//!
//! Note that when we say a packet "changes," this only applies to its fields.
//! If only its ID is changed, then the ID update can be incorporated in the original
//! packet struct using a `match version { ... }`.

use crate::packets::parsing::{FieldType, Packet, PacketId, PacketIdentifier, Protocol, TypeName};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub type ProtocolVersion = semver::Version;

#[derive(Debug, Clone, Default)]
pub struct IntegrationData {
    /// Vector of integrated protocols, where a protocol at
    /// `protocols[n]` inherits from `protocols[n - 1]`.
    /// If `n - 1 < 0`, then the protocol inherits from nothing,
    /// and it defines all the packets from the parsed data.
    ///
    /// Each protocol only stores the packets which have changed
    /// for that version.
    ///
    /// In addition, the packets of each protocol have a mapping
    /// from protocol version to packet ID. As mentioned in the module-level
    /// docs, a packet does not have to be regenerated for a given version if its ID changes;
    /// instead, the packet struct in the inherited version uses a match on the protocol version
    /// to determine the ID.
    pub protocols: Vec<IntegratedProtocol>,
}

#[derive(Debug, Clone, Default)]
pub struct IntegratedProtocol {
    /// Stores the packets which have changed since the previous protocol.
    ///
    /// A packet is not "changed" if its ID is different; only the fields are checked.
    pub packets: HashMap<PacketIdentifier, IntegratedPacket>,
    /// Stores the types which have changed since the previous protocol.
    pub types: HashMap<TypeName, FieldType>,
}

#[derive(Debug, Clone)]
pub struct IntegratedPacket {
    inner: Packet,
    /// Mapping from protocol version => ID of this packet for that version.
    pub ids: HashMap<ProtocolVersion, PacketId>,
}

impl Deref for IntegratedPacket {
    type Target = Packet;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for IntegratedPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Integrates a set of protocols.
///
/// For all `n` in `[0, protocols.len())`, the version at `versions[n]`
/// must correspond to the protocol at `protocols[n]`.
///
/// # Panics
/// Panics if `versions.len() != protocols.len()`.
pub fn integrate(
    protocols: Vec<Protocol>,
    versions: Vec<ProtocolVersion>,
) -> anyhow::Result<IntegrationData> {
    assert_eq!(versions.len(), protocols.len());
    assert!(!protocols.is_empty());

    let mut data = IntegrationData::default();

    // combine protocols and versions so we can sort them by version
    let mut combined: Vec<_> = protocols.into_iter().zip(versions).collect();

    combined.sort_unstable_by(|(_, v1), (_, v2)| v2.cmp(v1)); // intentional reverse comparison: later versions first

    // fill `protocols` with empty integrated protocols,
    // one for each protocol which will be integrated
    data.protocols
        .extend(combined.iter().map(|_| IntegratedProtocol::default()));

    // the first (most recently released) protocol inherits from nothing,
    // so we just copy the data
    copy_protocol(&combined[0].0, &combined[0].1, &mut data.protocols[0]);

    // for each protocol which inherits from another,
    // check for changes
    todo!();

    Ok(data)
}

fn copy_protocol(
    protocol: &Protocol,
    version: &ProtocolVersion,
    integrated: &mut IntegratedProtocol,
) {
    integrated.packets = protocol
        .packets
        .clone()
        .into_iter()
        .map(|(k, packet)| {
            (
                k,
                IntegratedPacket {
                    inner: packet,
                    ids: HashMap::new(),
                },
            )
        })
        .collect();

    integrated.packets.values_mut().for_each(|packet| {
        packet.ids.insert(version.clone(), packet.id);
    });

    integrated.types = protocol.types.clone();
}
