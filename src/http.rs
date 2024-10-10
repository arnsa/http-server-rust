use std::{fmt::Display, str::{self, FromStr}};

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl FromStr for Method {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Method, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "PATCH" => Ok(Method::PATCH),
            "DELETE" => Ok(Method::DELETE),
            _ => Err("Unknown HTTP method"),
        }
    }
}

#[allow(dead_code)]
pub enum HttpCode {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl HttpCode {
    pub fn to_u16(&self) -> u16 {
        match self {
            HttpCode::Ok => 200,
            HttpCode::Created => 201,
            HttpCode::Accepted => 202,
            HttpCode::NoContent => 204,
            HttpCode::MovedPermanently => 301,
            HttpCode::Found => 302,
            HttpCode::NotModified => 304,
            HttpCode::BadRequest => 400,
            HttpCode::Unauthorized => 401,
            HttpCode::Forbidden => 403,
            HttpCode::NotFound => 404,
            HttpCode::MethodNotAllowed => 405,
            HttpCode::InternalServerError => 500,
            HttpCode::NotImplemented => 501,
            HttpCode::BadGateway => 502,
            HttpCode::ServiceUnavailable => 503,
        }
    }
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            HttpCode::Ok => "OK",
            HttpCode::Created => "Created",
            HttpCode::Accepted => "Accepted",
            HttpCode::NoContent => "No Content",
            HttpCode::MovedPermanently => "Moved Permanently",
            HttpCode::Found => "Found",
            HttpCode::NotModified => "Not Modified",
            HttpCode::BadRequest => "Bad Request",
            HttpCode::Unauthorized => "Unauthorized",
            HttpCode::Forbidden => "Forbidden",
            HttpCode::NotFound => "Not Found",
            HttpCode::MethodNotAllowed => "Method Not Allowed",
            HttpCode::InternalServerError => "Internal Server Error",
            HttpCode::NotImplemented => "Not Implemented",
            HttpCode::BadGateway => "Bad Gateway",
            HttpCode::ServiceUnavailable => "Service Unavailable",
        };
        write!(f, "{}", result)
    }
}

pub enum HttpHeader {
    ContentType(String),
    ContentLength(usize),
    ContentEncoding(String),
}

impl Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpHeader::ContentType(value) => write!(f, "Content-Type: {}", value),
            HttpHeader::ContentLength(value) => write!(f, "Content-Length: {}", value),
            HttpHeader::ContentEncoding(value) => write!(f, "Content-Encoding: {}", value),
        }
    }
}