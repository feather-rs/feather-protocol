//! Defines the data model for the input protocol
//! spec files.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Defines a "root" protocol: a protocol
/// from which all other versions inherit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Root {
    /// Defines the protocol name + version.
    pub protocol: Protocol,
    /// Mapping from struct name => struct field map
    pub types: IndexMap<String, Type>,
    /// Mapping from packet stage (e.g. "login") => set of packets for that stage
    pub packets: IndexMap<String, Packets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protocol {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Struct {
    pub fields: IndexMap<String, FieldType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub tag: Box<FieldType>,
    pub variants: IndexMap<EnumTag, EnumVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum EnumTag {
    Integer(i32),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    #[serde(default)]
    pub fields: IndexMap<String, FieldType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Packets(
    /// Mapping from packet direction ("clientbound", "serverbound") => packets
    pub IndexMap<String, IndexMap<String, Packet>>,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    VarInt,

    F32,
    F64,

    Bool,

    String,

    Uuid,
    Nbt,

    Array(Box<FieldType>, ArrayLength),
    Option(Box<FieldType>, OptionTag),

    Custom(Box<CustomType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomType {
    Struct {
        name: String,
        // Normally we would use #[serde(flatten)] inner: Struct,
        // but https://github.com/serde-rs/serde/issues/1346 prevents this.
        fields: IndexMap<String, FieldType>,
    },
    Enum {
        name: String,
        tag: Box<FieldType>,
        variants: IndexMap<EnumTag, EnumVariant>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayLength {
    Prefixed(Box<FieldType>),
    Inferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionTag {
    Prefixed(Box<FieldType>),
}
