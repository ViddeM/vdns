use crate::messages::parsing::ReaderError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Reader error")]
    BufferReadError(#[from] ReaderError),
    #[error("Question error")]
    Question,
    #[error("Answer error")]
    Answer,
    #[error("Authority error")]
    Authority,
    #[error("Additional error")]
    Additional,
    #[error("Domain name error, '{0}'")]
    DomainNameError(String),
    #[error("Resource record error, '{0}'")]
    RRError(String),
}

pub type ParseResult<T> = Result<T, ParseError>;
