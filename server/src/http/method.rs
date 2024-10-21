use std::str::{self, FromStr};

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_valid_methods() {
        assert_eq!(HttpMethod::from_str("GET").unwrap(), HttpMethod::GET);
        assert_eq!(HttpMethod::from_str("POST").unwrap(), HttpMethod::POST);
        assert_eq!(HttpMethod::from_str("PUT").unwrap(), HttpMethod::PUT);
        assert_eq!(HttpMethod::from_str("PATCH").unwrap(), HttpMethod::PATCH);
        assert_eq!(HttpMethod::from_str("DELETE").unwrap(), HttpMethod::DELETE);
    }

    #[test]
    fn test_from_str_invalid_methods() {
        assert_eq!(
            HttpMethod::from_str("OPTIONS").unwrap_err(),
            "Unknown HTTP method"
        );

        assert_eq!(HttpMethod::from_str("").unwrap_err(), "Unknown HTTP method");

        assert_eq!(
            HttpMethod::from_str("get").unwrap_err(),
            "Unknown HTTP method"
        );
    }
}
