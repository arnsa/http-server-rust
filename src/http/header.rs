use std::fmt::Display;

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
