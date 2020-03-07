use crate::EnumRepr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid repr {0:?} for enum {1}")]
    InvalidEnumRepr(EnumRepr, &'static str),
}
