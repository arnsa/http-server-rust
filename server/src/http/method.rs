//! # HTTP Method Module
//!
//! This module defines the `HttpMethod` enum, representing the standard HTTP methods used in web communication.
//! It provides functionality to parse these methods from strings using the `FromStr` trait.
//!
//! ## Usage
//!
//! ```rust
//! use server::http::method::HttpMethod;
//! use std::str::FromStr;
//!
//! // Parsing a valid HTTP method
//! let method = HttpMethod::from_str("GET").unwrap();
//! assert_eq!(method, HttpMethod::GET);
//!
//! // Attempting to parse an invalid HTTP method
//! let invalid_method = HttpMethod::from_str("CONNECT");
//! assert!(invalid_method.is_err());
//! ```

use std::str::{self, FromStr};

/// Represents standard HTTP methods.
///
/// The `HttpMethod` enum includes common HTTP methods used in web communication.
/// Each variant corresponds to a specific HTTP request method.
#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    /// `GET` method.
    ///
    /// The `GET` method requests a representation of the specified resource.
    /// Requests using `GET` should only retrieve data and should have no other effect.
    GET,

    /// `POST` method.
    ///
    /// The `POST` method submits data to be processed to a specified resource.
    /// The data is included in the body of the request.
    POST,

    /// `PUT` method.
    ///
    /// The `PUT` method replaces all current representations of the target resource
    /// with the uploaded content.
    PUT,

    /// `PATCH` method.
    ///
    /// The `PATCH` method applies partial modifications to a resource.
    /// It is used to make partial updates without replacing the entire resource.
    PATCH,

    /// `DELETE` method.
    ///
    /// The `DELETE` method removes all current representations of the target resource
    /// given by a URL.
    DELETE,
}

impl FromStr for HttpMethod {
    type Err = &'static str;

    /// Parses a string into an `HttpMethod`.
    ///
    /// This implementation converts a string slice to its corresponding `HttpMethod` variant.
    /// If the input string does not match any known HTTP method, it returns an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use server::http::method::HttpMethod;
    /// use std::str::FromStr;
    ///
    /// // Successful parsing
    /// let method = HttpMethod::from_str("POST").unwrap();
    /// assert_eq!(method, HttpMethod::POST);
    ///
    /// // Parsing an unknown method
    /// let invalid_method = HttpMethod::from_str("CONNECT");
    /// assert!(invalid_method.is_err());
    /// assert_eq!(invalid_method.unwrap_err(), "Unknown HTTP method");
    /// ```
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
