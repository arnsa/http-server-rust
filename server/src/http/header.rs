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