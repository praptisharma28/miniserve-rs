use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request{
    fn from_byte_array(buf: &[u8]) -> Result<Self, String>{
        unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request format",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Unsupported protocol version",
            Self::InvalidMethod => "Unsupported HTTP method",
        }
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {
}