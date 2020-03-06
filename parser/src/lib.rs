use async_std::fs::File;
use async_std::io::prelude::WriteExt;
use async_std::io::ReadExt;
use async_std::path::Path;
use std::str::Chars;
use thiserror::Error;

pub async fn compile_def(input: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut input = File::open(input).await?;
    let mut output = File::open(output).await?;

    let mut in_str = String::new();
    input.read_to_string(&mut in_str).await?;
    in_str.retain(|char| char != '\n');

    let out_str = compile_str(&mut in_str)?;
    output.write_all(out_str.as_bytes()).await?;

    Ok(())
}

struct SyntaxTree {
    clientbound: Vec<Def>,
    serverbound: Vec<Def>,
}

enum Def {
    Packet { name: String, fields: Vec<Field> },
    Annotation(Annotation),
}

struct Field {
    ty: FieldType,
    name: String,
    init: Option<FieldInitializer>,
}

struct FieldType {
    ty: ConcreteType,
    convert_from: Option<FieldConvertFrom>,
}

struct FieldConvertFrom {
    ty: ConcreteType,
    field: Option<ConcreteTypeField>,
}

enum ConcreteType {
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
    Identifier,
    Block,
    Item,
    Optional(Box<ConcreteType>),
    Array(Box<ConcreteType>),
    Enum(Box<Enum>),
    Struct(Struct),
}

enum ConcreteValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Ubyte(u8),
    Ushort(u16),
    Uint(u32),
    Ulong(u64),
    Boolean(bool),
    String(String),
    Chat(String),
    Identifier(String),
}

struct Enum {
    repr: ConcreteType,
    cases: Vec<EnumCase>,
}

struct EnumCase {
    name: String,
    fields: Option<Struct>,
    value: ConcreteValue,
}

struct Struct {
    name: Option<String>,
    fields: Vec<Field>,
}

enum ConcreteTypeField {
    BlockId,
    ItemId,
    BlockType,
}

enum FieldInitializer {
    ArrayLength { array_field: String },
}

enum Annotation {
    Manual { path: String },
    Skip { packet_name: String },
    SkipTo { id: u32 },
}

fn compile_str(input: &mut str) -> anyhow::Result<String> {
    let mut output = String::new();

    let tree = parse_str(input)?;

    Ok(output)
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("expected keyword {.keyword}")]
    ExpectedKeyword { keyword: &'static str },
}

fn parse_str(input: &mut str) -> anyhow::Result<SyntaxTree> {
    let mut tree = SyntaxTree {
        clientbound: vec![],
        serverbound: vec![],
    };

    let mut input = input.chars();

    parse_defs(&mut input, &mut tree.clientbound, "clientbound")?;
    parse_defs(&mut input, &mut tree.serverbound, "serverbound")?;

    Ok(tree)
}

fn parse_defs(input: &mut Chars, defs: &mut Vec<Def>, bound: &'static str) -> anyhow::Result<()> {
    parse_keyword(input, bound)?;

    parse_block(input, |input_str| {
        let mut input = input_str.chars();

        Ok(())
    });
}

fn parse_block(
    input: &mut Chars,
    f: impl FnOnce(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    parse_keyword(input, "{")?;
}

fn parse_keyword(input: &mut Chars, keyword: &'static str) -> anyhow::Result<()> {
    let mut keyword_iter = keyword.chars();
    loop {
        if let Some(expected) = keyword_iter.next() {
            let char = input
                .next()
                .ok_or(ParseError::ExpectedKeyword { keyword })?;

            if char.is_whitespace() {
                continue;
            }

            if char != expected {
                return Err(ParseError::ExpectedKeyword { keyword }.into());
            }
        } else {
            return Ok(());
        }
    }
}
