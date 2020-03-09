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
    /// Mapping from packet identifiers to the versions which this packet version
    /// is used for.
    pub packet_versions: HashMap<PacketIdentifier, Vec<Version>>,
}

pub fn integrate(packet_definitions: &[PacketDefinitions]) -> anyhow::Result<IntegrationData> {
    let mut data = IntegrationData::default();

    apply_id_overrides(packet_definitions, &mut data);

    find_packet_versions(packet_definitions, &mut data);

    Ok(data)
}

fn apply_id_overrides(packet_definitions: &[PacketDefinitions], data: &mut IntegrationData) {
    for def in packet_definitions {
        let ids = determine_packet_ids_for(def, packet_definitions);

        if def.inherits_from.is_some() {
            for (identifier, id) in ids.iter() {
                data.packet_id_overrides
                    .entry(identifier.clone())
                    .or_default()
                    .push((def.version.clone(), *id));
            }
        }
    }
}

fn determine_packet_ids_for(
    def: &PacketDefinitions,
    defs: &[PacketDefinitions],
) -> HashMap<PacketIdentifier, u32> {
    let mut ids = if let Some(ref inherits) = def.inherits_from {
        determine_packet_ids_for(defs.iter().find(|d| &d.version == inherits).unwrap(), defs)
    } else {
        let mut ids = HashMap::new();
        for packet in def.serverbound.iter().chain(def.clientbound.iter()) {
            ids.insert(
                PacketIdentifier {
                    name: packet.name.clone(),
                    version: def.version.clone(),
                },
                packet.id,
            );
        }
        ids
    };

    // apply overrides
    for (packet_name, over) in def.packet_id_overrides.iter() {
        let identifier = ids
            .keys()
            .find(|identifier| &identifier.name == packet_name)
            .unwrap()
            .clone();
        let old_id = ids[&identifier];

        match over {
            IdOverride::Set(id) => {
                ids.insert(identifier, *id);
            }
            IdOverride::Insert(id) => {
                let mut changes = vec![];
                for (other_identifier, other_id) in ids.iter().filter(|(_, i)| **i >= *id) {
                    changes.push((other_identifier.clone(), *other_id + 1));
                }

                changes.drain(..).for_each(|(key, value)| {
                    ids.insert(key, value);
                });

                for (other_identifier, other_id) in
                    ids.iter().filter(|(_, i)| **i <= *id && **i > old_id)
                {
                    changes.push((other_identifier.clone(), *other_id - 1));
                }

                changes.drain(..).for_each(|(key, value)| {
                    ids.insert(key, value);
                });

                ids.insert(identifier, *id);
            }
        }
    }

    ids
}

fn find_packet_versions(defs: &[PacketDefinitions], data: &mut IntegrationData) {
    for def in defs {
        let mut packets = def.clientbound.iter().chain(def.serverbound.iter());

        for packet in packets.by_ref() {
            let identifier = PacketIdentifier {
                name: packet.name.clone(),
                version: def.version.clone(),
            };

            data.packet_versions
                .entry(identifier)
                .or_default()
                .push(def.version.clone());
        }
        packets = def.clientbound.iter().chain(def.serverbound.iter());

        if let Some(ref inherits) = def.inherits_from {
            for identifier in trace_inherits(inherits, defs) {
                // don't add if the packet is overridden here
                if packets
                    .by_ref()
                    .any(|packet| packet.name == identifier.name)
                {
                    continue;
                }
                packets = def.clientbound.iter().chain(def.serverbound.iter());

                data.packet_versions
                    .entry(identifier)
                    .or_default()
                    .push(def.version.clone());
            }
        }
    }
}

fn trace_inherits(inherits: &Version, defs: &[PacketDefinitions]) -> Vec<PacketIdentifier> {
    let def = defs
        .iter()
        .find(|d| &d.version == inherits)
        .expect("no matching version");

    let mut res = vec![];

    if let Some(ref inherits) = def.inherits_from {
        res.append(&mut trace_inherits(inherits, defs));
    }

    for packet in def
        .clientbound
        .iter()
        .chain(def.serverbound.iter())
        .filter(|packet| packet.manual.is_none())
    {
        let identifier = PacketIdentifier {
            name: packet.name.clone(),
            version: inherits.clone(),
        };
        res.push(identifier);
    }

    res
}
