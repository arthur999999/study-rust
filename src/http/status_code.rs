use std::fmt::{Display, Formatter, Result as fmtResult};

#[derive(Clone, Copy)]

pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    ServerError = 500
}

impl StatusCode {
    pub fn status_reason(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "NoT Found",
            StatusCode::ServerError => "Server Error",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "{}", *self as u16)
    }
}
