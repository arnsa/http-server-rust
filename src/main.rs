use std::{env, fs};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

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
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let request = String::from_utf8_lossy(&buffer);
    let request = request.split("\r\n").collect::<Vec<&str>>();

    if request.len() == 0 {
        return;
    }

    let request_line = request[0];

    let response_line = match request_line {
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        "GET /user-agent HTTP/1.1" => {
            let user_agent = request.iter().find(|line| line.contains("User-Agent:"));

            if user_agent.is_none() {
                return;
            }

            let response = user_agent.unwrap().replace("User-Agent: ", "");
            let length = response.len();

            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{response}")
        },
        line if line.starts_with("GET /echo/") && line.ends_with("HTTP/1.1") => {
            let echo_str = parse_file_name_from_url(&line, "GET /echo/");
            let length = echo_str.len();

            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{echo_str}")
        },
        line if line.starts_with("GET /files/") && line.ends_with("HTTP/1.1") => {
            let file_name = parse_file_name_from_url(&line, "GET /files/");
            let directory = parse_directory_from_args();
            let file_contents = fs::read_to_string(format!("{}{}", directory, file_name));

            match file_contents {
                Ok(contents) => {
                    let length = contents.len();

                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {length}\r\n\r\n{contents}")
                },
                Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        }
        line if line.starts_with("POST /files/") && line.ends_with("HTTP/1.1") => {
            let file_name = parse_file_name_from_url(&line, "POST /files/");
            let directory = parse_directory_from_args();
            let content_length = request.iter().find(|x| x.contains("Content-Length"));

            if content_length.is_none() {
                "HTTP/1.1 500 An Error Ocurred\r\n\r\n".to_string()
            } else {
                let content_length = content_length.unwrap().split("Content-Length: ").nth(1).unwrap().parse::<usize>().unwrap();
                let body = &request[request.len() - 1][0..content_length];

                if let Err(_) = fs::write(format!("{}{}", directory, file_name), body) {
                    "HTTP/1.1 500 An Error Ocurred\r\n\r\n".to_string()
                } else {
                    "HTTP/1.1 201 Created\r\n\r\n".to_string()
                }
            }
        }
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    };

    stream.write_all(response_line.as_bytes()).unwrap();
}

fn parse_directory_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let mut directory = String::from("./");
    let directory_arg_idx = args.iter().position(|arg| arg == "directory" || arg == "--directory");

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