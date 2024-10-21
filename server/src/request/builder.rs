use crate::{http::method::HttpMethod, url::Url};

use anyhow::{anyhow, Result};
use std::{io::Read, net::TcpStream, str::FromStr};

#[derive(Debug)]
pub struct Request {
    pub request: Vec<String>,
    pub user_agent: Option<String>,
    pub accept_encoding: Option<String>,
    pub content_length: Option<usize>,
    pub method: Option<HttpMethod>,
    pub url: Option<Url>,
    pub http_version: String,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Result<Request> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;

        let request = Self::get_request(&buffer, bytes_read)?;
        let request_line = request[0].to_string();

        let user_agent = Self::get_user_agent_header(&request);
        let accept_encoding = Self::get_accept_encoding(&request);
        let content_length = Self::get_content_length(&request);
        let method = Self::get_method(&request_line);
        let url_str = Self::get_url(&request_line);
        let url = url_str.as_ref().map(|s| Url::new(s.as_str()));
        let http_version = Self::get_http_version(&request_line);

        Ok(Request {
            request,
            user_agent,
            accept_encoding,
            content_length,
            method,
            url,
            http_version,
        })
    }

    fn get_request(buffer: &[u8], bytes_read: usize) -> Result<Vec<String>> {
        let request_str = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();

        if request_str.is_empty() {
            Err(anyhow!("Error: request is empty"))
        } else {
            let request = request_str
                .split("\r\n")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            Ok(request)
        }
    }

    fn get_user_agent_header(request: &Vec<String>) -> Option<String> {
        request
            .iter()
            .find(|line| line.contains("User-Agent:"))
            .and_then(|result| Some(result.replace("User-Agent: ", "")))
    }

    fn get_accept_encoding(request: &Vec<String>) -> Option<String> {
        request
            .iter()
            .find(|x| x.contains("Accept-Encoding"))
            .and_then(|ac| ac.split("Accept-Encoding: ").nth(1))
            .and_then(|e| Some(e.to_string()))
    }

    fn get_content_length(request: &Vec<String>) -> Option<usize> {
        request
            .iter()
            .find(|x| x.contains("Content-Length"))
            .and_then(|cl| cl.split("Content-Length: ").nth(1))
            .and_then(|cl| cl.parse::<usize>().ok())
    }

    fn get_method(request_line: &String) -> Option<HttpMethod> {
        request_line
            .split(" ")
            .nth(0)
            .and_then(|s| HttpMethod::from_str(s).ok())
    }

    fn get_url(request_line: &String) -> Option<String> {
        request_line
            .split(" ")
            .nth(1)
            .and_then(|s| Some(s.to_string()))
    }

    fn get_http_version(request_line: &String) -> String {
        request_line
            .split(" ")
            .nth(3)
            .and_then(|s| Some(s.to_string()))
            .get_or_insert_with(|| "HTTP/1.1".to_string())
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use anyhow::Result;

    fn create_stream(raw_request: &str) -> Result<(TcpStream, TcpStream)> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;
        let client = TcpStream::connect(addr)?;
        let (mut server, _) = listener.accept()?;

        server.write_all(raw_request.as_bytes())?;
        server.flush()?;

        Ok((client, server))
    }

    #[test]
    fn test_new_valid_get_request() {
        let raw_request = "GET /home HTTP/1.1\r\n\
                           Host: localhost\r\n\
                           User-Agent: TestAgent/1.0\r\n\
                           Accept-Encoding: gzip, deflate\r\n\
                           Content-Length: 0\r\n\r\n";

        let (mut client_stream, _) = create_stream(raw_request).unwrap();
        let request = Request::new(&mut client_stream).unwrap();

        assert_eq!(request.method, Some(HttpMethod::GET));
        assert_eq!(request.url, Some(Url::new("/home")));
        assert_eq!(request.http_version, "HTTP/1.1".to_string());
        assert_eq!(request.user_agent, Some("TestAgent/1.0".to_string()));
        assert_eq!(request.accept_encoding, Some("gzip, deflate".to_string()));
        assert_eq!(request.content_length, Some(0));
        assert_eq!(request.request.len(), 5);
        assert_eq!(request.request[0], "GET /home HTTP/1.1");
        assert_eq!(request.request[1], "Host: localhost");
        assert_eq!(request.request[2], "User-Agent: TestAgent/1.0");
        assert_eq!(request.request[3], "Accept-Encoding: gzip, deflate");
        assert_eq!(request.request[4], "Content-Length: 0");
    }

    #[test]
    fn test_new_valid_post_request_with_body() {
        let raw_request = "POST /submit HTTP/1.1\r\n\
                           Host: example.com\r\n\
                           User-Agent: TestAgent/2.0\r\n\
                           Accept-Encoding: br\r\n\
                           Content-Length: 27\r\n\r\n\
                           field1=value1&field2=value2";

        let (mut client_stream, _) = create_stream(raw_request).unwrap();
        let request = Request::new(&mut client_stream).unwrap();

        assert_eq!(request.method, Some(HttpMethod::POST));
        assert_eq!(request.url, Some(Url::new("/submit")));
        assert_eq!(request.http_version, "HTTP/1.1".to_string());
        assert_eq!(request.user_agent, Some("TestAgent/2.0".to_string()));
        assert_eq!(request.accept_encoding, Some("br".to_string()));
        assert_eq!(request.content_length, Some(27));
        assert_eq!(request.request.len(), 7);
        assert_eq!(request.request[0], "POST /submit HTTP/1.1");
        assert_eq!(request.request[1], "Host: example.com");
        assert_eq!(request.request[2], "User-Agent: TestAgent/2.0");
        assert_eq!(request.request[3], "Accept-Encoding: br");
        assert_eq!(request.request[4], "Content-Length: 27");
        assert_eq!(request.request[6].contains("field1=value1&field2=value2"), true);
    }

    #[test]
    fn test_new_request_missing_optional_headers() {
        let raw_request = "DELETE /resource/123 HTTP/1.1\r\n\
                           Host: api.example.com\r\n\r\n";

        let (mut client_stream, _) = create_stream(raw_request).unwrap();
        let request = Request::new(&mut client_stream).unwrap();

        assert_eq!(request.method, Some(HttpMethod::DELETE));
        assert_eq!(request.url, Some(Url::new("/resource/123")));
        assert_eq!(request.http_version, "HTTP/1.1".to_string());
        assert_eq!(request.user_agent, None);
        assert_eq!(request.accept_encoding, None);
        assert_eq!(request.content_length, None);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0], "DELETE /resource/123 HTTP/1.1");
        assert_eq!(request.request[1], "Host: api.example.com");
    }

    #[test]
    fn test_new_request_invalid_method() {
        let raw_request = "FETCH /data HTTP/1.1\r\n\
                           Host: localhost\r\n\r\n";

        let (mut client_stream, _) = create_stream(raw_request).unwrap();
        let request = Request::new(&mut client_stream).unwrap();

        assert_eq!(request.method, None);
        assert_eq!(request.url, Some(Url::new("/data")));
        assert_eq!(request.http_version, "HTTP/1.1".to_string());
        assert_eq!(request.user_agent, None);
        assert_eq!(request.accept_encoding, None);
        assert_eq!(request.content_length, None);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0], "FETCH /data HTTP/1.1");
        assert_eq!(request.request[1], "Host: localhost");
    }

    #[test]
    fn test_new_request_empty_buffer() {
        let raw_request = "";

        let (mut client_stream, _) = create_stream(raw_request).unwrap();
        let result = Request::new(&mut client_stream);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Error: request is empty");
    }

    #[test]
    fn test_new_request_malformed_request_line() {
        let raw_request = "INVALID_REQUEST_LINE\r\n\
                           Host: localhost\r\n\r\n";

        let (mut client_stream, _) = create_stream(raw_request).unwrap();
        let request = Request::new(&mut client_stream).unwrap();

        assert_eq!(request.method, None);
        assert_eq!(request.url, None);
        assert_eq!(request.http_version, "HTTP/1.1".to_string());
        assert_eq!(request.user_agent, None);
        assert_eq!(request.accept_encoding, None);
        assert_eq!(request.content_length, None);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0], "INVALID_REQUEST_LINE");
        assert_eq!(request.request[1], "Host: localhost");
    }

    #[test]
    fn test_new_request_multiple_headers_same_type() {
        let raw_request = "GET /multi HTTP/1.1\r\n\
                           Host: localhost\r\n\
                           User-Agent: TestAgent/1.0\r\n\
                           User-Agent: TestAgent/2.0\r\n\
                           Accept-Encoding: gzip\r\n\
                           Accept-Encoding: br\r\n\
                           Content-Length: 50\r\n\r\n";

        let (mut client_stream, _) = create_stream(raw_request).unwrap();
        let request = Request::new(&mut client_stream).unwrap();

        assert_eq!(request.method, Some(HttpMethod::GET));
        assert_eq!(request.url, Some(Url::new("/multi")));
        assert_eq!(request.http_version, "HTTP/1.1".to_string());
        assert_eq!(request.user_agent, Some("TestAgent/1.0".to_string()));
        assert_eq!(request.accept_encoding, Some("gzip".to_string()));
        assert_eq!(request.content_length, Some(50));
        assert_eq!(request.request.len(), 7);
        assert_eq!(request.request[0], "GET /multi HTTP/1.1");
        assert_eq!(request.request[1], "Host: localhost");
        assert_eq!(request.request[2], "User-Agent: TestAgent/1.0");
        assert_eq!(request.request[3], "User-Agent: TestAgent/2.0");
        assert_eq!(request.request[4], "Accept-Encoding: gzip");
        assert_eq!(request.request[5], "Accept-Encoding: br");
        assert_eq!(request.request[6], "Content-Length: 50");
    }
}