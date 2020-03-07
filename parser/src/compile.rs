pub struct CompiledTree {
    clientbound: Vec<Def>,
    serverbound: Vec<Def>,
}

pub enum Def {
    Packet { name: String, fields: Vec<Field> },
    Annotation(Annotation),
}

pub struct Field {
    ty: FieldType,
    name: String,
    init: Option<FieldInitializer>,
}

pub struct FieldType {
    ty: ConcreteType,
    convert_from: Option<FieldConvertFrom>,
}

pub struct FieldConvertFrom {
    ty: ConcreteType,
    field: Option<ConcreteTypeField>,
}

pub enum ConcreteType {
    Byte,
    Short,
    Int,
    Long,
    Ubyte,
    Ushort,
    Uint,
    Ulong,
    Boolean,
    String,
    Chat,
    Angle,
    Identifier,
    Block,
    Item,
    Optional(Box<ConcreteType>),
    Array(Box<ConcreteType>),
    Enum(Box<Enum>),
    Struct(Struct),
}

pub enum ConcreteValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Ubyte(u8),
    Ushort(u16),
    Uint(u32),
    Ulong(u64),
    Boolean(bool),
    Angle(u8),
    String(String),
    Chat(String),
    Identifier(String),
}

pub struct Enum {
    repr: ConcreteType,
    cases: Vec<EnumCase>,
}

pub struct EnumCase {
    name: String,
    fields: Option<Struct>,
    value: ConcreteValue,
}

pub struct Struct {
    name: Option<String>,
    fields: Vec<Field>,
}

pub enum ConcreteTypeField {
    BlockId,
    ItemId,
    BlockType,
}

pub enum FieldInitializer {
    ArrayLength { array_field: String },
}

pub enum Annotation {
    Manual { path: String },
    Skip { packet_name: String },
    SkipTo { id: u32 },
}
