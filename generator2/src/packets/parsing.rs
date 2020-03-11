//! Parses a protocol JSON file, outputting a `Protocol`.
use anyhow::{anyhow, bail};
use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::collections::VecDeque;
use thiserror::Error;

pub type PacketIdentifier = String;
pub type PacketName = String;
pub type PacketId = u32;
pub type FieldName = String;
pub type TypeName = String;

/// Defines all packets and field types for a protocol version.
#[derive(Debug, Clone, Default)]
pub struct Protocol {
    /// The set of all packets defined for this protocol.
    ///
    /// Keys in this map are the packet identifiers as listed in the JSON
    /// file, e.g. "packet_set_protocol."
    pub packets: HashMap<PacketIdentifier, Packet>,
    /// Custom types defined by this protocol.
    pub types: HashMap<TypeName, FieldType>,
}

/// A packet defined for a protocol.
#[derive(Debug, Clone)]
pub struct Packet {
    /// This packet's identifier from the JSON file, e.g. "packet_set_protocol."
    pub identifier: PacketIdentifier,
    /// This packet's name, derived from its identifier, e.g. "SetProtocol."
    pub name: PacketName,
    /// ID of this packet.
    pub id: PacketId,
    /// Fields of this packet, in the order they are written/read.
    ///
    /// Keys in this map are the field names; values are the field types.
    pub fields: IndexMap<FieldName, FieldType>,
}

/// A field's type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    Varint,
    U8,
    U16,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Pstring {
        count_type: Box<FieldType>,
    },
    /// Vec<u8>
    Buffer,
    Uuid,
    Option {
        of: Box<FieldType>,
    },
    EntityMetadataLoop {
        end_value: u8,
        of: Box<FieldType>,
    },
    Bitfield {
        /// Mapping from field names of the bitfield to
        /// size of this field in bits.
        fields: IndexMap<FieldName, u32>,
    },
    /// struct
    Container {
        fields: IndexMap<FieldName, FieldType>,
    },
    Switch {
        compare_to: FieldName,
        fields: HashMap<Literal, FieldType>,
    },
    Void,
    Array {
        count_type: Box<FieldType>,
        of: Box<FieldType>,
    },
    RestBuffer,
    OptionalNbt,
    Nbt,
    /// Field type defined in the protocol packet definitions.
    ///
    /// Must be contained within `Protocol::types`
    Custom {
        name: TypeName,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Integer(i32),
    String(String),
    Bool(bool),
}

impl Literal {
    pub fn from_str(s: &str) -> Self {
        if let Ok(b) = s.parse::<bool>() {
            Literal::Bool(b)
        } else if let Ok(x) = s.parse::<i32>() {
            Literal::Integer(x)
        } else {
            Literal::String(s.to_owned())
        }
    }
}

pub fn parse(input: &str) -> anyhow::Result<Protocol> {
    let mut prot = Protocol::default();

    let input: Value = serde_json::from_str(input)?;
    let input = input
        .as_object()
        .ok_or(anyhow!("expected root to be an object"))?;

    let types = input.get("types").ok_or(anyhow!("no types object"))?;
    let types = types
        .as_object()
        .ok_or(anyhow!("expected type definitions to be an object"))?;

    parse_types(types, &mut prot)?;

    Ok(prot)
}

#[derive(Debug, Error)]
#[error("missing type {name}")]
struct MissingTypeError {
    name: TypeName,
}

fn parse_types(types: &Map<String, Value>, prot: &mut Protocol) -> anyhow::Result<()> {
    let mut queue: VecDeque<_> = types.clone().into_iter().collect();

    while let Some((name, value)) = queue.pop_front() {
        match parse_type(&name, &value, prot) {
            Ok(_) => (),
            Err(e) => {
                // if there is a missing type which we might not yet have
                // parsed, and there are more types left to parse,
                // then push this type onto the back of the queue so
                // it will be parsed after its dependencies are parsed.
                if !queue.is_empty() {
                    if let Some(_) = e.downcast_ref::<MissingTypeError>() {
                        queue.push_back((name, value));
                        continue;
                    }
                }
                bail!(e);
            }
        }
    }

    Ok(())
}

fn parse_type(name: &str, value: &Value, prot: &mut Protocol) -> anyhow::Result<()> {
    if let Some("native") = value.as_str() {
        // type is already defined natively by `FieldType`.
        // skip.
        return Ok(());
    }

    let ty = parse_field_type(
        name,
        value.as_array().ok_or(anyhow!(
            "expected type definition for '{}' to be an array object",
            name
        ))?,
        prot,
    )?;
    prot.types.insert(name.to_owned(), ty);

    Ok(())
}

fn parse_field_type(
    for_field: &str,
    value: &[Value],
    prot: &Protocol,
) -> anyhow::Result<FieldType> {
    // no idea who came up with this format, but here it is:
    // value[0]: string which specifies the parent type, e.g. "pstring"
    // value[1]: optional value which depends on the field type

    let parent = value
        .get(0)
        .ok_or(anyhow!("field type for '{}' is empty", for_field))?
        .as_str()
        .ok_or(anyhow!(
            "expected field type for '{}' to start with a string",
            for_field
        ))?;

    let ty = match parent {
        "varint" => FieldType::Varint,
        "pstring" => parse_pstring_type(&value[1..], prot)?,
        "u16" => FieldType::U16,
        "u8" => FieldType::U8,
        "i64" => FieldType::I64,
        "buffer" => FieldType::Buffer,
        "i32" => FieldType::I32,
        "i8" => FieldType::I8,
        "bool" => FieldType::Bool,
        "i16" => FieldType::I16,
        "f32" => FieldType::F32,
        "f64" => FieldType::F64,
        "UUID" => FieldType::Uuid,
        "option" => parse_option_type(&value[1..], prot)?,
        "entityMetadataLoop" => parse_entity_metadata_loop_type(&value[1..], prot)?,
        "bitfield" => parse_bitfield_type(&value[1..])?,
        "container" => parse_container_type(&value[1..], prot)?,
        "switch" => parse_switch_type(&value[1..], prot)?,
        "void" => FieldType::Void,
        "array" => parse_array_type(&value[1..], prot)?,
        "restBuffer" => FieldType::RestBuffer,
        "nbt" => FieldType::Nbt,
        "optionalNbt" => FieldType::OptionalNbt,
        name => {
            // try to find the type defined within the protocol;
            // otherwise, return an error, since the type is not defined
            match prot.types.get(name) {
                Some(ty) => ty.clone(),
                None => bail!(MissingTypeError {
                    name: name.to_owned()
                }),
            }
        }
    };

    Ok(ty)
}

fn parse_pstring_type(value: &[Value], prot: &Protocol) -> anyhow::Result<FieldType> {
    let value = value
        .get(0)
        .ok_or(anyhow!("missing pstring metadata"))?
        .as_object()
        .ok_or(anyhow!("expected pstring type metadata to be an object"))?;

    let count_type = value
        .get("countType")
        .ok_or(anyhow!("pstring missing countType metadata"))?
        .as_str()
        .ok_or(anyhow!("pstring countType must be a string"))?;

    let ty = parse_field_type("pstring", &[Value::String(count_type.to_owned())], prot)?;

    Ok(FieldType::Pstring {
        count_type: Box::new(ty),
    })
}

fn parse_option_type(value: &[Value], prot: &Protocol) -> anyhow::Result<FieldType> {
    let of = value
        .get(0)
        .ok_or(anyhow!("missing option metadata"))?
        .as_str()
        .ok_or(anyhow!("option metadata must be a string"))?;

    let of = parse_field_type("option", &[Value::String(of.to_owned())], prot)?;

    Ok(FieldType::Option { of: Box::new(of) })
}

fn parse_entity_metadata_loop_type(value: &[Value], prot: &Protocol) -> anyhow::Result<FieldType> {
    let value = value
        .get(0)
        .ok_or(anyhow!("missing entity_metadata_loop metadata"))?
        .as_object()
        .ok_or(anyhow!("entity_metadata_loop metadata must be an object"))?;

    let end_val = value
        .get("endVal")
        .ok_or(anyhow!("entity_metadata_loop missing endVal"))?
        .as_i64()
        .ok_or(anyhow!("entity_metadata_loop endVal must be an integer"))?;

    let ty = value
        .get("type")
        .ok_or(anyhow!("entity_metadata_loop missing type"))?
        .as_array()
        .ok_or(anyhow!("entity_metadata_loop type must be an array"))?;

    let ty = parse_field_type("entity_metadata_loop", ty.as_slice(), prot)?;

    Ok(FieldType::EntityMetadataLoop {
        end_value: end_val as u8,
        of: Box::new(ty),
    })
}

fn parse_bitfield_type(value: &[Value]) -> anyhow::Result<FieldType> {
    let value = value
        .get(0)
        .ok_or(anyhow!("missing bitfield metadata"))?
        .as_array()
        .ok_or(anyhow!("bitfield metadata must be an array"))?;

    let mut fields = IndexMap::with_capacity(value.len());

    for field in value {
        let field = field
            .as_object()
            .ok_or(anyhow!("bitfield field must be an object"))?;

        let name = field
            .get("name")
            .ok_or(anyhow!("bitfield field has no name"))?
            .as_str()
            .ok_or(anyhow!("bitfield field name must be a string"))?;

        let size = field
            .get("size")
            .ok_or(anyhow!("bitfield field has no size"))?
            .as_i64()
            .ok_or(anyhow!("bitfield field size must be an integer"))?;

        fields.insert(name.to_owned(), size as u32);
    }

    Ok(FieldType::Bitfield { fields })
}

fn parse_container_type(value: &[Value], prot: &Protocol) -> anyhow::Result<FieldType> {
    let value = value
        .get(0)
        .ok_or(anyhow!("container has no metadata"))?
        .as_array()
        .ok_or(anyhow!("container metadata must be an array"))?;

    let mut fields = IndexMap::with_capacity(value.len());

    for field in value {
        let field = field
            .as_object()
            .ok_or(anyhow!("container field must be an object"))?;

        let is_anon = field
            .get("anon")
            .unwrap_or(&Value::Bool(false))
            .as_bool()
            .ok_or(anyhow!(
                "container anonymous field parameter must be a boolean"
            ))?;

        let name = if is_anon {
            "anonymous"
        } else {
            field
                .get("name")
                .ok_or(anyhow!(
                    "non-anonymous container field must have a name parameter"
                ))?
                .as_str()
                .ok_or(anyhow!("container field name must be a string"))?
        };

        let ty = field
            .get("type")
            .ok_or(anyhow!("container field must have a type"))?;

        let ty = parse_arbitrary_type(ty, "container", prot)?;

        fields.insert(name.to_owned(), ty);
    }

    Ok(FieldType::Container { fields })
}

fn parse_switch_type(value: &[Value], prot: &Protocol) -> anyhow::Result<FieldType> {
    let value = value
        .get(0)
        .ok_or(anyhow!("switch metadata missing"))?
        .as_object()
        .ok_or(anyhow!("switch metadata must be an object"))?;

    let compare_to = value
        .get("compareTo")
        .ok_or(anyhow!("switch metadata missing compareTo"))?
        .as_str()
        .ok_or(anyhow!("switch metadata compareTo must be a string"))?;

    let fields = value
        .get("fields")
        .ok_or(anyhow!("switch metadata missing fields"))?
        .as_object()
        .ok_or(anyhow!("switch metadata fields must be an object"))?;

    let mut fields_map = HashMap::with_capacity(fields.len());

    for (field_case, field) in fields {
        let field_case = Literal::from_str(field_case);

        let field_ty = parse_arbitrary_type(field, "switch", prot)?;

        fields_map.insert(field_case, field_ty);
    }

    Ok(FieldType::Switch {
        compare_to: compare_to.to_owned(),
        fields: fields_map,
    })
}

fn parse_array_type(value: &[Value], prot: &Protocol) -> anyhow::Result<FieldType> {
    let value = value
        .get(0)
        .ok_or(anyhow!("array missing metadata"))?
        .as_object()
        .ok_or(anyhow!("array metadata must be an object"))?;

    let count_type = value
        .get("countType")
        .ok_or(anyhow!("array metadata missing countType"))?;

    let count_type = parse_arbitrary_type(count_type, "array", prot)?;

    let of = value
        .get("type")
        .ok_or(anyhow!("array metadata missing type"))?;

    let of = parse_arbitrary_type(of, "array", prot)?;

    Ok(FieldType::Array {
        count_type: Box::new(count_type),
        of: Box::new(of),
    })
}

fn parse_arbitrary_type(
    value: &Value,
    for_field: &str,
    prot: &Protocol,
) -> anyhow::Result<FieldType> {
    let ty = if let Some(s) = value.as_str() {
        parse_field_type(for_field, &[Value::String(s.to_owned())], prot)?
    } else if let Some(a) = value.as_array() {
        parse_field_type(for_field, a, prot)?
    } else {
        bail!(
            "field type for '{}' must be either a string or an array",
            for_field
        );
    };

    Ok(ty)
}
