use combine::easy::Errors;
use thiserror::Error;

use crate::position::Pos;
use crate::tokenizer::Token;

pub type InternalError<'a> = Errors<Token<'a>, Token<'a>, Pos>;

/// Error parsing schema
///
/// This structure is opaque for forward compatibility. We are exploring a
/// way to improve both error message and API.
#[derive(Error, Debug)]
#[error("schema parse error: {}", _0)]
pub struct ParseError<'a>(InternalError<'a>);

impl<'a> From<InternalError<'a>> for ParseError<'a> {
    fn from(e: InternalError<'a>) -> ParseError<'a> {
        ParseError(e)
    }
}
