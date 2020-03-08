//! Generation of Rust packet code from the compiled packet definitions.

use crate::compile::{
    FieldFrom, FieldType, Packet, PacketDefinitions, PacketField, StructField, StructOrEnum,
    ValueFrom,
};
use crate::generate::{
    actual_field_type, ident, read_from_statement, tokenize_field_type, write_to_statement,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub fn generate_packets(defs: &PacketDefinitions) -> anyhow::Result<TokenStream> {
    let clientbound = generate_packet_set(defs, &defs.clientbound)?;
    let serverbound = generate_packet_set(defs, &defs.serverbound)?;

    let res = quote! {
        use crate::{Packet, PacketReader, DynPacket, BlockPosition, Node, Slot};
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
    let read_from = generate_packet_read_from_fn(packet, &analysis);

    let name = ident(&packet.name);

    let impl_start = if analysis.needs_type_parameter {
        quote! { impl <P> #name <P> where P: Provider }
    } else {
        quote! { impl #name }
    };

    let impl_packet_start = if analysis.needs_type_parameter {
        quote! { impl <P> Packet for #name <P> where P: Provider }
    } else {
        quote! { impl Packet for #name }
    };

    let reader_type_parameter = if analysis.needs_type_parameter {
        Some(quote! { <P: Provider> })
    } else {
        None
    };

    let reader_name = ident(format!("{}Reader", packet.name));
    let reader_name = if analysis.needs_type_parameter {
        quote! { #reader_name <P> }
    } else {
        quote! { #reader_name }
    };

    let reader_impl_start = if analysis.needs_type_parameter {
        quote! { impl <P> PacketReader for #reader_name where P: Provider }
    } else {
        quote! { impl PacketReader for #reader_name }
    };

    let reader_fields = if analysis.needs_type_parameter {
        quote! { (std::marker::PhantomData<P>) }
    } else {
        quote! {}
    };

    let reader_default_impl = if analysis.needs_type_parameter {
        quote! {
            impl <P> Default for #reader_name where P: Provider {
                fn default() -> Self {
                    Self(std::marker::PhantomData)
                }
            }
        }
    } else {
        quote! {
            impl Default for #reader_name {
                fn default() -> Self {
                    #reader_name
                }
            }
        }
    };

    let res = quote! {
        #def

        #impl_packet_start {
            #id
            #write_to
        }

        #[derive(Debug, Clone, Copy)]
        pub struct #reader_name #reader_fields;
        #reader_default_impl

        #reader_impl_start {
            #read_from
        }
    };
    Ok(res)
}

fn generate_packet_def(packet: &Packet, analysis: &PacketAnalysis) -> TokenStream {
    let fields: Vec<_> = packet
        .fields
        .iter()
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
                     #fname.into_iter().for_each(|x| #write_elem) ;
                })
            }
            _ => statements.push(write_to_statement(&field.inner, true)),
        }
    }
    let res = quote! {
        fn write_to(self, buf: &mut BytesMut) {
            let version = VERSION;
            #(#statements)*
        }
    };
    res
}

fn generate_packet_read_from_fn(packet: &Packet, analysis: &PacketAnalysis) -> TokenStream {
    let packet_name = ident(&packet.name);
    let mut statements = vec![];
    let fields: Vec<_> = packet
        .fields
        .iter()
        .filter(|field| field.value_from.is_none())
        .map(|field| {
            let fname = ident(&field.name);
            quote! { #fname }
        })
        .collect();

    for field in &packet.fields {
        let field_var = if let Some(ref value_from) = field.value_from {
            match value_from {
                ValueFrom::EnumRepr { field } => ident(format!("{}_repr", field)),
                ValueFrom::ArrayLength { field } => ident(format!("{}_len", field)),
            }
        } else {
            ident(&field.name)
        };

        match &field.ty {
            FieldType::Array(of) => {
                statements.push(generate_array_field_read_from(field, analysis, of))
            }
            FieldType::StructOrEnum { name } => {
                statements.push(generate_struct_field_read_from(field, analysis, name))
            }
            _ => {
                let field = StructField {
                    name: field_var.to_string(),
                    ty: field.ty.clone(),
                    ty_from: field.ty_from.clone(),
                };
                statements.push(read_from_statement(&field))
            }
        }
    }

    let type_parameter = if analysis.needs_type_parameter {
        quote! { ::<P> }
    } else {
        quote! {}
    };

    let res = quote! {
        fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
            let version = VERSION;
            #(#statements )*

            let packet = #packet_name #type_parameter {
                #(#fields ,)*
            };
            Ok(smallbox::smallbox!(packet))
        }
    };
    res
}

fn generate_array_field_read_from(
    field: &PacketField,
    analysis: &PacketAnalysis,
    of: &Box<FieldType>,
) -> TokenStream {
    let fname = ident(&field.name);
    let read_elem = read_from_statement(&StructField {
        name: String::from("elem"),
        ty: *of.clone(),
        ty_from: None,
    });
    if analysis.array_lengths.contains_key(field) {
        // length field was already read: use it
        let length_var = ident(format!("{}_len", field.name));

        quote! {
            let mut #fname = vec![];
            for _ in 0..#length_var {
                #read_elem
                #fname.push(elem);
            }
        }
    } else {
        // infer length of array

        quote! {
            let mut #fname = vec![];
            while buf.has_remaining() {
                #read_elem
                #fname.push(elem);
            }
        }
    }
}

fn generate_struct_field_read_from(
    field: &PacketField,
    analysis: &PacketAnalysis,
    name: &str,
) -> TokenStream {
    let fname = ident(&field.name);
    let typename = ident(name);

    if let Some(repr_field) = analysis.enum_reprs.get(field) {
        let repr_field = *repr_field;

        let repr_var = ident(format!("{}_repr", field.name));

        quote! {
            let #fname = #typename::<P>::read_from(buf, #repr_var as i64)?;
        }
    } else {
        quote! {
            let #fname = #typename::<P>::read_from(buf)?;
        }
    }
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
