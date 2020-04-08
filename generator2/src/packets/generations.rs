//! Takes in the integration data and outputs generated packet structs and write functions.

use crate::packets::integration::{IntegratedPacket, IntegrationData};
use crate::packets::parsing::{FieldType, PacketIdentifier};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

pub fn generate(data: IntegrationData) -> anyhow::Result<String> {
    let packets = generate_packets(data.packets)?;

    let res = quote! {

        #packets
    };
    Ok(res.to_string())
}

fn generate_packets(
    packets: HashMap<PacketIdentifier, IntegratedPacket>,
) -> anyhow::Result<TokenStream> {
    let mut for_sending = vec![];

    for packet in packets.values() {
        for_sending.push(generate_packet_for_sending(packet)?);
    }

    Ok(quote! {
        #(#for_sending)*
    })
}

fn generate_packet_for_sending(packet: &IntegratedPacket) -> anyhow::Result<TokenStream> {
    let mut fields = vec![];
    let mut write_to = vec![];
    let mut read_from = vec![];

    for (field_name, field_ty) in &packet.fields {
        let field_name = ident(field_name);

        let tokenized_type = tokenize_field_type(field_ty);
    }

    Ok(quote! {})
}

fn tokenize_field_type(ty: &FieldType) -> TokenStream {
    match ty {
        FieldType::Uuid => quote! { Uuid },
        FieldType::Varint => quote! { i32 },
        FieldType::U8 => quote! { u8 },
        FieldType::U16 => quote! { u16 },
        FieldType::I8 => quote! { i8 },
        FieldType::I16 => quote! { i16 },
        FieldType::I32 => quote! { i32 },
        FieldType::I64 => quote! { i64 },
        FieldType::F32 => quote! { f32 },
        FieldType::F64 => quote! { f64 },
        FieldType::Bool => quote! { bool },
        FieldType::Mapper { mappings } => quote! { undefined },
        FieldType::Pstring { .. } => quote! { String },
        FieldType::Buffer => quote! { Vec<u8> },
        FieldType::Option { of } => {
            let inner = tokenize_field_type(of);
            quote! { Option<#inner> }
        }
        FieldType::EntityMetadataLoop { .. } => quote! { BTreeMap<u8, MetaValue> },
        FieldType::Bitfield { fields } => quote! { u8 },
        FieldType::Container { .. } => quote! { undefined },
        FieldType::Switch {
            compare_to: _,
            fields: _,
        } => quote! { u8 },
        FieldType::Void => quote! { u8 },
        FieldType::Array {
            count_type: _,
            of: _,
        } => quote! { u8 },
        FieldType::RestBuffer => quote! { u8 },
        FieldType::OptionalNbt => quote! { u8 },
        FieldType::Nbt => quote! { u8 },
        FieldType::Custom { name: _ } => quote! { u8 },
    }
}

fn ident(s: impl AsRef<str>) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}
