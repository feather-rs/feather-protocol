//! Generation of Rust packet code from the compiled packet definitions.

use crate::compile::{FieldType, Packet, PacketDefinitions, PacketField, StructOrEnum, ValueFrom};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub fn generate_packets(defs: &PacketDefinitions) -> anyhow::Result<TokenStream> {
    let clientbound = generate_packet_set(defs, &defs.clientbound)?;
    let serverbound = generate_packet_set(defs, &defs.serverbound)?;

    let res = quote! {
        #clientbound
        #serverbound
    };
    Ok(res)
}

fn generate_packet_set(
    defs: &PacketDefinitions,
    packets: &[Packet],
) -> anyhow::Result<TokenStream> {
    let generated_packets = packets
        .iter()
        .map(|packet| generate_packet(defs, packet))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let res = quote! {
        #(#generated_packets)*
    };
    Ok(res)
}

#[derive(Debug, Clone, Default)]
struct PacketAnalysis<'a> {
    /// Mapping from array fields to fields used
    /// to store length of array.
    array_lengths: HashMap<&'a PacketField, &'a PacketField>,
    /// Mapping from enum fields to fields used
    /// to store repr of enum (to determine
    /// which variant to read).
    enum_reprs: HashMap<&'a PacketField, &'a PacketField>,
    /// Mapping from struct/enum fields to the actual structs
    /// and enums in the compiled definition.
    types: HashMap<&'a PacketField, &'a StructOrEnum>,
}

fn generate_packet(defs: &PacketDefinitions, packet: &Packet) -> anyhow::Result<TokenStream> {
    let analysis = analyze_packet(defs, packet)?;

    let res = quote! {};
    Ok(res)
}

fn analyze_packet<'a>(
    defs: &'a PacketDefinitions,
    packet: &'a Packet,
) -> anyhow::Result<PacketAnalysis<'a>> {
    let mut analysis = PacketAnalysis::default();

    for (field_num, field) in packet.fields.iter().enumerate() {
        if let FieldType::StructOrEnum { name: type_name } = &field.ty {
            // Determine which struct/enum in the definitiosn this refers to.
            let concrete_type = determine_concrete_type(defs, type_name)?;
            analysis.types.insert(field, concrete_type);

            // enums need a separate repr field
            if let StructOrEnum::Enum(e) = concrete_type {
                let repr_field = find_repr_field(&packet.fields, &field.name)?;
                analysis.enum_reprs.insert(field, repr_field);
            }
        }

        // arrays need a separate length field unless they're the last field in the packet
        // (in which case the length can be inferred)
        if let FieldType::Array(of) = &field.ty {
            match find_array_length_field(&packet.fields, &field.name) {
                Ok(length_field) => {
                    analysis.array_lengths.insert(field, length_field);
                }
                Err(e) => {
                    if field_num != packet.fields.len() - 1 {
                        return Err(e);
                    }
                }
            }
        }
    }

    Ok(analysis)
}

fn determine_concrete_type<'a>(
    defs: &'a PacketDefinitions,
    type_name: &str,
) -> anyhow::Result<&'a StructOrEnum> {
    match defs
        .structs_and_enums
        .iter()
        .find(|ty| ty.name() == type_name)
    {
        Some(ty) => Ok(ty),
        None => Err(anyhow::anyhow!("struct or enum {} not found", type_name)),
    }
}

fn find_repr_field<'a>(
    fields: &'a [PacketField],
    field_name: &str,
) -> anyhow::Result<&'a PacketField> {
    match fields.iter().find(|field| {
        if let Some(ValueFrom::EnumRepr { field: check_name }) = &field.value_from {
            check_name == field_name
        } else {
            false
        }
    }) {
        Some(field) => Ok(field),
        None => Err(anyhow::anyhow!(
            "no field found which determines the variant of the enum for field {}",
            field_name
        )),
    }
}

fn find_array_length_field<'a>(
    fields: &'a [PacketField],
    field_name: &str,
) -> anyhow::Result<&'a PacketField> {
    match fields.iter().find(|field| {
        if let Some(ValueFrom::ArrayLength { field: check_name }) = &field.value_from {
            check_name == field_name
        } else {
            false
        }
    }) {
        Some(field) => Ok(field),
        None => Err(anyhow::anyhow!(
            "no field found which determines the length of field {}",
            field_name
        )),
    }
}
