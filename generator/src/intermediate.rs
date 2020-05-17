use crate::model::{self, FieldType};
use std::convert::TryFrom;

/// An intermediate representation of the input data.
/// Designed for reading by the generator.
#[derive(Debug)]
pub struct Root<'a> {
    /// The set of types defined by this file.
    pub types: Vec<Type<'a>>,
    /// The set of packets defined by this file.
    pub packets: Vec<Packet>,
}

impl<'a> TryFrom<model::Root<'a>> for Root<'a> {
    type Error = anyhow::Error;

    fn try_from(mut model: model::Root<'a>) -> Result<Self, Self::Error> {
        // Read in the types
        let mut types = Vec::new();
        for (struct_name, fields) in model.structs {
            let fields = fields
                .into_iter()
                .map(|(name, typ)| Field { name, typ })
                .collect::<Vec<_>>();

            let typ = Type {
                name: struct_name,
                fields,
            };
            types.push(typ);
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
                    let strukt = types
                        .iter()
                        .position(|typ| typ.name == packet_name)
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

        Ok(Self { types, packets })
    }
}

#[derive(Debug)]
pub struct Type<'a> {
    /// The name of this type.
    pub name: &'a str,
    /// The fields of this struct (TODO: enum support)
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug)]
pub struct Field<'a> {
    /// The name of this field.
    pub name: &'a str,
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
    pub fn strukt<'a, 'b>(&self, model: &'a Root<'b>) -> &'a Type<'b> {
        &model.types[self.strukt]
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
