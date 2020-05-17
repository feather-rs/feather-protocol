//! Defines the data model for the input protocol
//! spec files.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Defines a "root" protocol: a protocol
/// from which all other versions inherit.
#[derive(Debug, Serialize, Deserialize)]
pub struct Root<'a> {
    /// Mapping from struct name => struct field map
    #[serde(borrow)]
    pub structs: IndexMap<&'a str, IndexMap<&'a str, FieldType>>,
    /// Mapping from packet stage (e.g. "login") => set of packets for that stage
    #[serde(borrow)]
    pub packets: IndexMap<&'a str, Packets<'a>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Packets<'a>(
    /// Mapping from packet direction ("clientbound", "serverbound") => packets
    #[serde(borrow)]
    pub IndexMap<&'a str, IndexMap<&'a str, Packet>>,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
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

    Array(Box<FieldType>, ArrayLength),
    Option(Box<FieldType>, OptionTag),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ArrayLength {
    Prefixed(Box<FieldType>),
    Inferred,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OptionTag {
    Prefixed(Box<FieldType>),
}
