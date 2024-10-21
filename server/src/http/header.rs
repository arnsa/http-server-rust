//! # HTTP Header Module
//!
//! This module defines the `HttpHeader` enum, representing various HTTP headers commonly used in HTTP requests and responses.
//! It provides functionality to format these headers as human-readable strings using the `Display` trait.
//!
//! ## Usage
//!
//! ```rust
//! use server::http::header::HttpHeader;
//!
//! let content_type = HttpHeader::ContentType("application/json".to_string());
//! let content_length = HttpHeader::ContentLength(348);
//! let content_encoding = HttpHeader::ContentEncoding("gzip".to_string());
//!
//! println!("{}", content_type);        // Outputs: Content-Type: application/json
//! println!("{}", content_length);      // Outputs: Content-Length: 348
//! println!("{}", content_encoding);    // Outputs: Content-Encoding: gzip
//! ```

use std::fmt::Display;

/// Represents standard HTTP headers.
///
/// The `HttpHeader` enum includes common HTTP headers used in web communication.
/// Each variant corresponds to a specific HTTP header field.
///
/// ## Variants
///
/// - `ContentType(String)`: Specifies the media type of the resource.
/// - `ContentLength(usize)`: Indicates the size of the response body in bytes.
/// - `ContentEncoding(String)`: Defines the encoding transformations that have been applied to the resource.
///
/// ## Examples
///
/// ```rust
/// use server::http::header::HttpHeader;
///
/// let header1 = HttpHeader::ContentType("application/json".to_string());
/// let header2 = HttpHeader::ContentLength(1234);
/// let header3 = HttpHeader::ContentEncoding("gzip".to_string());
///
/// println!("{}", header1); // Outputs: Content-Type: application/json
/// println!("{}", header2); // Outputs: Content-Length: 1234
/// println!("{}", header3); // Outputs: Content-Encoding: gzip
/// ```
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum HttpHeader {
    /// `Content-Type` header field.
    ///
    /// Specifies the media type of the resource.
    /// For example, `application/json` or `text/html`.
    ContentType(String),

    /// `Content-Length` header field.
    ///
    /// Indicates the size of the response body in bytes.
    /// For example, `348`.
    ContentLength(usize),

    /// `Content-Encoding` header field.
    ///
    /// Defines the encoding transformations that have been applied to the resource.
    /// For example, `gzip` or `deflate`.
    ContentEncoding(String),
}

impl Display for HttpHeader {
    /// Formats the `HttpHeader` as a string.
    ///
    /// Each variant is formatted according to the standard HTTP header format:
    /// `Header-Name: Header-Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use server::http::header::HttpHeader;
    ///
    /// let header = HttpHeader::ContentType("application/json".to_string());
    /// assert_eq!(format!("{}", header), "Content-Type: application/json");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpHeader::ContentType(value) => write!(f, "Content-Type: {}", value),
            HttpHeader::ContentLength(value) => write!(f, "Content-Length: {}", value),
            HttpHeader::ContentEncoding(value) => write!(f, "Content-Encoding: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_content_type() {
        let header = HttpHeader::ContentType("application/json".to_string());
        assert_eq!(format!("{}", header), "Content-Type: application/json");

        let header_empty = HttpHeader::ContentType("".to_string());
        assert_eq!(format!("{}", header_empty), "Content-Type: ");
    }

    #[test]
    fn test_display_content_length() {
        let header = HttpHeader::ContentLength(348);
        assert_eq!(format!("{}", header), "Content-Length: 348");

        let header_zero = HttpHeader::ContentLength(0);
        assert_eq!(format!("{}", header_zero), "Content-Length: 0");

        let header_large = HttpHeader::ContentLength(usize::MAX);
        assert_eq!(format!("{}", header_large), format!("Content-Length: {}", usize::MAX));
    }

    #[test]
    fn test_display_content_encoding() {
        let header = HttpHeader::ContentEncoding("gzip".to_string());
        assert_eq!(format!("{}", header), "Content-Encoding: gzip");

        let header_empty = HttpHeader::ContentEncoding("".to_string());
        assert_eq!(format!("{}", header_empty), "Content-Encoding: ");
    }

    #[test]
    fn test_multiple_headers() {
        let headers = vec![
            HttpHeader::ContentType("text/html".to_string()),
            HttpHeader::ContentLength(1024),
            HttpHeader::ContentEncoding("deflate".to_string()),
        ];

        let expected = vec![
            "Content-Type: text/html",
            "Content-Length: 1024",
            "Content-Encoding: deflate",
        ];

        for (header, &exp) in headers.iter().zip(expected.iter()) {
            assert_eq!(format!("{}", header), exp);
        }
    }
}