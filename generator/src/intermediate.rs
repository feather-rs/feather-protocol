use crate::model::{self, FieldType};
use model::EnumTag;
use std::convert::TryFrom;

/// An intermediate representation of the input data.
/// Designed for reading by the generator.
#[derive(Debug)]
pub struct Root {
    /// The set of structs defined by this file.
    pub structs: Vec<Struct>,
    /// The set of enums defined by this file.
    pub enums: Vec<Enum>,
    /// The set of packets defined by this file.
    pub packets: Vec<Packet>,
}

impl TryFrom<model::Root> for Root {
    type Error = anyhow::Error;

    fn try_from(mut model: model::Root) -> Result<Self, Self::Error> {
        // Read in structs and enums.
        // Note that types can have nested types
        // as well.
        let mut structs = Vec::new();
        let mut enums = Vec::new();

        for (name, typ) in &model.types {
            process_type(name.into(), typ, &mut structs, &mut enums);
        }

        // Read in packets
        let stages = [
            ("login", Stage::Login),
            ("handshake", Stage::Handshake),
            ("status", Stage::Status),
            ("play", Stage::Play),
        ];
        let directions = [
            ("clientbound", Direction::Clientbound),
            ("serverbound", Direction::Serverbound),
        ];

        let mut packets = Vec::new();
        for &(stage_name, stage) in &stages {
            let mut stage_packets = model.packets.remove(stage_name).unwrap_or_default();

            for &(direction_name, direction) in &directions {
                let direction_packets = stage_packets.0.remove(direction_name).unwrap_or_default();

                for (packet_name, packet) in direction_packets {
                    // Find the packet's struct (same name as `packet_name`).
                    let strukt = structs
                        .iter()
                        .position(|strukt| strukt.name == packet_name)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "could not find corresponding struct for packet `{}`",
                                packet_name
                            )
                        })?;

                    let packet = Packet {
                        strukt,
                        stage,
                        direction,
                        id: packet.id,
                    };

                    packets.push(packet);
                }
            }
        }

        Ok(Self {
            structs,
            enums,
            packets,
        })
    }
}

fn process_type(name: String, typ: &model::Type, structs: &mut Vec<Struct>, enums: &mut Vec<Enum>) {
    // Recursively process inner `Custom` field variants.
    let inner_field_types = inner_field_types(typ);
    for field in inner_field_types {
        if let FieldType::Custom(custom) = field {
            let (nested_name, typ) = match *(&*custom).clone() {
                model::CustomType::Struct { name, fields } => {
                    (name, model::Type::Struct(model::Struct { fields }))
                }
                model::CustomType::Enum {
                    name,
                    variants,
                    tag,
                } => (name, model::Type::Enum(model::Enum { variants, tag })),
            };

            // Name of a nested type is the name of the
            // current type concatenated with the nested type's name.
            let name = format!("{}_{}", name, nested_name);

            process_type(name, &typ, structs, enums);
        }
    }

    // Create either a `Struct` or an `Enum`, then add
    // it to the corresponding vector.
    match typ {
        model::Type::Struct(s) => {
            let strukt = process_struct(name, s);
            structs.push(strukt);
        }
        model::Type::Enum(e) => {
            let en = process_enum(name, e);
            enums.push(en);
        }
    }
}

fn process_struct(name: String, s: &model::Struct) -> Struct {
    Struct {
        name: name.into(),
        fields: s
            .fields
            .iter()
            .map(|(name, typ)| Field {
                name: name.clone(),
                typ: typ.clone(),
            })
            .collect(),
    }
}

fn process_enum(name: String, e: &model::Enum) -> Enum {
    let variants = e
        .variants
        .iter()
        .map(|(tag, variant)| {
            let name = variant.name.to_owned();
            EnumVariant {
                name,

                fields: variant
                    .fields
                    .iter()
                    .map(|(name, typ)| Field {
                        name: name.clone(),
                        typ: typ.clone(),
                    })
                    .collect(),
                tag: tag.clone(),
            }
        })
        .collect();
    Enum {
        name: name.into(),
        tag: *e.tag.clone(),
        variants,
    }
}

/// Returns all field types contained within the given type.
fn inner_field_types<'a>(typ: &'a model::Type) -> Box<dyn Iterator<Item = &'a FieldType> + 'a> {
    match typ {
        model::Type::Struct(s) => {
            let fields_struct = s.fields.values();
            Box::new(fields_struct)
        }
        model::Type::Enum(e) => {
            let fields_enum = e
                .variants
                .values()
                .flat_map(|variant| variant.fields.values());
            Box::new(fields_enum)
        }
    }
}

#[derive(Debug)]
pub struct Struct {
    /// The name of this struct.
    pub name: String,
    /// The fields of this struct.
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Enum {
    /// The name of this enum.
    pub name: String,
    /// The tag type for this enum—indicates
    /// which variant is present.
    ///
    /// Always either an integer type or a string.
    pub tag: FieldType,
    /// Mapping from tag value => variant.
    pub variants: Vec<EnumVariant>,
}

/// A variant of an enum.
#[derive(Debug)]
pub struct EnumVariant {
    /// This variant's name.
    pub name: String,
    /// This variant's tag value.
    pub tag: EnumTag,
    /// Fields of this variant.
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    /// The name of this field.
    pub name: String,
    /// The type of this field.
    pub typ: FieldType,
}

#[derive(Debug)]
pub struct Packet {
    /// Index into `Root.types` of this packet's struct.
    strukt: usize,
    /// The ID of this packet.
    pub id: u32,
    /// The direction of this packet.
    pub direction: Direction,
    /// The stage during which this packet is valid.
    pub stage: Stage,
}

impl Packet {
    /// Given the rest of the `Root` model, returns
    /// the struct corresponding to this packet.
    pub fn strukt<'a>(&self, model: &'a Root) -> &'a Struct {
        &model.structs[self.strukt]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Clientbound,
    Serverbound,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Stage {
    Handshake,
    Status,
    Login,
    Play,
}
