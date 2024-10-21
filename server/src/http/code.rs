//! # HTTP Code Module
//!
//! This module defines the `HttpCode` enum, representing various HTTP status codes.
//! It provides functionality to convert these codes to their numerical (`u16`) representations
//! and to format them as human-readable strings.
//!
//! ## Usage
//!
//! ```rust
//! use server::http::code::HttpCode;
//!
//! let status = HttpCode::Ok;
//! println!("Status Code: {}", status.to_u16()); // Outputs: Status Code: 200
//! println!("Status Text: {}", status); // Outputs: Status Text: OK
//! ```

use std::fmt::Display;

/// Represents standard HTTP status codes.
///
/// The `HttpCode` enum includes common HTTP status codes used in web communication.
/// Each variant corresponds to a specific HTTP response status.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
pub enum HttpCode {
    /// 200 OK: The request has succeeded.
    Ok = 200,
    /// 201 Created: The request has been fulfilled and resulted in a new resource being created.
    Created = 201,
    /// 202 Accepted: The request has been accepted for processing, but the processing has not been completed.
    Accepted = 202,
    /// 204 No Content: The server successfully processed the request, and is not returning any content.
    NoContent = 204,
    /// 301 Moved Permanently: The requested resource has been assigned a new permanent URI.
    MovedPermanently = 301,
    /// 302 Found: The requested resource resides temporarily under a different URI.
    Found = 302,
    /// 304 Not Modified: Indicates that the resource has not been modified since the last request.
    NotModified = 304,
    /// 400 Bad Request: The server could not understand the request due to invalid syntax.
    BadRequest = 400,
    /// 401 Unauthorized: The client must authenticate itself to get the requested response.
    Unauthorized = 401,
    /// 403 Forbidden: The client does not have access rights to the content.
    Forbidden = 403,
    /// 404 Not Found: The server can not find the requested resource.
    NotFound = 404,
    /// 405 Method Not Allowed: The request method is known by the server but has been disabled and cannot be used.
    MethodNotAllowed = 405,
    /// 500 Internal Server Error: The server has encountered a situation it doesn't know how to handle.
    InternalServerError = 500,
    /// 501 Not Implemented: The request method is not supported by the server and cannot be handled.
    NotImplemented = 501,
    /// 502 Bad Gateway: The server was acting as a gateway or proxy and received an invalid response from the upstream server.
    BadGateway = 502,
    /// 503 Service Unavailable: The server is not ready to handle the request.
    ServiceUnavailable = 503,
}

impl HttpCode {
    /// Converts the `HttpCode` variant to its corresponding numerical (`u16`) value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use server::http::code::HttpCode;
    ///
    /// let status = HttpCode::Ok;
    /// assert_eq!(status.to_u16(), 200);
    /// ```
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
    /// Formats the `HttpCode` as its standard textual representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use server::http::code::HttpCode;
    ///
    /// let status = HttpCode::Ok;
    /// assert_eq!(format!("{}", status), "OK");
    /// ```
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