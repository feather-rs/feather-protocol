use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid repr {0:?} for enum {1}")]
    InvalidEnumRepr(i64, &'static str),
}
