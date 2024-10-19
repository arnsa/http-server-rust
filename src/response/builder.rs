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
