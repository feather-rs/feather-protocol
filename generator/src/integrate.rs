use crate::compile::{IdOverride, PacketDefinitions};
use crate::Version;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PacketIdentifier {
    /// Name of this packet
    pub name: String,
    /// Protocol version for which this packet is defined (e.g. V1_15_2)
    pub version: String,
}

#[derive(Debug, Clone, Default)]
pub struct IntegrationData {
    /// Packets which have identical definitions in multiple protocol
    /// versions but differ in IDs are only generated once. This mapping
    /// stores which ID to return from `Packet::id()` based on the version
    /// argument.
    ///
    /// The key in this map is a tuple of `(PacketIdentifier, Version)`.
    /// The packet identifier identifies the packet for the version it is defined;
    /// the version in the tuple stores which version the mapped ID applies to,
    /// which may differ from the packet identifier's version.
    pub packet_id_overrides: HashMap<PacketIdentifier, Vec<(Version, u32)>>,
}

pub fn integrate(packet_definitions: &[PacketDefinitions]) -> anyhow::Result<IntegrationData> {
    let mut data = IntegrationData::default();

    apply_id_overrides(packet_definitions, &mut data);

    Ok(data)
}

fn apply_id_overrides(packet_definitions: &[PacketDefinitions], data: &mut IntegrationData) {
    // iterate over id overrides in packet definitions; apply to the integration data
    packet_definitions
        .iter()
        .filter(|def| def.inherits_from.is_some())
        .map(|def| {
            (
                def,
                &def.packet_id_overrides,
                def.inherits_from.clone().unwrap(),
                def.version.clone(),
            )
        })
        .for_each(|(def, overrides, inherits, version)| {
            for (packet, over) in overrides {
                let identifier = PacketIdentifier {
                    name: packet.clone(),
                    version: inherits.clone(),
                };
                match over {
                    IdOverride::Set(id) => {
                        data.packet_id_overrides
                            .entry(identifier)
                            .or_default()
                            .push((version.clone(), *id));
                    }
                    IdOverride::Insert(id) => {
                        // apply override to each packet in this version definition with an id >= `id`
                        data.packet_id_overrides
                            .entry(identifier)
                            .or_default()
                            .push((version.clone(), *id));
                        def.serverbound
                            .iter()
                            .chain(def.clientbound.iter())
                            .filter(|packet| packet.id >= *id)
                            .filter(|packet| {
                                // check that version doesn't override the packet
                                def.clientbound
                                    .iter()
                                    .chain(def.serverbound.iter())
                                    .map(|p| &p.name)
                                    .find(|name| *name == &packet.name)
                                    .is_none()
                            })
                            .for_each(|packet| {
                                let identifier = PacketIdentifier {
                                    name: packet.name.clone(),
                                    version: inherits.clone(),
                                };
                                data.packet_id_overrides
                                    .entry(identifier)
                                    .or_default()
                                    .push((version.clone(), packet.id + 1));
                            });
                    }
                }
            }
        });
}
