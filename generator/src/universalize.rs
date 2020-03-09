//! Generates wrapper structs over packets for all versions.
//!
//! # Behavior
//! * For each packet name, generates a struct __for sending__ which contains
//! the union of the sets of fields for each version of this packet.
//! * Generates a `try_into_version` function which returns a `Option<DynPacket>`.
//! If the packet can't be written for a given version (e.g. it sends beehive
//! data, but the packet is being sent to 1.14, which doesn't support bees),
//! returns `None`. Otherwise, returns the corresponding version of the packet.
//!
//! * For each packet name, also generates a struct __for receiving__
//! which is like the wrapper struct for sending, but any fields which
//! are not present in all versions are wrapped in an `Option`.
//! * For this struct, generates a `from_version` function which is given
//! a `DynPacket` and converts this packet to the wrapped type.
//!
//! * For each struct/enum name, does the same thing as above.

use crate::compile::{Packet, PacketDefinitions, PacketField};
use crate::generate::packet::packet_needs_type_parameter;
use crate::generate::{actual_field_type, ident, tokenize_field_type};
use crate::integrate::{IntegrationData, PacketIdentifier};
use crate::Version;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::iter;

pub fn universalize(
    defs: &[PacketDefinitions],
    integration: &IntegrationData,
) -> anyhow::Result<String> {
    let analysis = analyze(defs)?;

    let packets = universalize_packets(defs, &analysis, integration)?;

    Ok(quote! {
        #![allow(warnings)]
        use bytes::{Bytes, BytesMut, Buf, BufMut};
        use crate::{BytesExt, BytesMutExt, Provider, Error, ProtocolVersion};
        use super::*;

        #packets
    }
    .to_string())
}

#[derive(Default, Debug, Clone)]
struct Analysis {
    /// Mapping from packet names to union of all fields for the packet.
    union_fields: HashMap<String, Vec<PacketField>>,
    /// Mapping from packet names to tuple (field, bool) where the bool
    /// indicates whether all versions of this packet have that field.
    union_opt_fields: HashMap<String, Vec<(PacketField, bool)>>,
    /// Mapping from packet names to the fields of each version for the packet.
    versioned_fields: HashMap<String, Vec<Vec<PacketField>>>,
    /// Mapping from packet names to the different versions of the packet.
    packets: HashMap<String, Vec<(Packet, Version)>>,
}

fn analyze(defs: &[PacketDefinitions]) -> anyhow::Result<Analysis> {
    let mut analysis = Analysis::default();

    // iterate over packets defined with same name
    for def in defs {
        for packet in def.clientbound.iter().chain(def.serverbound.iter()) {
            if analysis.union_opt_fields.contains_key(&packet.name) {
                continue;
            }

            // determine packets with same name
            let mut field_sets: Vec<Vec<PacketField>> = vec![];
            let mut versions = vec![];
            for (p, d) in defs
                .iter()
                .flat_map(|d| {
                    d.serverbound
                        .iter()
                        .chain(d.clientbound.iter())
                        .zip(iter::repeat(d))
                })
                .filter(|(p, _)| p.name == packet.name)
            {
                let mut fields = p.fields.clone();
                fields.sort();
                field_sets.push(fields);
                versions.push((p.clone(), d.version.clone()));
            }

            analysis.packets.insert(packet.name.clone(), versions);

            // determine union of all fields
            let union = field_sets
                .iter()
                .map(|x| x.iter())
                .flatten()
                .cloned()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            analysis.union_fields.insert(packet.name.clone(), union);

            // determine which fields exist in all versions
            let mut opt_fields = vec![];

            // count instances of each field
            let mut count_per_field: BTreeMap<PacketField, usize> = BTreeMap::new();
            for field in field_sets.iter().map(|x| x.iter()).flatten() {
                *count_per_field.entry(field.clone()).or_default() += 1;
            }

            // fill opt_fields based on count_per_field
            for (field, field_count) in count_per_field {
                let is_optional = field_count == field_sets.len();
                opt_fields.push((field, !is_optional));
            }

            analysis
                .union_opt_fields
                .insert(packet.name.clone(), opt_fields);

            analysis
                .versioned_fields
                .insert(packet.name.clone(), field_sets);
        }
    }

    Ok(analysis)
}

fn universalize_packets(
    defs: &[PacketDefinitions],
    analysis: &Analysis,
    integration: &IntegrationData,
) -> anyhow::Result<TokenStream> {
    let mut packets = vec![];

    for (packet_name, union_opt_fields) in analysis.union_opt_fields.iter() {
        packets.push(universalize_packet(
            packet_name.as_str(),
            union_opt_fields.as_slice(),
            defs,
            analysis,
            integration,
        )?);
    }

    let res = quote! {
        #(#packets )*
    };
    Ok(res)
}

fn universalize_packet(
    packet_name: &str,
    union_opt_fields: &[(PacketField, bool)],
    defs: &[PacketDefinitions],
    analysis: &Analysis,
    integration: &IntegrationData,
) -> anyhow::Result<TokenStream> {
    let needs_type_parameter = analysis.packets[packet_name]
        .iter()
        .any(|packet| packet_needs_type_parameter(&packet.0));

    let fields: Vec<_> = union_opt_fields
        .iter()
        .map(|(field, _)| field)
        .filter_map(|field| {
            let name = ident(&field.name);
            let ty = tokenize_field_type(&actual_field_type(field));
            if field.value_from.is_some() {
                None
            } else {
                Some(quote! { pub #name: #ty })
            }
        })
        .collect();

    let name = ident(packet_name);

    let type_parameter = if needs_type_parameter {
        quote! { <P: Provider> }
    } else {
        quote! {}
    };

    let impl_universal = if needs_type_parameter {
        quote! { impl <P> UniversalPacketSend for #name <P> where P: Provider }
    } else {
        quote! { impl UniversalPacketSend for #name }
    };

    let mut try_into_version = vec![];
    for (packet, version) in &analysis.packets[packet_name] {
        let version_module = ident(version.to_lowercase());

        let possible_versions = dbg!(&integration.packet_versions)
            .get(dbg!(&PacketIdentifier {
                name: packet.name.clone(),
                version: version.clone(),
            }))
            .unwrap()
            .iter()
            .map(|ver| {
                let ver = ident(ver);
                quote! { ProtocolVersion::#ver }
            })
            .collect::<Vec<_>>();

        let mut fields = vec![];
        for field in packet
            .fields
            .iter()
            .filter(|field| field.value_from.is_none())
        {
            let fname = ident(&field.name);
            fields.push(quote! {
                #fname: self.#fname
            })
        }

        try_into_version.push(quote! {
            #(#possible_versions)|* => {
                Some(smallbox::smallbox!(#version_module::#name {
                    #(#fields ,)*
                }))
            }
        })
    }

    let res = quote! {
        pub struct #name #type_parameter {
            #(#fields ,)*
        }

        #impl_universal {
            fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
                match version {
                    #(#try_into_version ,)*
                    _ => None,
                }
            }
        }
    };
    Ok(res)
}
