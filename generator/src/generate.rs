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
        #![allow(warnings)]
        use bytes::{Bytes, BytesMut, Buf, BufMut};
        use crate::{BytesExt, BytesMutExt, Provider, Error, ProtocolVersion};
        const VERSION: ProtocolVersion = ProtocolVersion::V1_15_2;

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
            _phantom: std::marker::PhantomData<P>,
        }

        impl <P> #name<P> where P: Provider {
            pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
                let version = VERSION;
                #(#read_from)*

                Ok(Self {
                    #(#field_initializers)*
                    _phantom: Default::default(),
                })
            }

            pub fn write_to(self, buf: &mut BytesMut) {
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
    let name_str = &e.name;

    let mut defs = vec![];
    let mut read_froms = vec![];
    let mut write_tos = vec![];
    let mut reprs = vec![];

    for variant in &e.variants {
        let (def, read_from, write_to, repr) = generate_enum_variant(e, variant);
        defs.push(def);
        read_froms.push(read_from);
        write_tos.push(write_to);
        reprs.push(repr);
    }

    let res = quote! {
        pub enum #name<P: Provider> {
            #(#defs ,)*
            _Phantom(std::marker::PhantomData<P>),
        }

        impl <P> #name<P> where P: Provider {
            pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
                match repr {
                    #(#read_froms ,)*
                    repr => Err(Error::InvalidEnumRepr(repr, #name_str).into()),
                }
            }

            pub fn write_to(self, buf: &mut BytesMut) {
                match self {
                    #(#write_tos ,)*
                    #name::_Phantom(_) => panic!("phantom in {} not allowed", #name_str),
                }
            }

            pub fn repr(&self) -> i64 {
                match self {
                    #(#reprs ,)*
                    #name::_Phantom(_) => panic!("phantom in {} not allowed", #name_str),
                }
            }
        }
    };
    res
}

fn generate_enum_variant(
    e: &Enum,
    variant: &EnumVariant,
) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    (
        generate_enum_variant_def(e, variant),
        generate_enum_variant_read_from(e, variant),
        generate_enum_variant_write_to(e, variant),
        generate_enum_variant_repr(e, variant),
    )
}

fn generate_enum_variant_def(_e: &Enum, variant: &EnumVariant) -> TokenStream {
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

    let repr = variant.repr;

    quote! {
        #repr => {
            #(#read_from ;)*
            Ok(#enum_name::#name {
                #(#finish, )*
            })
        }
    }
}

fn generate_enum_variant_write_to(e: &Enum, variant: &EnumVariant) -> TokenStream {
    let name = ident(&variant.name);
    let enum_name = ident(&e.name);

    let mut write_to = vec![];
    let mut fields = vec![];

    for field in &variant.fields {
        let name = ident(&field.name);
        fields.push(quote! { #name });
        write_to.push(write_to_statement(field));
    }

    quote! {
        #enum_name::#name { #(#fields, )* } => {
            #(#write_to ;)*
        }
    }
}

fn generate_enum_variant_repr(e: &Enum, variant: &EnumVariant) -> TokenStream {
    let enum_name = ident(&e.name);
    let name = ident(&variant.name);
    let repr_lit = variant.repr;
    quote! {
       #enum_name::#name { .. } => #repr_lit
    }
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
        FieldType::Identifier | FieldType::Chat | FieldType::String => {
            quote! { buf.try_get_string()? }
        }
        x => {
            let ident = ident(format!("try_get_{}", tokenize_field_type(x)));
            quote! { buf.#ident()? }
        }
    }
}

fn write_to_statement(field: &StructField) -> TokenStream {
    let field_name = &field.name;

    let f = write_fn_ident(field_name, &field.ty);

    let field_name = ident(field_name);

    let convert = if let Some(ref ty_from) = field.ty_from {
        let tokens = ty_from.tokens_for_write(quote! { #field_name });

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
        FieldType::String | FieldType::Identifier | FieldType::Chat => {
            quote! { buf.put_string(#field_name) }
        }
        FieldType::Array(_) => panic!("struct can't have array field"),
        x => {
            let ident = ident(format!("put_{}", tokenize_field_type(x)));
            let x = tokenize_field_type(x);
            quote! { buf.#ident(#field_name as #x) }
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
            quote! { #ident::<P> }
        }
    }
}

pub fn ident(s: impl AsRef<str>) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}
