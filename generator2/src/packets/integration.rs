//! This module handles the _integration_ of protocols. It
//! is used to generate descriptors for _wrapper packets_ which
//! act as wrappers of the same packet for all supported versions.

use crate::packets::parsing::{
    FieldName, FieldType, Packet, PacketId, PacketIdentifier, PacketName, Protocol, TypeName,
};
use std::collections::{HashMap, HashSet};
use std::iter;

pub type ProtocolVersion = semver::Version;

#[derive(Debug, Clone, Default)]
pub struct IntegrationData {
    pub packets: HashMap<PacketIdentifier, IntegratedPacket>,
}

/// An "integrated" packet. This type describes the fields
/// of a packet for all versions.
#[derive(Debug, Clone, Default)]
pub struct IntegratedPacket {
    /// The different versions of this packet.
    pub versions: HashMap<ProtocolVersion, Packet>,
    /// The set of fields which exist in all versions of this packet.
    pub fields_union: HashSet<FieldName>,
    /// The set of fields which exist in a subset of versions of this packet.
    ///
    /// This is a mapping from field name => set of versions for which
    /// the field exists.
    pub fields_version_subset: HashMap<FieldName, HashSet<ProtocolVersion>>,
    /// All fields that can exist for this packet.
    pub fields: HashMap<FieldName, FieldType>,
    /// Packet IDs for this packet for each version.
    pub ids: HashMap<ProtocolVersion, PacketId>,
}

/// An integrated type. Fields are the same as for `IntegratedPacket`.
#[derive(Debug, Clone, Default)]
pub struct IntegratedType {
    pub versions: HashMap<ProtocolVersion, FieldType>,
    pub fields_union: HashSet<FieldName>,
    pub fields_version_subset: HashMap<FieldName, HashSet<ProtocolVersion>>,
}

/// Integrates a set of protocol versions.
pub fn integrate(protocols: &[(Protocol, ProtocolVersion)]) -> anyhow::Result<IntegrationData> {
    let mut data = IntegrationData::default();

    protocols
        .iter()
        .flat_map(|(protocol, version)| {
            let version = version.clone();
            protocol
                .clone()
                .packets
                .into_iter()
                .zip(iter::repeat_with(move || version.clone()))
        })
        .for_each(|((_, packet), version)| {
            apply_versioned_packet_to_integrated(packet, version, &mut data);
        });

    data.packets
        .values_mut()
        .for_each(complete_packet_integration);

    Ok(data)
}

fn apply_versioned_packet_to_integrated(
    packet: Packet,
    version: ProtocolVersion,
    data: &mut IntegrationData,
) {
    let integrated = data.packets.entry(packet.identifier.clone()).or_default();

    integrated.ids.insert(version.clone(), packet.id);
    integrated.versions.insert(version.clone(), packet.clone());
}

fn complete_packet_integration(packet: &mut IntegratedPacket) {
    let mut known_fields = HashMap::new();

    for (_, packet) in &packet.versions {
        known_fields.extend(packet.fields.clone().into_iter());
    }

    packet.fields = known_fields.clone();

    for field in known_fields.keys() {
        if packet
            .versions
            .values()
            .all(|packet| packet.fields.contains_key(field))
        {
            packet.fields_union.insert(field.to_owned());
        } else {
            packet.fields_version_subset.insert(
                field.to_owned(),
                packet
                    .versions
                    .iter()
                    .filter_map(|(version, packet)| {
                        if packet.fields.contains_key(field) {
                            Some(version.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        }
    }
}
