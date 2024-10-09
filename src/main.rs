use std::{env, fs};
use std::io::{BufRead, BufReader, Write};
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
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if request.len() == 0 {
        return;
    }

    let request_line = &request[0];

    let response_line = match request_line.as_str() {
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
            let prefix = "GET /echo/";
            let suffix = "HTTP/1.1";
            let start = prefix.len();
            let end = line.len() - suffix.len() - 1;
            let echo_str = &line[start..end];
            let length = echo_str.len();

            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{echo_str}")
        },
        line if line.starts_with("GET /files/") && line.ends_with("HTTP/1.1") => {
            let prefix = "GET /files/";
            let suffix = "HTTP/1.1";
            let start = prefix.len();
            let end = line.len() - suffix.len() - 1;
            let file_name = &line[start..end];

            let args: Vec<String> = env::args().collect();
            let mut directory = ".";
            let directory_arg_idx = args.iter().position(|arg| arg == "directory");

            if directory_arg_idx.is_some() && directory_arg_idx.unwrap() < args.len() {
                directory = &args[directory_arg_idx.unwrap() + 1];
            }

            let file_contents = fs::read_to_string(format!("{}/{}", directory, file_name));

            match file_contents {
                Ok(contents) => {
                    let length = contents.len();

                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{contents}")
                },
                Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        }
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    };

    stream.write_all(response_line.as_bytes()).unwrap();
}