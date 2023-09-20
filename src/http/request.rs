use std::{convert::TryFrom, error::Error, str::{Utf8Error, Chars}, str, fmt::Pointer };
use core::fmt::{ Display, Formatter, Debug, Result as fmtResult};

use method::{Method, MethodError};

use crate::http::method;

pub struct Request<'inBufferStream>{
    path: &'inBufferStream str,
    query_string: Option<& 'inBufferStream str>,
    method: super::method::Method
}


impl <'inBufferStream> TryFrom<&'inBufferStream [u8]> for Request<'inBufferStream>{
    type Error = ParseError;

    fn try_from(buf: &'inBufferStream [u8]) -> Result<Request< 'inBufferStream > , Self::Error> {

        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or( ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return  Err(ParseError::InvalidProtocol);
        }


        let method: Method = method.parse()?;

        let mut query_string: Option<&str> =None;
       

        if let Some(i) = path.find("?") {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        return Ok(Self { path: path, query_string: query_string , method: method });

    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (i, c) in request.chars().enumerate(){
        if (c == ' ') || (c == '\r') {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

pub enum ParseError{
    InvalidRequest,
    InvalidProtocol,
    InvalidEncoding,
    InvalidMethod
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult{
        write!(f, "{}", self.message())
    }
}

impl ParseError {

    pub fn message(&self) -> &str {

        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethod => "Invalid Method",
        }
    }
}



impl Error for ParseError {}