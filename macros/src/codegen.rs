//! Uses the MIR as input to generate the output code.

use crate::mir;
use mir::{Conversion, FieldType, Model, ModelId, Source, Struct, StructId};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

pub fn generate(mir: mir::Protocol) -> TokenStream {
    let model_readers = model_readers(mir.models());
    let model_writers = model_writers(mir.models());

    let struct_send_definitions = struct_send_definitions(mir.structs());
    let struct_recv_definitions = struct_recv_definitions(mir.structs());

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
    let mut output = TokenStream::new();

    // Write out all fields.
    // Since this is a "send" struct, all
    // fields are required, regardless of
    // whether they're not present in some MC version.
    for field in &strukt.fields {
        let name = &field.name;
        let typ = &field.typ;

        output.extend(quote! {
            pub #name: #typ,
        });
    }

    let name = &strukt.name;

    quote! {
        #[derive(Debug, Clone)]
        pub struct #name {
            #output
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
    let mut output = TokenStream::new();

    let name = &strukt.name;

    for field in &strukt.fields {
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
            pub #name: #typ,
        })
    }

    quote! {
        #[derive(Debug, Clone)]
        pub struct #name {
            #output
        }
    }
}
