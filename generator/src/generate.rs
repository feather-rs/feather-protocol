use crate::{
    intermediate::{Direction, Field, Root, Stage, Type},
    model::FieldType,
};
use heck::CamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;

/// Given a protocol `Root`, generates code for that protocol.
pub fn generate(model: &Root) -> String {
    let types = generate_types(&model.types);

    let modules = generate_modules(&types, model);

    modules.to_string()
}

/// Generates a mapping from type name => type definition
/// (includes impl Readable and Writeable).
fn generate_types<'a>(types: &[Type<'a>]) -> HashMap<&'a str, TokenStream> {
    types
        .iter()
        .map(|typ| (typ.name, generate_type(typ)))
        .collect()
}

fn generate_type(typ: &Type) -> TokenStream {
    let ident = mk_ident(typ.name.to_camel_case());

    let definition = generate_type_def(typ, &ident);
    let readable = generate_readable(typ, &ident);

    quote! {
        #definition
        #readable
    }
}

fn generate_type_def(typ: &Type, ident: &Ident) -> TokenStream {
    let fields = typ
        .fields
        .iter()
        .map(|field| quote! { pub #field })
        .collect::<Vec<_>>();

    quote! {
        #[derive(Debug, Clone)]
        pub struct #ident {
            #(#fields,)*
        }
    }
}

fn generate_readable(typ: &Type, ident: &Ident) -> TokenStream {
    let read_statements = typ
        .fields
        .iter()
        .map(|field: &Field| {
            let ident = mk_ident(field.name);

            let fully_qualified = field.typ.tokens_fully_qualified();

            let error_ctx = format!(
                "Failed to read field `{}` of struct `{}`",
                field.name, typ.name
            );
            quote! {
                let #ident = #fully_qualified::read(buffer).context(#error_ctx)?;
            }
        })
        .collect::<Vec<_>>();

    let initializers = typ
        .fields
        .iter()
        .map(|field: &Field| {
            let ident = mk_ident(field.name);
            quote! {
                #ident
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl Readable for #ident {
            fn read(buffer: &mut Bytes) -> anyhow::Result<Self> where Self: Sized {
                #(#read_statements)*

                Ok(Self {
                    #(#initializers,)*
                })
            }
        }
    }
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let t = match self {
            FieldType::U8 => quote! { u8 },
            FieldType::U16 => quote! { u16 },
            FieldType::U32 => quote! { u32 },
            FieldType::U64 => quote! { u64 },
            FieldType::I8 => quote! { i8 },
            FieldType::I16 => quote! { i16 },
            FieldType::I32 => quote! { i32 },
            FieldType::I64 => quote! { u64 },
            FieldType::Bool => quote! { bool },
            FieldType::Array(inner, _) => quote! { Vec<#inner> },
            FieldType::Option(inner, _) => quote! { Option<#inner> },
            FieldType::VarInt => quote! { VarInt },
            FieldType::F32 => quote! { f32 },
            FieldType::F64 => quote! { f64 },
            FieldType::String => quote! { String },
        };
        tokens.extend(t);
    }
}

impl FieldType {
    pub fn tokens_fully_qualified(&self) -> TokenStream {
        match self {
            FieldType::Array(inner, _) => {
                let inner = inner.tokens_fully_qualified();
                quote! { Vec::<#inner> }
            }
            FieldType::Option(inner, _) => {
                let inner = inner.tokens_fully_qualified();
                quote! { Option::<#inner> }
            }
            x => quote! { #x },
        }
    }
}

impl<'a> ToTokens for Field<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = mk_ident(self.name);
        let typ = &self.typ;

        tokens.extend(quote! {
            #ident: #typ
        })
    }
}

fn generate_modules<'a>(types: &HashMap<&'a str, TokenStream>, model: &'a Root) -> TokenStream {
    // Make a module tree such that packet
    // structs are located in module `direction::stage`.
    let mut direction_modules = HashMap::new();

    for &(direction_name, direction) in &[
        ("clientbound", Direction::Clientbound),
        ("serverbound", Direction::Serverbound),
    ] {
        let mut stage_modules = HashMap::new();

        for &(stage_name, stage) in &[
            ("login", Stage::Login),
            ("handshake", Stage::Handshake),
            ("status", Stage::Status),
            ("play", Stage::Play),
        ] {
            // Find all packets belonging to this direction + stage.
            let packets = model
                .packets
                .iter()
                .filter(|packet| packet.stage == stage && packet.direction == direction)
                .map(|packet| packet.strukt(model))
                .map(|typ| types[&typ.name].clone())
                .collect::<Vec<_>>();

            stage_modules.insert(stage_name, quote! { #(#packets)* });
        }
        direction_modules.insert(direction_name, stage_modules);
    }

    // Produce code from the module tree.
    let mut result = TokenStream::new();

    for (direction, module) in direction_modules {
        let mut contents = TokenStream::new();

        for (stage, stage_module) in module {
            let stage_ident = mk_ident(stage);
            contents.extend(quote! {
                pub mod #stage_ident {
                    use super::*;
                    #stage_module
                }
            });
        }

        let direction_ident = mk_ident(direction);
        result.extend(quote! {
            pub mod #direction_ident {
                use super::*;
                #contents
            }
        });
    }

    quote! {
        use crate::{Readable, Writeable, VarInt};
        use anyhow::Context;
        use bytes::{Bytes, BytesMut};
        #result
    }
}

fn mk_ident(x: impl AsRef<str>) -> Ident {
    Ident::new(x.as_ref(), Span::call_site())
}
