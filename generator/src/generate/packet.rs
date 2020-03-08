//! Generation of Rust packet code from the compiled packet definitions.

use crate::compile::{
    FieldFrom, FieldType, Packet, PacketDefinitions, PacketField, StructField, StructOrEnum,
    ValueFrom,
};
use crate::generate::{actual_field_type, ident, tokenize_field_type, write_to_statement};
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
    /// Whether this packet needs a <P: Provider> type parameter.
    needs_type_parameter: bool,
}

fn generate_packet(defs: &PacketDefinitions, packet: &Packet) -> anyhow::Result<TokenStream> {
    let analysis = analyze_packet(defs, packet)?;

    let def = generate_packet_def(packet, &analysis);
    let id = generate_packet_id_fn(packet);
    let write_to = generate_packet_write_to_fn(packet);

    let name = ident(&packet.name);

    let impl_start = if analysis.needs_type_parameter {
        quote! { impl <P> #name <P> where P: Provider }
    } else {
        quote! { impl #name }
    };

    let res = quote! {
        #def

        #impl_start {
            #id
            #write_to
        }
    };
    Ok(res)
}

fn generate_packet_def(packet: &Packet, analysis: &PacketAnalysis) -> TokenStream {
    let fields: Vec<_> = packet
        .fields
        .iter()
        .map(|field| {
            let name = ident(&field.name);
            let ty = tokenize_field_type(&actual_field_type(field));
            quote! { #name: #ty }
        })
        .collect();

    let name = ident(&packet.name);

    let type_parameter = if analysis.needs_type_parameter {
        Some(quote! { <P: Provider> })
    } else {
        None
    };

    quote! {
        #[derive(Debug, Clone)]
        pub struct #name #type_parameter {
            #(#fields ,)*
        }
    }
}

fn generate_packet_id_fn(packet: &Packet) -> TokenStream {
    let id = packet.id;
    let name = &packet.name;
    quote! {
        fn id(&self) -> u32 { #id }
        fn name(&self) -> &'static str { #name }
    }
}

fn generate_packet_write_to_fn(packet: &Packet) -> TokenStream {
    let mut statements = vec![];

    // Write each field
    for field in &packet.fields {
        let fname = ident(&field.name);

        statements.push(if let Some(ref value_from) = field.value_from {
            let as_clause = tokenize_field_type(&field.ty);
            match value_from {
                ValueFrom::ArrayLength { field } => {
                    let field = ident(field);
                    quote! { let #fname = self.#field.len() as #as_clause; }
                }
                ValueFrom::EnumRepr { field } => {
                    let field = ident(field);
                    quote! { let #fname = self.#field.repr() as #as_clause; }
                }
            }
        } else {
            quote! { let #fname = self.#fname; }
        });

        match &field.ty {
            FieldType::Array(of) => {
                // use fake struct field
                let write_elem = write_to_statement(
                    &StructField {
                        name: String::from("x"),
                        ty: *of.clone(),
                        ty_from: None,
                    },
                    false,
                );
                statements.push(quote! {
                     #fname.iter().for_each(|x| #write_elem) ;
                })
            }
            _ => statements.push(write_to_statement(&field.inner, true)),
        }
    }
    let res = quote! {
        fn write_to(self, buf: &mut BytesMut) {
            #(#statements)*
        }
    };
    res
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

    analysis.needs_type_parameter = packet_needs_type_parameter(packet);

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

fn packet_needs_type_parameter(packet: &Packet) -> bool {
    packet.fields.iter().any(|field| {
        if let Some(_) = field.ty_from {
            return true;
        }
        match &field.ty {
            FieldType::StructOrEnum { .. } | FieldType::Block | FieldType::Item => true,
            FieldType::Array(of) => {
                if let FieldType::StructOrEnum { .. } = &*of.clone() {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    })
}
