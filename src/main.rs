use flate2::{write::GzEncoder, Compression};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    str,
    thread,
    {env, fs},
};

use codecrafters_http_server::Request;

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

    let (response_line, data) = match &request.request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n\r\n".to_string(), None),
        "GET /user-agent HTTP/1.1" => {
            if request.user_agent.is_none() {
                return;
            }

            let user_agent = request.user_agent.unwrap();
            let length = user_agent.len();

            (format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{user_agent}"), None)
        }
        line if line.starts_with("GET /echo/") && line.ends_with("HTTP/1.1") => {
            let echo_str = parse_file_name_from_url(&line, "GET /echo/");
            let mut length = echo_str.len();
            let mut content_encoding = String::from("\r\n");

            match request.accept_encoding {
                Some(encoding) => {
                    if encoding.contains("gzip") {
                        let mut encoder = GzEncoder::new(vec![], Compression::default());
                        let _ = encoder.write_all(&echo_str.as_bytes());
                        let compressed_data = encoder.finish().unwrap();

                        length = compressed_data.len();
                        content_encoding = format!("\r\nContent-Encoding: gzip\r\n");

                        (format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain{content_encoding}Content-Length: {length}\r\n\r\n"), Some(compressed_data))
                    } else {
                        (format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain{content_encoding}Content-Length: {length}\r\n\r\n{echo_str}"), None)
                    }
                }
                None => (format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain{content_encoding}Content-Length: {length}\r\n\r\n{echo_str}"), None),
            }
        }
        line if line.starts_with("GET /files/") && line.ends_with("HTTP/1.1") => {
            let file_name = parse_file_name_from_url(&line, "GET /files/");
            let directory = parse_directory_from_args();
            let file_contents = fs::read_to_string(format!("{}{}", directory, file_name));

            match file_contents {
                Ok(contents) => {
                    let length = contents.len();

                    (
                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {length}\r\n\r\n{contents}"),
                        None
                    )
                }
                Err(_) => ("HTTP/1.1 404 Not Found\r\n\r\n".to_string(), None),
            }
        }
        line if line.starts_with("POST /files/") && line.ends_with("HTTP/1.1") => {
            let file_name = parse_file_name_from_url(&line, "POST /files/");
            let directory = parse_directory_from_args();

            if let Some(cl) = request.content_length {
                let body = &request.request[request.request.len() - 1][0..cl];

                if let Err(_) = fs::write(format!("{}{}", directory, file_name), body) {
                    ("HTTP/1.1 500 An Error Ocurred\r\n\r\n".to_string(), None)
                } else {
                    ("HTTP/1.1 201 Created\r\n\r\n".to_string(), None)
                }
            } else {
                ("HTTP/1.1 500 An Error Ocurred\r\n\r\n".to_string(), None)
            }
        }
        _ => ("HTTP/1.1 404 Not Found\r\n\r\n".to_string(), None),
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

fn parse_file_name_from_url(line: &str, prefix: &str) -> String {
    let suffix = "HTTP/1.1";
    let start = prefix.len();
    let end = line.len() - suffix.len() - 1;

    String::from(&line[start..end])
}
