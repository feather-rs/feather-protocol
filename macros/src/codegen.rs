//! Uses the MIR as input to generate the output code.

use crate::mir;
use mir::{
    Conversion, Enum, EnumId, FieldType, Model, ModelId, Source, Struct, StructField, StructId,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use syn::Ident;

pub fn generate(mir: mir::Protocol) -> TokenStream {
    let model_readers = model_readers(mir.models());
    let model_writers = model_writers(mir.models());

    let struct_send_definitions = struct_send_definitions(mir.structs());
    let struct_recv_definitions = struct_recv_definitions(mir.structs());

    let enum_send_definitions = enum_send_definitions(mir.enums());
    let enum_recv_definitions = enum_recv_definitions(mir.enums());

    todo!()
}

/// Generates a mapping from model ID => code to read its fields.
fn model_readers<'a>(
    models: impl Iterator<Item = (ModelId, &'a Model)>,
) -> BTreeMap<ModelId, TokenStream> {
    models
        .map(|(id, model)| (id, model_reader(model)))
        .collect()
}

fn model_reader(model: &Model) -> TokenStream {
    let mut output = TokenStream::new();

    // Read each field.
    for field in &model.fields {
        // Read in the field from the buffer.
        let read = read(&field.typ);

        // Apply any necessary conversions.
        let conversion = field.conversion.as_ref().map(read_conversion);

        // Create a binding with the field's name.
        let name = &field.name;
        let bind = quote! { let #name = __field; };

        // Merge the above components into a single token stream.
        output.extend(quote! {
            #read
            #conversion
            #bind
        });
    }

    output
}

fn read(typ: &FieldType) -> TokenStream {
    match typ {
        FieldType::Rust(typ) => quote! {
            let __field = #typ::read(buffer, version);
        },
        FieldType::Array { length, typ } => {
            let read_inner = read(typ);
            quote! {
                if #length > 65_536 {
                    anyhow::bail!("array length too large");
                }

                let mut __field_vec = Vec::with_capacity(#length);
                for _ in 0..#length {
                    #read_inner
                    __field_vec.push(__field);
                }

                let __field = __field_vec;
            }
        }
    }
}

fn read_conversion(conversion: &Conversion) -> TokenStream {
    match conversion {
        Conversion::Primitive { from, .. } => quote! {
            let __field = __field as #from; // NB: conversion has to be backwards, since we're reading the value, not writing it.
        },
        Conversion::VarInt => quote! {
            let __field = __field.0;
        },
    }
}

/// Generates a mapping from model ID => code to write its fields.
fn model_writers<'a>(
    models: impl Iterator<Item = (ModelId, &'a Model)>,
) -> BTreeMap<ModelId, TokenStream> {
    models
        .map(|(id, model)| (id, model_writer(model)))
        .collect()
}

fn model_writer(model: &Model) -> TokenStream {
    let mut output = TokenStream::new();

    // Write each field.
    for field in &model.fields {
        // Bind the variable `__field` to the field's value.
        let name = &field.name;
        let bind = match &field.source {
            Some(source) => write_source(source),
            None => quote! { let __field = #name },
        };

        // Apply any necessary type conversions.
        let conversion = field.conversion.as_ref().map(write_conversion);

        // Write out the field.
        let write = write(&field.typ);

        // Combine the above components into one TokenStream.
        output.extend(quote! {
            #bind
            #conversion
            #write
        });
    }

    output
}

fn write_source(source: &Source) -> TokenStream {
    match source {
        Source::ArrayLength { array } => {
            quote! {
                let __field = #array.len();
            }
        }
    }
}

fn write_conversion(conversion: &Conversion) -> TokenStream {
    match conversion {
        Conversion::Primitive { to, .. } => {
            quote! {
                let __field = __field as #to;
            }
        }
        Conversion::VarInt => {
            quote! {
                let __field = crate::VarInt(__field as i32);
            }
        }
    }
}

fn write(typ: &FieldType) -> TokenStream {
    match typ {
        FieldType::Rust(_) => quote! { __field.write(buffer, version); },
        FieldType::Array { typ, .. } => {
            let write_inner = write(typ);
            quote! {
                for element in field {
                    let __field = element;
                    #write_inner
                }
            }
        }
    }
}

impl quote::ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FieldType::Rust(typ) => quote! { #typ },
            FieldType::Array { typ, .. } => quote! { Vec<#typ> },
        })
    }
}

/// Returns a mapping from struct ID => its definition for the `send` module (struct Struct {} block)
fn struct_send_definitions<'a>(
    structs: impl Iterator<Item = (StructId, &'a Struct)>,
) -> BTreeMap<StructId, TokenStream> {
    structs
        .map(|(id, strukt)| (id, struct_send_definition(strukt)))
        .collect()
}

fn struct_send_definition(strukt: &Struct) -> TokenStream {
    let content = fields_recv_definition(&strukt.fields, true);
    let name = &strukt.name;

    quote! {
        #[derive(Debug, Clone)]
        pub struct #name {
            #content
        }
    }
}

/// Returns a mapping from struct ID => its definition for the `recv` module.
fn struct_recv_definitions<'a>(
    structs: impl Iterator<Item = (StructId, &'a Struct)>,
) -> BTreeMap<StructId, TokenStream> {
    structs
        .map(|(id, strukt)| (id, struct_recv_definition(strukt)))
        .collect()
}

fn struct_recv_definition(strukt: &Struct) -> TokenStream {
    let content = fields_recv_definition(&strukt.fields, true);
    let name = &strukt.name;

    quote! {
        #[derive(Debug, Clone)]
        pub struct #name {
            #content
        }
    }
}

fn enum_send_definitions<'a>(
    enums: impl Iterator<Item = (EnumId, &'a Enum)>,
) -> BTreeMap<EnumId, TokenStream> {
    enums
        .map(|(id, en)| (id, enum_send_definition(en)))
        .collect()
}

fn enum_recv_definitions<'a>(
    enums: impl Iterator<Item = (EnumId, &'a Enum)>,
) -> BTreeMap<EnumId, TokenStream> {
    enums
        .map(|(id, en)| (id, enum_recv_definition(en)))
        .collect()
}

fn enum_send_definition(en: &Enum) -> TokenStream {
    enum_definition(en, |fields| fields_send_definition(fields, false))
}

fn enum_recv_definition(en: &Enum) -> TokenStream {
    enum_definition(en, |fields| fields_recv_definition(fields, false))
}

fn enum_definition(
    en: &Enum,
    mut fields: impl FnMut(&[StructField]) -> TokenStream,
) -> TokenStream {
    let mut content = TokenStream::new();

    // Generate each variant.
    for variant in &en.all_variants {
        let fields = fields(&variant.fields);
        let name = &variant.name;

        content.extend(quote! {
            #name {
                #fields
            },
        });
    }

    let name = &en.name;

    quote! {
        #[derive(Debug, Clone)]
        pub enum #name {
            #content
        }
    }
}

fn fields_send_definition(fields: &[StructField], pub_token: bool) -> TokenStream {
    let mut output = TokenStream::new();

    let pub_token = if pub_token {
        Some(quote! { pub })
    } else {
        None
    };

    // Write out all fields.
    // Since this is a "send" struct, all
    // fields are required, regardless of
    // whether they're not present in some MC version.
    for field in fields {
        let name = &field.name;
        let typ = &field.typ;

        output.extend(quote! {
            #pub_token #name: #typ,
        });
    }

    output
}

fn fields_recv_definition(fields: &[StructField], pub_token: bool) -> TokenStream {
    let mut output = TokenStream::new();

    let pub_token = if pub_token {
        Some(quote! { pub })
    } else {
        None
    };

    for field in fields {
        let typ = &field.typ;

        // Fields which aren't present in all versions
        // may or may not be present, since this struct
        // was received from a client/server with an arbitrary
        // MC protocol. Therefore, if a field isn't present
        // in all versions, we make it an `Option<T>`.
        let typ = if field.always_present {
            quote! { #typ }
        } else {
            quote! { Option<#typ> }
        };
        let name = &field.name;

        output.extend(quote! {
            #pub_token #name: #typ,
        });
    }

    output
}

/// Generates a mapping from struct ID => code to read that struct.
fn struct_readers<'a>(
    structs: impl Iterator<Item = (StructId, &'a Struct)>,
    model_readers: &BTreeMap<ModelId, TokenStream>,
    mir: &mir::Protocol,
) -> BTreeMap<StructId, TokenStream> {
    structs
        .map(|(id, strukt)| (id, struct_reader(strukt, model_readers, mir)))
        .collect()
}

fn struct_reader(
    strukt: &Struct,
    model_readers: &BTreeMap<ModelId, TokenStream>,
    mir: &mir::Protocol,
) -> TokenStream {
    // Match over the protocol version to read the fields.
    let match_arms: Vec<TokenStream> = strukt
        .versions
        .iter()
        .map(|(&version_id, &model_id)| {
            let version = mir.version(version_id);
            let version_ident = &version.ident;

            let model = mir.model(model_id);

            let read_fields = &model_readers[&model_id];

            // To initialize the struct, we have to cover each field, even
            // if the field doesn't exist in this version. Fields not
            // present in this version are set to `None`.
            let field_initializers = strukt.fields.iter().map(|field| {
                // See if a field with the same ident exists in this version.
                let exists_in_this_version =
                    model.fields.iter().any(|field2| field2.name == field.name);
                let name = &field.name;

                if exists_in_this_version {
                    if field.always_present {
                        quote! { #name }
                    } else {
                        quote! { #name: Some(#name) }
                    }
                } else {
                    assert!(!field.always_present);
                    quote! { #name: None }
                }
            });

            quote! {
                crate::ProtocolVersion::#version_ident => {
                    #read_fields
                    Ok(Self {
                        #(#field_initializers,)*
                    })
                }
            }
        })
        .collect();

    let name = &strukt.name;

    quote! {
        impl crate::Readable for #name {
            fn read_from(buffer: &mut std::io::Cursor<&[u8]>, version: crate::ProtocolVersion) -> anyhow::Result<Self>
                where Self: Sized
            {
                match version {
                    #(#match_arms,)*
                    version => anyhow::bail!("packet '{}' does not exist in version {:?}", stringify!(#name), version),
                }
            }
        }
    }
}

/// Generates a mapping from struct ID => code to
/// write that struct.
fn struct_writers<'a>(
    structs: impl Iterator<Item = (StructId, &'a Struct)>,
    model_writers: &BTreeMap<ModelId, TokenStream>,
    mir: &mir::Protocol,
) -> BTreeMap<StructId, TokenStream> {
    structs
        .map(|(id, strukt)| (id, struct_writer(strukt, model_writers, mir)))
        .collect()
}

fn struct_writer(
    strukt: &Struct,
    model_writers: &BTreeMap<ModelId, TokenStream>,
    mir: &mir::Protocol,
) -> TokenStream {
    // Bind all fields of the struct to local variables.
    let fields: Vec<&Ident> = strukt.fields.iter().map(|field| &field.name).collect();
    let bind = quote! {
        let Self { #(#fields,)* } = self;
    };

    // Match over the protocol version and write the fields.
    let match_arms: Vec<TokenStream> = strukt
        .versions
        .iter()
        .map(|(&version_id, model_id)| {
            let version = mir.version(version_id);
            let version_ident = &version.ident;

            let model_writer = &model_writers[model_id];

            quote! {
                crate::ProtocolVersion::#version_ident => {
                    #model_writer
                }
            }
        })
        .collect();

    let name = &strukt.name;

    quote! {
        impl crate::Writeable for #name {
            fn write_to(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) {
                #bind
                match version {
                    #(#match_arms)*
                    version => panic!("cannot write packet '{}' for version {:?}", stringify!(#name), version),
                }
            }
        }
    }
}
