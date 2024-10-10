use std::{
  io::Read,
  net::TcpStream,
  str::{self, FromStr},
};
use crate::http::HttpMethod;

pub struct Request {
  pub request: Vec<String>,
  pub user_agent: Option<String>,
  pub accept_encoding: Option<String>,
  pub content_length: Option<usize>,
  pub method: Option<HttpMethod>,
  pub url: Option<String>,
  pub http_version: String,
}

impl Request {
  pub fn new(stream: &mut TcpStream) -> Result<Request, &'static str> {
      let mut buffer = [0; 1024];
      let _ = stream.read(&mut buffer);

      let request = Self::get_request(&buffer)?;
      let request_line = request[0].to_string();

      let user_agent = Self::get_user_agent_header(&request);
      let accept_encoding = Self::get_accept_encoding(&request);
      let content_length = Self::get_content_length(&request);
      let method = Self::get_method(&request_line);
      let url = Self::get_url(&request_line);
      let http_version = Self::get_http_version(&request_line);

      Ok(Request {
          request,
          user_agent,
          accept_encoding,
          content_length,
          method,
          url,
          http_version
      })
  }

  fn get_request(buffer: &[u8]) -> Result<Vec<String>, &'static str> {
      let request = String::from_utf8_lossy(buffer);
      let request = request.split("\r\n").map(|s| s.to_string()).collect::<Vec<String>>();

      if request.len() == 0 {
          Err("Error: request is empty")
      } else {
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
      request_line.split(" ").nth(0).and_then(|s| HttpMethod::from_str(s).ok())
  }

  fn get_url(request_line: &String) -> Option<String> {
      request_line.split(" ").nth(1).and_then(|s| Some(s.to_string()))
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
