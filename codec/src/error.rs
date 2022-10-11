use std::io;

#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error("IO error:{0}")]
    IOError(#[from] io::Error),

    #[error("unknown error:{0}")]
    Other(String),
}
