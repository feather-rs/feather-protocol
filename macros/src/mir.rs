//! A mid-level intermediate representation of a protocol.
//!
//! This representation is designed for direct use by the code generator.
//! It differs from the HIR in that it:
//! * Has a new concept of "models," which are effectively
//! lists of fields. We have this unified concept because both
//! structs and enums have this sort of field-list structure.
//! * "Monomorphizes" different versions of the same model
//! into different structs, each with fields that match that
//! version of the packet.
//! * Has no concept of "attributes"; these are too high level for consumption
//! by the code generator. Instead, all attribute data is applied in the monomorphization
//! process.

use crate::hir;
use std::collections::BTreeMap;
use syn::{Ident, Lit, Type};

/// Unique ID of a model.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModelId(usize);

/// Unique ID of a struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructId(usize);

/// Unique ID of an enum.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EnumId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VersionId(usize);

/// A protocol in the MIR format.
#[derive(Debug, Clone)]
pub struct Protocol {
    /// The set of models needed to read/write packets in this protocol.
    pub models: Vec<Model>,
    /// The set of structs in this protocol.
    pub structs: Vec<Struct>,
    /// The set of enums in this protocol.
    pub enums: Vec<Enum>,
    /// The set of packets in this protocol.
    pub packets: Vec<Packet>,
    /// The set of versions covered by this protocol.
    pub versions: Vec<hir::Version>,
}

impl Protocol {
    /// Returns the model with the given ID.
    pub fn model(&self, id: ModelId) -> &Model {
        &self.models[id.0]
    }

    /// Returns the version with the given ID.
    pub fn version(&self, id: VersionId) -> &hir::Version {
        &self.versions[id.0]
    }

    /// Returns an iterator over (model_id, model)
    pub fn models<'a>(&'a self) -> impl Iterator<Item = (ModelId, &'a Model)> + 'a {
        self.models
            .iter()
            .enumerate()
            .map(|(id, model)| (ModelId(id), model))
    }

    /// Returns an iterator over (struct_id, strukt)
    pub fn structs<'a>(&'a self) -> impl Iterator<Item = (StructId, &'a Struct)> + 'a {
        self.structs
            .iter()
            .enumerate()
            .map(|(id, strukt)| (StructId(id), strukt))
    }

    /// Returns an iterator over (enum_id, en)
    pub fn enums<'a>(&'a self) -> impl Iterator<Item = (EnumId, &'a Enum)> + 'a {
        self.enums
            .iter()
            .enumerate()
            .map(|(id, en)| (EnumId(id), en))
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    /// Fields in this model.
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Struct {
    /// Name of this struct.
    pub name: Ident,
    /// Mapping from MC version => the model of this struct for that version.
    pub versions: BTreeMap<VersionId, ModelId>,
    /// The union of all fields of this struct.
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct Enum {
    /// Name of this enum.
    pub name: Ident,
    /// Mapping from MC version => variants of this enum for that version.
    pub versions: BTreeMap<VersionId, Vec<VersionedEnumVariant>>,
    /// The union of all variants of this enum.
    pub all_variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub struct VersionedEnumVariant {
    /// Name of this variant.
    pub name: Ident,
    // The model containing this variant's fields.
    pub model: ModelId,
    /// Tag value of this variant.
    pub tag_value: Lit,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    /// Name of this variant.
    pub name: Ident,
    /// Fields of this variant.
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct Field {
    /// Name of this field.
    pub name: Ident,
    /// Type of this field.
    pub typ: FieldType,
    /// A special way to derive the field when writing.
    pub source: Option<Source>,
    /// An optional type conversion to perform.
    pub conversion: Option<Conversion>,
}

#[derive(Debug, Clone)]
pub enum Conversion {
    /// Use a primitive cast to convert from one
    /// type to another (e.g. `x as i32`)
    Primitive { from: Type, to: Type },
    /// Convert an `i32` to a `VarInt`.
    VarInt,
}

#[derive(Debug, Clone)]
pub enum Source {
    /// Source a field's value from the length of an array.
    ArrayLength {
        /// Name of the array field to source from.
        array: Ident,
    },
}

#[derive(Debug, Clone)]
pub struct StructField {
    /// Name of this field.
    pub name: Ident,
    /// Type of this field.
    pub typ: FieldType,
    /// Whether this field is present across all MC versions.
    pub always_present: bool,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Rust(Type),

    Array {
        /// Name of the field which contains the array's length.
        length: Ident,
        /// The inner type of the array.
        typ: Box<FieldType>,
    },
}

#[derive(Debug, Clone)]
pub struct Packet {
    /// The struct containing this packet's fields.
    pub strukt: StructId,
    /// Direction this packet gets sent in.
    pub direction: hir::PacketDirection,
    /// Stage of the connection where this packet is valid.
    pub stage: hir::ProtocolStage,
    /// Mapping from version => ID of this packet for that version.
    pub ids: BTreeMap<VersionId, hir::PacketId>,
}
