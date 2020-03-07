use crate::compile::{
    Enum, EnumVariant, FieldFrom, FieldType, PacketDefinitions, Struct, StructField, StructOrEnum,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn generate_packet_code(defs: &PacketDefinitions) -> anyhow::Result<String> {
    let structs_and_enums = generate_structs_and_enums(defs)?;

    let output = quote! {
        #structs_and_enums
    };

    Ok(output.to_string())
}

fn generate_structs_and_enums(defs: &PacketDefinitions) -> anyhow::Result<TokenStream> {
    let mut tokens = vec![];

    for se in &defs.structs_and_enums {
        match se {
            StructOrEnum::Struct(s) => tokens.push(generate_struct(s)),
            StructOrEnum::Enum(e) => tokens.push(generate_enum(e)),
        }
    }

    Ok(quote! {
        use bytes::{Bytes, BytesMut, Buf, BufMut};
        use crate::{BytesExt, BytesMutExt, Provider};
        const VERSION: crate::ProtocolVersion = ProtocolVersion::V1_15_2;
        #(#tokens)*
    })
}

fn generate_struct(s: &Struct) -> TokenStream {
    dbg!(s);
    let name = ident(&s.name);
    let mut fields = vec![];
    let mut write_to = vec![];
    let mut read_from = vec![];
    let mut field_initializers = vec![];

    for field in &s.fields {
        let fname = ident(&field.name);
        let ty = tokenize_field_type(&actual_field_type(field));

        fields.push(quote! {
            #fname: #ty
        });

        read_from.push(read_from_statement(field));
        let write = write_to_statement(field);

        write_to.push(quote! { let #fname = self.#fname; #write });

        field_initializers.push(quote! { #fname, })
    }

    let res = quote! {
        pub struct #name<P: Provider> {
            #(#fields ,)*
        }

        impl <P> #name<P> where P: Provider {
            pub fn read_from(buf: &mut bytes::Bytes) -> anyhow::Result<Self> {
                let version = VERSION;
                #(#read_from)*

                Ok(Self {
                    #(#field_initializers)*
                })
            }

            pub fn write_to(&self, buf: &mut BytesMut) {
                let version = VERSION;
                #(#write_to)*
            }
        }
    };
    res
}

fn generate_enum(e: &Enum) -> TokenStream {
    dbg!(e);
    let name = ident(&e.name);

    let mut defs = vec![];

    for variant in &e.variants {
        let (def, read_from, write_to, repr, from_repr) = generate_enum_variant(e, variant);
        defs.push(def);
    }

    let res = quote! {
        pub enum #name<P: Provider> {
            #(#defs ,)*
        }
    };
    res
}

fn generate_enum_variant(
    e: &Enum,
    variant: &EnumVariant,
) -> (
    TokenStream,
    TokenStream,
    TokenStream,
    TokenStream,
    TokenStream,
) {
    (
        generate_enum_variant_def(e, variant),
        generate_enum_variant_read_from(e, variant),
        generate_enum_variant_write_to(e, variant),
        generate_enum_variant_repr(e, variant),
        generate_enum_variant_from_repr(e, variant),
    )
}

fn generate_enum_variant_def(e: &Enum, variant: &EnumVariant) -> TokenStream {
    let name = ident(&variant.name);

    let mut fields = vec![];
    for field in &variant.fields {
        let fname = ident(&field.name);
        let ty = tokenize_field_type(&actual_field_type(field));

        fields.push(quote! {
            #fname: #ty
        });
    }

    quote! {
        #name {
            #(#fields, )*
        }
    }
}

fn generate_enum_variant_read_from(e: &Enum, variant: &EnumVariant) -> TokenStream {
    let name = ident(&variant.name);
    let enum_name = ident(&e.name);

    let mut read_from = vec![];
    let mut finish = vec![];

    for field in &variant.fields {
        read_from.push(read_from_statement(field));

        let fname = ident(&field.name);
        finish.push(quote! { #fname });
    }

    let repr = variant.repr.as_enum_repr();

    quote! {
        #repr => {
            #(#read_from ;)*
            #enum_name::#name {
                #(#finish, )*
            }
        }
    }
}

fn generate_enum_variant_write_to(e: &Enum, variant: &EnumVariant) -> TokenStream {
    quote! {}
}

fn generate_enum_variant_repr(e: &Enum, variant: &EnumVariant) -> TokenStream {
    quote! {}
}

fn generate_enum_variant_from_repr(e: &Enum, variant: &EnumVariant) -> TokenStream {
    quote! {}
}

fn read_from_statement(field: &StructField) -> TokenStream {
    let f = read_fn_ident(&field.ty);

    let field_name = ident(&field.name);

    let convert = if let Some(ref ty_from) = field.ty_from {
        let tokens = ty_from.tokens_for_read(field_name.clone());
        Some(quote! {
            let #field_name = #tokens ;
        })
    } else {
        None
    };

    quote! {
        let #field_name = #f ;
        #convert
    }
}

fn read_fn_ident(ty: &FieldType) -> TokenStream {
    match ty {
        FieldType::StructOrEnum { name } => {
            let ident = ident(name);
            quote! {
                #ident::read_from(buf)?
            }
        }
        FieldType::Array(_) => panic!("struct can't have array field"),
        FieldType::OptChat => {
            quote! {
                {
                    let exists = buf.try_get_bool()?;
                    if exists {
                        Some(buf.try_get_string()?)
                    } else {
                        None
                    }
                }
            }
        }
        FieldType::Identifier | FieldType::Chat => quote! { buf.try_get_string()? },
        x => {
            let ident = ident(format!("try_get_{}", tokenize_field_type(x)));
            quote! { #ident()? }
        }
    }
}

fn write_to_statement(field: &StructField) -> TokenStream {
    let field_name = &field.name;

    let f = write_fn_ident(field_name, &field.ty);

    let field_name = ident(field_name);

    let convert = if let Some(ref ty_from) = field.ty_from {
        let tokens = ty_from.tokens_for_write(quote! { self.#field_name });

        Some(quote! {
            let #field_name = #tokens
        })
    } else {
        None
    };

    quote! { #convert ; #f ; }
}

fn write_fn_ident(field_name: &str, ty: &FieldType) -> TokenStream {
    let field_name = ident(field_name);
    match ty {
        FieldType::StructOrEnum { name: _ } => {
            quote! {
                #field_name.write_to(buf)
            }
        }
        FieldType::OptChat => {
            quote! {
                buf.put_bool(#field_name.is_some());
                if let Some(ref #field_name) = #field_name {
                    buf.put_string(#field_name);
                }
            }
        }
        FieldType::String => quote! { buf.put_string(#field_name) },
        FieldType::Array(_) => panic!("struct can't have array field"),
        x => {
            let ident = ident(format!("put_{}", tokenize_field_type(x)));
            quote! { buf.#ident(#field_name) }
        }
    }
}

fn actual_field_type(field: &StructField) -> FieldType {
    field
        .ty_from
        .as_ref()
        .map(FieldFrom::actual_type)
        .unwrap_or(field.ty.clone())
}

fn tokenize_field_type(ty: &FieldType) -> TokenStream {
    match ty {
        FieldType::Byte => quote! { i8 },
        FieldType::Short => quote! { i16 },
        FieldType::Int => quote! { i32 },
        FieldType::Long => quote! { i64 },
        FieldType::Ubyte => quote! { u8 },
        FieldType::Ushort => quote! { u16 },
        FieldType::Uint => quote! { u32 },
        FieldType::Ulong => quote! { u64 },
        FieldType::Block => quote! { P::Block },
        FieldType::Item => quote! { P::Item },
        FieldType::Identifier => quote! { String },
        FieldType::Chat => quote! { String },
        FieldType::OptChat => quote! { Option<String> },
        FieldType::Boolean => quote! { bool },
        FieldType::Position => quote! { BlockPosition },
        FieldType::Nbt => quote! { nbt::Blob },
        FieldType::Varint => quote! { i32 },
        FieldType::Uuid => quote! { uuid::Uuid },
        FieldType::Float => quote! { f32 },
        FieldType::Angle => quote! { u8 },
        FieldType::Double => quote! { f64 },
        FieldType::String => quote! { String },
        FieldType::Array(of) => {
            let inner = tokenize_field_type(of);
            quote! { Vec<#inner> }
        }
        FieldType::StructOrEnum { name } => {
            let ident = ident(name);
            quote! { #ident }
        }
    }
}

pub fn ident(s: impl AsRef<str>) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}
