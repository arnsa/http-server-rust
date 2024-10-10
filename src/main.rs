mod http;
mod request;
mod response;

use flate2::{write::GzEncoder, Compression};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    str,
    thread,
    {env, fs},
};

use response::Response;
use request::Request;
use http::{HttpCode, HttpHeader, Method};
fn main() {
    let mut threads = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                threads.push(thread::spawn(|| {
                    handle_connection(_stream);
                }));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    for t in threads {
        let _ = t.join();
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = Request::new(&mut stream).unwrap();

    let (response_line, data) = match &request.method.unwrap() {
        Method::GET => {
            match &request.url.unwrap()[..] {
                "/" => (Response {
                    status_code: HttpCode::Ok,
                    status_text: HttpCode::Ok.to_string(),
                    http_version: request.http_version,
                    body: None,
                    headers: None
                }.to_string(), None),
                "/user-agent" => {
                    if request.user_agent.is_none() {
                        return;
                    }

                    let user_agent = request.user_agent.unwrap();
                    let length = user_agent.len();
                    let headers = Some(Vec::from([
                        HttpHeader::ContentType("text/plain".to_string()),
                        HttpHeader::ContentLength(length)
                    ]));

                    (Response {
                        status_code: HttpCode::Ok,
                        status_text: HttpCode::Ok.to_string(),
                        http_version: request.http_version,
                        headers,
                        body: Some(user_agent),
                    }.to_string(), None)
                }
                url if url.starts_with("/echo/") => {
                    let echo_str = parse_file_name_from_url(&url, "/echo/");

                    match request.accept_encoding {
                        Some(encoding) => {
                            if encoding.contains("gzip") {
                                let mut encoder = GzEncoder::new(vec![], Compression::default());
                                let _ = encoder.write_all(&echo_str.as_bytes());
                                let compressed_data = encoder.finish().unwrap();

                                let headers = Some(Vec::from([
                                    HttpHeader::ContentType("text/plain".to_string()),
                                    HttpHeader::ContentEncoding("gzip".to_string()),
                                    HttpHeader::ContentLength(compressed_data.len()),
                                ]));

                                (Response {
                                    status_code: HttpCode::Ok,
                                    status_text: HttpCode::Ok.to_string(),
                                    http_version: request.http_version,
                                    headers,
                                    body: None,
                                }.to_string(), Some(compressed_data))
                            } else {
                                let length = echo_str.len();
                                let headers = Some(Vec::from([
                                    HttpHeader::ContentType("text/plain".to_string()),
                                    HttpHeader::ContentLength(length),
                                ]));

                                (Response {
                                    status_code: HttpCode::Ok,
                                    status_text: HttpCode::Ok.to_string(),
                                    http_version: request.http_version,
                                    headers,
                                    body: Some(echo_str),
                                }.to_string(), None)
                            }
                        }
                        None => {
                            let length = echo_str.len();
                            let headers = Some(Vec::from([
                                HttpHeader::ContentType("text/plain".to_string()),
                                HttpHeader::ContentEncoding("gzip".to_string()),
                                HttpHeader::ContentLength(length),
                            ]));

                            (Response {
                                status_code: HttpCode::Ok,
                                status_text: HttpCode::Ok.to_string(),
                                http_version: request.http_version,
                                headers,
                                body: Some(echo_str),
                            }.to_string(), None)
                        },
                    }
                }
                url if url.starts_with("/files/") => {
                    let file_name = parse_file_name_from_url(&url, "/files/");
                    let directory = parse_directory_from_args();
                    let file_contents = fs::read_to_string(format!("{}{}", directory, file_name));

                    match file_contents {
                        Ok(contents) => {
                            let length = contents.len();
                            let headers = Some(Vec::from([
                                HttpHeader::ContentType("application/octet-stream".to_string()),
                                HttpHeader::ContentLength(length),
                            ]));

                            (Response {
                                status_code: HttpCode::Ok,
                                status_text: HttpCode::Ok.to_string(),
                                http_version: request.http_version,
                                headers,
                                body: Some(contents),
                            }.to_string(), None)
                        }
                        Err(_) => (Response {
                            status_code: HttpCode::NotFound,
                            status_text: HttpCode::NotFound.to_string(),
                            http_version: request.http_version,
                            headers: None,
                            body: None,
                        }.to_string(), None),
                    }
                }
                _ => (Response {
                    status_code: HttpCode::NotFound,
                    status_text: HttpCode::NotFound.to_string(),
                    http_version: request.http_version,
                    headers: None,
                    body: None,
                }.to_string(), None),
            }
        },
        Method::POST => {
            match &request.url.unwrap()[..] {
                url if url.starts_with("/files/") => {
                    let file_name = parse_file_name_from_url(&url, "/files/");
                    let directory = parse_directory_from_args();

                    if let Some(cl) = request.content_length {
                        let body = &request.request[request.request.len() - 1][0..cl];

                        if let Err(_) = fs::write(format!("{}{}", directory, file_name), body) {
                            (Response {
                                status_code: HttpCode::InternalServerError,
                                status_text: HttpCode::InternalServerError.to_string(),
                                http_version: request.http_version,
                                headers: None,
                                body: None,
                            }.to_string(), None)
                        } else {
                            (Response {
                                status_code: HttpCode::Created,
                                status_text: HttpCode::Created.to_string(),
                                http_version: request.http_version,
                                headers: None,
                                body: None,
                            }.to_string(), None)
                        }
                    } else {
                        (Response {
                            status_code: HttpCode::InternalServerError,
                            status_text: HttpCode::InternalServerError.to_string(),
                            http_version: request.http_version,
                            headers: None,
                            body: None,
                        }.to_string(), None)
                    }
                }
                _ => (Response {
                    status_code: HttpCode::NotFound,
                    status_text: HttpCode::NotFound.to_string(),
                    http_version: request.http_version,
                    headers: None,
                    body: None,
                }.to_string(), None),
            }
        }
        _ => (Response {
            status_code: HttpCode::NotFound,
            status_text: HttpCode::NotFound.to_string(),
            http_version: request.http_version,
            headers: None,
            body: None,
        }.to_string(), None),
    };

    stream.write_all(response_line.as_bytes()).unwrap();

    if let Some(d) = data {
        stream.write_all(&d).unwrap();
    }
}

fn parse_directory_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let mut directory = String::from("./");
    let directory_arg_idx = args
        .iter()
        .position(|arg| arg == "directory" || arg == "--directory");

    if directory_arg_idx.is_some() && directory_arg_idx.unwrap() < args.len() {
        directory = String::from(&args[directory_arg_idx.unwrap() + 1]);
    }

    directory
}

fn parse_file_name_from_url(url: &str, prefix: &str) -> String {
    String::from(&url[prefix.len()..url.len()])
}
