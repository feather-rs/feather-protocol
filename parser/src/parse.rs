use std::collections::HashSet;
use std::str::{Chars, FromStr};
use strum::*;
use strum_macros::*;
use thiserror::Error;

#[derive(Debug, Clone, Default)]
pub struct SyntaxTree {
    pub constructs: Vec<Construct>,
}

#[derive(Debug, Clone)]
pub enum Construct {
    Keyword(Keyword),
    Identifier(String),
    Block(SyntaxTree),
    Parenthesized(SyntaxTree),
    Literal(Literal),
    Token(Token),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Semicolon,  // ;
    Comma,      // ,
    Equals,     // =
    Yields,     // =>
    Array,      // []
    Annotation, // @
}

impl Token {
    fn from_str(x: &str) -> Option<Self> {
        match x {
            ";" => Some(Token::Semicolon),
            "," => Some(Token::Comma),
            "=" => Some(Token::Equals),
            "=>" => Some(Token::Yields),
            "[]" => Some(Token::Array),
            "@" => Some(Token::Annotation),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, EnumIter)]
pub enum Keyword {
    Clientbound,
    Serverbound,
    Byte,
    Short,
    Int,
    Long,
    Ubyte,
    Ushort,
    Uint,
    Ulong,
    Struct,
    Enum,
    Block,
    Item,
    Identifier,
    Chat,
    Boolean,
    Position,
    Nbt,
    Varint,
    Uuid,
    Float,
    Angle,
    Double,
}

pub fn parse_str(input: &str) -> anyhow::Result<SyntaxTree> {
    let mut tree = SyntaxTree::default();

    let mut input = input.chars();

    parse(&mut input, &mut tree, None)?;

    Ok(tree)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected end of input")]
    UnexpectedEof,
}

fn parse(input: &mut Chars, tree: &mut SyntaxTree, until: Option<char>) -> Result<(), Error> {
    let mut current = String::new();
    loop {
        if let Some(char) = input.next() {
            if until.map_or(false, |until| until == char) {
                break;
            }

            if char == '{' {
                let mut block = SyntaxTree::default();
                parse(input, &mut block, Some('}'))?;
                tree.constructs.push(Construct::Block(block));
                current.clear();
                continue;
            } else if char == '(' {
                let mut parenthesized = SyntaxTree::default();
                parse(input, &mut parenthesized, Some(')'))?;
                tree.constructs
                    .push(Construct::Parenthesized(parenthesized));
                current.clear();
                continue;
            } else if char == '"' {
                tree.constructs
                    .push(Construct::Literal(parse_string_literal(input)?));
                current.clear();
                continue;
            } else if let Some(token) = Token::from_str(&char.to_string()) {
                if !current.is_empty() {
                    tree.constructs.push(construct(&current)?);
                }
                current.clear();
                tree.constructs.push(Construct::Token(token));
                continue;
            }

            if char.is_whitespace() {
                if !current.is_empty() {
                    tree.constructs.push(construct(&current)?);
                }
                current.clear();
            } else {
                current.push(char);
            }
        } else {
            if until.is_some() {
                return Err(Error::UnexpectedEof);
            } else {
                break;
            }
        }
    }

    Ok(())
}

fn parse_string_literal(input: &mut Chars) -> Result<Literal, Error> {
    let mut s = String::new();

    loop {
        let char = input.next().ok_or(Error::UnexpectedEof)?;

        if char == '"' {
            return Ok(Literal::String(s));
        } else {
            s.push(char);
        }
    }
}

fn construct(from: &str) -> Result<Construct, Error> {
    let keywords: HashSet<String> = Keyword::iter()
        .map(|keyword| keyword.to_string().to_lowercase())
        .collect();

    let construct = if keywords.contains(from) {
        Construct::Keyword(keyword_from_str(from))
    } else if let Ok(x) = i64::from_str(from) {
        Construct::Literal(Literal::Integer(x))
    } else if let Some(token) = Token::from_str(from) {
        Construct::Token(token)
    } else {
        Construct::Identifier(from.to_string())
    };

    Ok(construct)
}

fn keyword_from_str(x: &str) -> Keyword {
    Keyword::from_str(&capitalize_first(x)).unwrap()
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    chars
        .next()
        .map(|first_letter| first_letter.to_uppercase())
        .into_iter()
        .flatten()
        .chain(chars)
        .collect()
}
