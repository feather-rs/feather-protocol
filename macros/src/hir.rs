//! A representation of the protocol which corresponds
//! roughly with the input syntax, but on a higher level
//! (all unnecessary information is weeded out.)

use syn::{Ident, Type};

pub type PacketId = u32;

/// A version of the MC protocol.
#[derive(Debug, Clone)]
pub struct Version {
    /// The version's display name as written in the source, e.g. "1.16.1."
    pub display_name: String,
    /// The identifier for the `ProtocolVersion` enum, e.g. `V1_16_1`.
    pub ident: Ident,
}

/// Unique ID of a version.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VersionId(usize);

/// Unique ID of a struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructId(usize);

/// ID of a struct's field. Unique only within that struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldId(usize);

/// A high-level representation of the protocol.
///
/// A protocol includes all packets present
/// in some range of Minecraft versions.
#[derive(Debug, Clone)]
pub struct Protocol {
    /// Structs defined by this protocol.
    pub structs: Vec<Struct>,
    /// Enums defined by this protocol.
    pub enums: Vec<Enum>,
    /// Packets defined by this protocol.
    pub packets: Vec<Packet>,
    /// Set of Minecraft versions covered by this protocol.
    pub versions: Vec<Version>,
}

#[derive(Debug, Clone)]
pub struct Struct {
    /// Name of this struct.
    pub name: Ident,
    /// Fields of this struct.
    pub fields: Vec<Field>,
}

/// An enum, or tagged union.
#[derive(Debug, Clone)]
pub struct Enum {
    /// Name of this enum.
    pub name: Ident,
    /// This enum's variants.
    pub variants: Variants,
}

/// The variants of an enum.
#[derive(Debug, Clone)]
pub enum Variants {
    /// Variants are tagged using integers to determine
    /// which variant is present.
    Integer {
        /// The integer type—i8, varint, i32, etc.
        integer_type: IntegerType,
        /// The set of variants in this enum.
        variants: Vec<Variant<i64>>,
    },
    /// Variants are tagged using strings to determine
    /// which variant is present.
    String {
        /// The set of variants in this enum.
        variants: Vec<Variant<String>>,
    },
}

/// A variant of an enum.
#[derive(Clone, Debug)]
pub struct Variant<T> {
    /// This variant's name.
    pub name: Ident,
    /// This variant's tag value, which is used
    /// to determine if this variant is present.
    ///
    /// Note that tag values represent the tag value
    /// in the _oldest_ supported version. `VariantAttribute`s
    /// can override the tag value for a specific set of versions.  
    pub tag_value: T,
    /// This variant's fields. If this is empty, it's
    /// a unit variant.
    pub fields: Vec<Field>,
    /// Attributes applied to this variant.
    pub attributes: Vec<KeyedAttribute<T>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntegerType {
    I8,
    I16,
    I32,
    I64,
    VarInt,
}

#[derive(Debug, Clone)]
pub struct Field {
    /// This field's name.
    pub name: Ident,
    /// This field's base type.
    ///
    /// A field's type represents its type in the _earliest_
    /// supported version. If the type changed in a later MC
    /// version, then there should be a `TypeChanged` attribute
    /// in the `attributes` vector.
    pub typ: FieldType,
    /// The attributes applied to this field, in order they're written.
    pub attributes: Vec<KeyedAttribute<FieldType>>,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    /// A Rust type, such as `i32`, `usize`, `Uuid`, etc.
    ///
    /// Excludes other special-cased types, such as `Vec`.
    Rust(Type),

    /// An array, equivalent to a `Vec<T>` in the source.
    Array {
        /// The type of the array.
        typ: Box<FieldType>,
        /// How the array's length is encoded.
        length: ArrayLength,
    },

    /// A variable-length integer.
    VarInt,
}

#[derive(Debug, Clone)]
pub enum ArrayLength {
    /// Array length is prefixed with a varint.
    VarintPrefixed,
    /// Another field in this struct is interpreted as the length.
    Field(FieldId),
}

/// A packet defined within a protocol.
#[derive(Debug, Clone)]
pub struct Packet {
    /// The struct which contains this packet's fields.
    pub strukt: StructId,
    /// The direction the packet is sent in.
    pub direction: PacketDirection,
    /// The protocol state when this packet is allowed.
    pub stage: ProtocolStage,
    /// This packet's ID in the earliest supported MC version.
    pub id: PacketId,
    /// Attributes applied to this packet.
    pub attributes: Vec<KeyedAttribute<PacketId>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PacketDirection {
    Serverbound,
    Clientbound,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProtocolStage {
    Handshake,
    Status,
    Login,
    Play,
}

#[derive(Debug, Clone)]
pub enum KeyedAttribute<T> {
    /// This item was added in some version.
    ///
    /// It is not present in prior versions.
    AddedIn(VersionId),

    /// This item was removed in some version.
    ///
    /// It is not present in future versions.
    RemovedIn(VersionId),

    /// This item's associated data changed in some version.
    ///
    /// For field attributes, type T is the field type. For enum
    /// variant attributes, type T is the variant tag value. For packet
    /// attributes, type T is the packet's ID.
    ///
    /// In versions equal to and after this version,
    /// its type is the new type.
    DataChanged { in_version: VersionId, new_data: T },
}
