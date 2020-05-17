use crate::{
    intermediate::{Direction, Field, Root, Stage, Type},
    model::{ArrayLength, FieldType, OptionTag},
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
    let writeable = generate_writeable(typ, &ident);

    quote! {
        #definition
        #readable
        #writeable
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

            let read = field.typ.tokens_for_read(field.name);
            quote! {
                let #ident = #read;
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

fn generate_writeable(typ: &Type, ident: &Ident) -> TokenStream {
    // Generate code to write each field.
    let statements = typ
        .fields
        .iter()
        .map(|field| {
            let ident = mk_ident(field.name);
            field.typ.tokens_for_write(&quote! { (&self.#ident) })
        })
        .collect::<Vec<_>>();

    quote! {
        impl Writeable for #ident {
            fn write(&self, buffer: &mut BytesMut) {
                #(#statements)*
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

    fn tokens_for_read(&self, field_name: &str) -> TokenStream {
        match self {
            FieldType::Array(inner, length) => {
                let read_length = match length {
                    ArrayLength::Prefixed(typ) => {
                        let read_typ = typ.tokens_for_read(field_name);
                        quote! { let length = usize::try_from(#read_typ)?; }
                    }
                    ArrayLength::Inferred => {
                        quote! { let length = buffer.remaining(); }
                    }
                };
                let read_inner = inner.tokens_for_read(field_name);

                quote! {
                    {
                        #read_length
                        if length > std::u16::MAX as usize {
                            bail!("array length `{}` exceeds maximum allowed array length", length);
                        }
                        let mut buf = Vec::with_capacity(length);
                        for _ in 0..length {
                            buf.push(#read_inner);
                        }
                        buf
                    }
                }
            }
            FieldType::Option(inner, tag) => {
                let read_tag = match tag {
                    OptionTag::Prefixed(inner) => {
                        let read_inner = inner.tokens_for_read(field_name);
                        quote! {
                            let present = #read_inner;
                        }
                    }
                };

                let read_inner = inner.tokens_for_read(field_name);
                quote! {
                    {
                        #read_tag
                        if present {
                            Some(#read_inner)
                        } else {
                            None
                        }
                    }
                }
            }
            typ => {
                let error_ctx = format!("Failed to read field `{}`", field_name);
                let typ = typ.tokens_fully_qualified();
                quote! { #typ::read(buffer).context(#error_ctx)? }
            }
        }
    }

    fn tokens_for_write(&self, ident: &TokenStream) -> TokenStream {
        match self {
            FieldType::Array(inner, length) => tokens_for_array_write(inner, length, ident),
            FieldType::Option(inner, tag) => tokens_for_option_write(inner, tag, ident),
            _ => {
                quote! {
                    #ident.write(buffer);
                }
            }
        }
    }
}

fn tokens_for_array_write(
    inner: &FieldType,
    length: &ArrayLength,
    ident: &TokenStream,
) -> TokenStream {
    let write_length = match length {
        ArrayLength::Prefixed(typ) => {
            let write_length = typ.tokens_for_write(&quote! { length });
            let typ = typ.tokens_fully_qualified();
            quote! {
                let length = #typ::from(#ident.len());
                #write_length
            }
        }
        ArrayLength::Inferred => quote! {}, // write nothing - length is implcit
    };

    let write_inner = inner.tokens_for_write(&quote! { elem });
    quote! {
        #write_length

        for elem in #ident {
            #write_inner
        }
    }
}

fn tokens_for_option_write(inner: &FieldType, tag: &OptionTag, ident: &TokenStream) -> TokenStream {
    let write_tag = match tag {
        OptionTag::Prefixed(inner) => {
            let write_tag = inner.tokens_for_write(&quote! { tag });

            quote! {
                let tag = #ident.is_some();
                #write_tag
            }
        }
    };

    let write_inner = inner.tokens_for_write(&quote! { val });

    quote! {
        #write_tag
        if let Some(val) = #ident {
            #write_inner
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
        use anyhow::{Context, bail};
        use bytes::{Bytes, BytesMut, Buf, BufMut};
        use std::convert::TryFrom;
        #result
    }
}

fn mk_ident(x: impl AsRef<str>) -> Ident {
    Ident::new(x.as_ref(), Span::call_site())
}
