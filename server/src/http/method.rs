use std::str::{self, FromStr};

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl FromStr for HttpMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<HttpMethod, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "PATCH" => Ok(HttpMethod::PATCH),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err("Unknown HTTP method"),
        }
    }
}
