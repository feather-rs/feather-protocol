use crate::generate::ident;
use crate::parse::{Construct, Keyword, Literal, SyntaxTree, Token};
use proc_macro2::{Ident, TokenStream};
use std::iter::Peekable;
use strum_macros::*;
use thiserror::Error;

#[derive(Debug, Clone, Default)]
pub struct PacketDefinitions {
    pub clientbound: Vec<Packet>,
    pub serverbound: Vec<Packet>,
    pub structs_and_enums: Vec<StructOrEnum>,
    pub version: String,
}

#[derive(Debug, Clone)]
pub enum StructOrEnum {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub repr: EnumRepr,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumRepr {
    Integer,
    String,
}

impl EnumRepr {
    pub fn matches(self, lit: &Literal) -> bool {
        EnumRepr::from_lit(lit) == self
    }

    pub fn from_lit(lit: &Literal) -> Self {
        match lit {
            Literal::String(_) => EnumRepr::String,
            Literal::Integer(_) => EnumRepr::Integer,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<StructField>,
    pub repr: Literal,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: FieldType,
    pub ty_from: Option<FieldFrom>,
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub name: String,
    pub id: u32,
    pub fields: Vec<PacketField>,
}

#[derive(Debug, Clone)]
pub struct PacketField {
    pub name: String,
    pub ty: FieldType,
    pub ty_from: Option<FieldFrom>,
    pub value_from: Option<ValueFrom>,
}

#[derive(Debug, Clone)]
pub enum FieldFrom {
    BlockId,
    BlockType,
    ItemId,
    Enum { enum_name: String },
}

impl FieldFrom {
    pub fn from_construct(
        construct: &Construct,
        packet_name: &str,
        constructs: &mut Constructs,
        defs: &mut PacketDefinitions,
    ) -> Result<Self, Error> {
        if let Some(keyword) = construct.as_keyword() {
            // Enum.
            let ty = make_anonymous_struct_or_enum(constructs, packet_name, defs, keyword)?;

            let name = match ty {
                FieldType::StructOrEnum { name } => name,
                _ => unreachable!(),
            };

            Ok(FieldFrom::Enum { enum_name: name })
        } else if let Some(identifier) = construct.as_identifier() {
            match identifier.as_str() {
                "block.id" => Ok(FieldFrom::BlockId),
                "block.type" => Ok(FieldFrom::BlockType),
                "item.id" => Ok(FieldFrom::ItemId),
                x => Ok(FieldFrom::Enum {
                    enum_name: String::from(x),
                }),
            }
        } else {
            Err(Error::InvalidFieldType)
        }
    }

    pub fn actual_type(&self) -> FieldType {
        match self {
            FieldFrom::BlockId | FieldFrom::BlockType => FieldType::Block,
            FieldFrom::ItemId => FieldType::Item,
            FieldFrom::Enum { enum_name } => FieldType::StructOrEnum {
                name: enum_name.clone(),
            },
        }
    }

    pub fn tokens_for_read(&self, input_var: Ident) -> TokenStream {
        use quote::quote;

        match self {
            FieldFrom::BlockId => quote! { P::block_from_id(#input_var as u16, version)? },
            FieldFrom::BlockType => quote! { P::block_from_ty(#input_var as u16, version)? },
            FieldFrom::ItemId => quote! { P::item_from_id(#input_var as u16, version)? },
            FieldFrom::Enum { enum_name } => {
                let enum_name = ident(enum_name);
                quote! {
                    #enum_name::from_repr(#input_var as u16)?
                }
            }
        }
    }

    pub fn tokens_for_write(&self, input_var: TokenStream) -> TokenStream {
        use quote::quote;

        match self {
            FieldFrom::BlockId => quote! { P::block_id(#input_var, version) },
            FieldFrom::BlockType => quote! { P::block_ty(#input_var, version) },
            FieldFrom::ItemId => quote! { P::item_id(#input_var, version) },
            FieldFrom::Enum { enum_name: _ } => quote! { #input_var.repr().into() },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueFrom {
    ArrayLength { field: String },
}

impl ValueFrom {
    pub fn from_str(x: &str) -> Option<Self> {
        let split: Vec<&str> = x.split(".").collect();

        if split.len() != 2 {
            None
        } else {
            match split[1] {
                "length" => Some(ValueFrom::ArrayLength {
                    field: String::from(split[0]),
                }),
                _ => None,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    Byte,
    Short,
    Int,
    Long,
    Ubyte,
    Ushort,
    Uint,
    Ulong,
    Block,
    Item,
    Identifier,
    Chat,
    OptChat,
    Boolean,
    Position,
    Nbt,
    Varint,
    Uuid,
    Float,
    Angle,
    Double,
    String,
    Array(Box<FieldType>),
    StructOrEnum { name: String },
}

impl FieldType {
    pub fn from_construct(construct: &Construct) -> Option<Self> {
        match construct {
            Construct::Keyword(keyword) => match keyword {
                Keyword::Byte => Some(FieldType::Byte),
                Keyword::Short => Some(FieldType::Short),
                Keyword::Int => Some(FieldType::Int),
                Keyword::Long => Some(FieldType::Long),
                Keyword::Ubyte => Some(FieldType::Ubyte),
                Keyword::Ushort => Some(FieldType::Ushort),
                Keyword::Uint => Some(FieldType::Uint),
                Keyword::Ulong => Some(FieldType::Ulong),
                Keyword::Block => Some(FieldType::Block),
                Keyword::Item => Some(FieldType::Item),
                Keyword::Identifier => Some(FieldType::Identifier),
                Keyword::Chat => Some(FieldType::Chat),
                Keyword::OptChat => Some(FieldType::OptChat),
                Keyword::Boolean => Some(FieldType::Boolean),
                Keyword::Position => Some(FieldType::Position),
                Keyword::Nbt => Some(FieldType::Nbt),
                Keyword::Varint => Some(FieldType::Varint),
                Keyword::Uuid => Some(FieldType::Uuid),
                Keyword::Float => Some(FieldType::Float),
                Keyword::Angle => Some(FieldType::Angle),
                Keyword::Double => Some(FieldType::Double),
                Keyword::String => Some(FieldType::String),
                _ => None,
            },
            Construct::Identifier(s) => Some(FieldType::StructOrEnum { name: s.clone() }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr)]
enum PacketSet {
    Clientbound,
    Serverbound,
}

impl PacketSet {
    pub fn keyword(self) -> Keyword {
        match self {
            PacketSet::Clientbound => Keyword::Clientbound,
            PacketSet::Serverbound => Keyword::Serverbound,
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected end of input")]
    UnexpectedEof,
    #[error("no serverbound/clientbound block prefix")]
    NoBlockPrefix,
    #[error("did not expect construct {0:?}")]
    UnexpectedConstruct(Construct),
    #[error("expected struct/enum name; found something else")]
    ExpectedStructEnumName,
    #[error("expected block; found something else")]
    ExpectedBlock,
    #[error("did not expect keyword {0:?}")]
    UnexpectedKeyword(Keyword),
    #[error("invalid field type")]
    InvalidFieldType,
    #[error("expected token")]
    ExpectedToken,
    #[error("missing semicolon")]
    MissingSemicolon,
    #[error("enum variants have non-uniform repr")]
    EnumReprMismatch,
    #[error("enum has no variants")]
    EnumHasNoVariants,
    #[error("expected identifier; found something else")]
    ExpectedIdentifier,
    #[error("expected literal; found something else")]
    ExpectedLiteral,
    #[error("did not expect token {0:?}")]
    UnexpectedToken(Token),
    #[error("invalid value-from clause")]
    InvalidValueFrom,
}

type Constructs<'a> = Peekable<std::slice::Iter<'a, Construct>>;

pub fn compile_tree(tree: &SyntaxTree) -> anyhow::Result<PacketDefinitions> {
    let mut defs = PacketDefinitions {
        version: String::from("V1_15_2"), // TODO
        ..Default::default()
    };

    let mut constructs = tree.constructs.iter().peekable();

    compile_packet_set(&mut constructs, PacketSet::Clientbound, &mut defs)?;
    compile_packet_set(&mut constructs, PacketSet::Serverbound, &mut defs)?;

    Ok(defs)
}

fn compile_packet_set(
    constructs: &mut Constructs,
    set: PacketSet,
    defs: &mut PacketDefinitions,
) -> Result<(), Error> {
    // Check block prefix.
    let prefix = constructs
        .next()
        .ok_or(Error::UnexpectedEof)?
        .as_keyword()
        .ok_or(Error::NoBlockPrefix)?;
    if prefix != &set.keyword() {
        return Err(Error::NoBlockPrefix);
    }

    // Compile each definition in block. A definition could be either a packet, struct, or enum
    let constructs = constructs
        .next()
        .ok_or(Error::UnexpectedEof)?
        .as_block()
        .ok_or(Error::ExpectedBlock)?;
    let mut constructs = constructs.constructs.iter();

    let mut next_packet_id = 0u32;

    loop {
        let construct = match constructs.next() {
            Some(c) => c,
            None => break,
        };

        dbg!(&construct);

        match construct {
            Construct::Keyword(keyword) => {
                // Either struct or enum.
                let name = constructs
                    .next()
                    .ok_or(Error::UnexpectedEof)?
                    .as_identifier()
                    .ok_or(Error::ExpectedStructEnumName)?;
                let definition = constructs
                    .next()
                    .ok_or(Error::UnexpectedEof)?
                    .as_block()
                    .ok_or(Error::ExpectedBlock)?;

                match keyword {
                    Keyword::Struct => compile_struct(&name, definition, defs)?,
                    Keyword::Enum => compile_enum(&name, definition, defs)?,
                    x => return Err(Error::UnexpectedKeyword(*x)),
                }
            }
            Construct::Identifier(name) => {
                // Packet definition.
                let definition = constructs
                    .next()
                    .ok_or(Error::UnexpectedEof)?
                    .as_block()
                    .ok_or(Error::ExpectedBlock)?;

                compile_packet(name, definition, defs, set, next_packet_id)?;
                next_packet_id += 1;
            }
            Construct::Token(Token::Annotation) => {
                // TODO.
                let _paren = constructs.next().unwrap();
                let _paren = constructs.next().unwrap();
            }
            x => return Err(Error::UnexpectedConstruct(x.clone())),
        }
    }

    Ok(())
}

fn compile_struct(
    name: &str,
    definition: &SyntaxTree,
    defs: &mut PacketDefinitions,
) -> Result<(), Error> {
    let mut s = Struct {
        name: String::from(name),
        fields: vec![],
    };

    // Parse fields.
    let mut constructs = definition.constructs.iter().peekable();

    loop {
        match compile_struct_field(&mut constructs, name, defs) {
            Ok(field) => s.fields.push(field),
            Err(Error::UnexpectedEof) => break,
            Err(e) => return Err(e),
        }
    }

    defs.structs_and_enums.push(StructOrEnum::Struct(s));

    Ok(())
}

fn compile_enum(
    name: &str,
    definition: &SyntaxTree,
    defs: &mut PacketDefinitions,
) -> Result<(), Error> {
    let mut variants = vec![];
    let mut repr: Option<EnumRepr> = None;
    let mut constructs = definition.constructs.iter().peekable();

    // Parse variants.
    loop {
        match compile_enum_variant(&mut constructs, name, defs) {
            Ok(variant) => {
                if let Some(ref repr) = repr {
                    if !repr.matches(&variant.repr) {
                        return Err(Error::EnumReprMismatch);
                    }
                } else {
                    repr = Some(EnumRepr::from_lit(&variant.repr));
                }

                variants.push(variant);
            }
            Err(Error::UnexpectedEof) => break,
            Err(e) => return Err(e),
        }
    }

    let e = Enum {
        name: String::from(name),
        variants,
        repr: repr.ok_or(Error::EnumHasNoVariants)?,
    };
    defs.structs_and_enums.push(StructOrEnum::Enum(e));

    Ok(())
}

fn compile_packet(
    name: &str,
    definition: &SyntaxTree,
    defs: &mut PacketDefinitions,
    set: PacketSet,
    id: u32,
) -> Result<(), Error> {
    // Compile fields.
    let mut fields = vec![];

    let mut constructs = definition.constructs.iter().peekable();
    loop {
        match compile_packet_field(&mut constructs, name, defs) {
            Ok(field) => fields.push(field),
            Err(Error::UnexpectedEof) => break,
            Err(e) => return Err(e),
        }
    }

    let packet = Packet {
        name: String::from(name),
        id,
        fields,
    };
    match set {
        PacketSet::Clientbound => &mut defs.clientbound,
        PacketSet::Serverbound => &mut defs.serverbound,
    }
    .push(packet);

    Ok(())
}

fn compile_packet_field(
    constructs: &mut Constructs,
    packet_name: &str,
    defs: &mut PacketDefinitions,
) -> Result<PacketField, Error> {
    let ty = constructs.next().ok_or(Error::UnexpectedEof)?;

    // Possible types: normal keyword, struct/enum, anonymous enum, anonymous struct.

    let mut ty = packet_field_type_from_construct(ty, constructs, packet_name, defs)?;

    // There may be a "field from," which indicates the type from which the field's actual
    // value is derived. e.g. varint field but from a block's state ID.
    let (ty_from, name) = parse_ty_from_and_name(constructs, packet_name, defs)?;

    // There may be a "value from," which indicates the value this field is bound to. e.g.
    // length of a separate array field.
    let (value_from, _semicolon) = {
        let mut value_from = None;
        loop {
            let next = constructs.next().ok_or(Error::UnexpectedEof)?;

            if let Some(paren) = next.as_parenthesized() {
                let identifier = paren
                    .constructs
                    .first()
                    .ok_or(Error::UnexpectedEof)?
                    .as_identifier()
                    .ok_or(Error::ExpectedIdentifier)?;
                value_from = Some(ValueFrom::from_str(&identifier).ok_or(Error::InvalidValueFrom)?);
            } else if let Some(tok) = next.as_token() {
                if tok == &Token::Array {
                    ty = FieldType::Array(Box::new(ty));
                    continue;
                }

                if tok != &Token::Semicolon {
                    return Err(Error::UnexpectedToken(*tok));
                }

                break (value_from, *tok);
            }
        }
    };

    let field = PacketField {
        name,
        ty,
        value_from,
        ty_from,
    };

    Ok(field)
}

fn parse_ty_from_and_name(
    constructs: &mut Constructs,
    packet_name: &str,
    defs: &mut PacketDefinitions,
) -> Result<(Option<FieldFrom>, String), Error> {
    let mut ty_from = None;
    loop {
        let next = constructs.next().ok_or(Error::UnexpectedEof)?;

        if let Some(paren) = next.as_parenthesized() {
            // there is a "field from"
            let mut constructs = paren.constructs.iter().peekable();
            let ty = constructs.next().ok_or(Error::UnexpectedEof)?;

            ty_from = Some(FieldFrom::from_construct(
                ty,
                packet_name,
                &mut constructs,
                defs,
            )?);
        } else if let Some(name) = next.as_identifier() {
            break Ok((ty_from, name));
        } else {
            break Err(Error::UnexpectedConstruct(next.clone()));
        }
    }
}

fn packet_field_type_from_construct(
    ty: &Construct,
    constructs: &mut Constructs,
    packet_name: &str,
    defs: &mut PacketDefinitions,
) -> Result<FieldType, Error> {
    if let Some(keyword) = ty.as_keyword() {
        match keyword {
            Keyword::Enum | Keyword::Struct => Ok(make_anonymous_struct_or_enum(
                constructs,
                packet_name,
                defs,
                keyword,
            )?),
            k => Ok(FieldType::from_construct(&Construct::Keyword(k.clone()))
                .ok_or(Error::InvalidFieldType)?),
        }
    } else if let Some(identifier) = ty.as_identifier() {
        Ok(FieldType::StructOrEnum {
            name: String::from(identifier),
        })
    } else {
        Err(Error::InvalidFieldType)
    }
}

fn make_anonymous_struct_or_enum(
    constructs: &mut Constructs,
    packet_name: &str,
    defs: &mut PacketDefinitions,
    keyword: &Keyword,
) -> Result<FieldType, Error> {
    // Compile anonymous enum/struct.
    let next = constructs.next().ok_or(Error::UnexpectedEof)?;
    let name = next.as_identifier().ok_or(Error::ExpectedIdentifier)?;
    let name = anonymous_type_name(packet_name, &name);

    let definition = constructs
        .next()
        .ok_or(Error::UnexpectedEof)?
        .as_block()
        .ok_or(Error::ExpectedBlock)?;

    if keyword == &Keyword::Enum {
        compile_enum(&name, definition, defs)?;
    } else {
        compile_struct(&name, definition, defs)?;
    }
    Ok(FieldType::StructOrEnum { name })
}

fn anonymous_type_name(packet_name: &str, name: &str) -> String {
    let mut packet_name = String::from(packet_name);
    packet_name.push_str(name);
    packet_name
}

fn compile_enum_variant(
    constructs: &mut Constructs,
    enum_name: &str,
    defs: &mut PacketDefinitions,
) -> Result<EnumVariant, Error> {
    let next = constructs.next().ok_or(Error::UnexpectedEof)?;
    let name = next.as_identifier().ok_or(Error::ExpectedIdentifier)?;

    // Optional field declarations
    let (field_block, _equals, repr) = {
        let mut field_block = None;
        let mut equals = None;
        let repr;

        loop {
            let next = constructs.next().ok_or(Error::UnexpectedEof)?;

            if let Some(block) = next.as_block() {
                field_block = Some(block);
            } else if let Some(tok) = next.as_token() {
                match tok {
                    Token::Equals => equals = Some(*tok),
                    _ => return Err(Error::UnexpectedToken(*tok)),
                }
            } else if let Some(lit) = next.as_literal() {
                repr = Some(lit.clone());
                break;
            }
        }

        (
            field_block,
            equals.ok_or(Error::ExpectedToken)?,
            repr.ok_or(Error::ExpectedLiteral)?,
        )
    };

    let _comma = constructs
        .next()
        .ok_or(Error::UnexpectedEof)?
        .as_token()
        .ok_or(Error::ExpectedToken)?;

    let mut fields = vec![];
    if let Some(block) = field_block {
        let mut constructs = block.constructs.iter().peekable();
        loop {
            match compile_struct_field(&mut constructs, enum_name, defs) {
                Ok(field) => fields.push(field),
                Err(Error::UnexpectedEof) => break,
                Err(e) => return Err(e),
            }
        }
    }

    Ok(EnumVariant {
        fields,
        repr,
        name: String::from(name),
    })
}

fn compile_struct_field(
    constructs: &mut Constructs,
    struct_name: &str,
    defs: &mut PacketDefinitions,
) -> Result<StructField, Error> {
    let ty = constructs.next().ok_or(Error::UnexpectedEof)?;
    let ty = FieldType::from_construct(ty).ok_or(Error::InvalidFieldType)?;

    let (ty_from, name) = parse_ty_from_and_name(constructs, struct_name, defs)?;

    let semicolon = constructs
        .next()
        .ok_or(Error::UnexpectedEof)?
        .as_token()
        .ok_or(Error::ExpectedToken)?;

    if semicolon != &Token::Semicolon {
        Err(Error::MissingSemicolon)
    } else {
        Ok(StructField {
            name: String::from(name),
            ty_from,
            ty,
        })
    }
}
