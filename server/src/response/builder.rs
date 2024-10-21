use std::fmt::Display;

use crate::http::{code::HttpCode, header::HttpHeader};

pub struct Response {
    pub status_code: HttpCode,
    pub status_text: String,
    pub http_version: String,
    pub headers: Option<Vec<HttpHeader>>,
    pub body: Option<String>,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::new();
        let mut body = String::from("\r\n\r\n");

        if let Some(h) = &self.headers {
            headers = h
                .iter()
                .map(|h| format!("\r\n{}", h.to_string()))
                .collect::<String>();
        }

        if let Some(b) = &self.body {
            body = format!("\r\n\r\n{}", b);
        }

        write!(
            f,
            "{} {} {}{}{}",
            &self.http_version,
            &self.status_code.to_u16(),
            &self.status_text,
            &headers,
            &body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{code::HttpCode, header::HttpHeader};

    #[test]
    fn test_display_response_with_headers_and_body() {
        let response = Response {
            status_code: HttpCode::Ok,
            status_text: "OK".to_string(),
            http_version: "HTTP/1.1".to_string(),
            headers: Some(vec![
                HttpHeader::ContentType("application/json".to_string()),
                HttpHeader::ContentEncoding("deflate".to_string()),
                HttpHeader::ContentLength(27),
            ]),
            body: Some("{\"message\":\"Success\"}".to_string()),
        };

        let expected = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Encoding: deflate\r\nContent-Length: 27\r\n\r\n{\"message\":\"Success\"}";
        let formatted = format!("{}", response);

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_display_response_without_headers() {
        let response = Response {
            status_code: HttpCode::NotFound,
            status_text: "Not Found".to_string(),
            http_version: "HTTP/1.0".to_string(),
            headers: None,
            body: Some("The requested resource was not found.".to_string()),
        };

        let expected = "HTTP/1.0 404 Not Found\r\n\r\nThe requested resource was not found.";
        let formatted = format!("{}", response);

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_display_response_without_body() {
        let response = Response {
            status_code: HttpCode::NoContent,
            status_text: "No Content".to_string(),
            http_version: "HTTP/1.1".to_string(),
            headers: Some(vec![
                HttpHeader::ContentType("text/plain".to_string()),
                HttpHeader::ContentEncoding("gzip".to_string()),
            ]),
            body: None,
        };

        let expected = "HTTP/1.1 204 No Content\r\nContent-Type: text/plain\r\nContent-Encoding: gzip\r\n\r\n";
        let formatted = format!("{}", response);

        assert_eq!(formatted, expected);
    }
}