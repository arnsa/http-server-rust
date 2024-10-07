use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let buf_reader = BufReader::new(&mut _stream);
                let request_line = buf_reader.lines().next();
                let response_line = match request_line {
                    Some(line) => {
                        match line.unwrap().as_str() {
                            "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\n",
                            _ => "HTTP/1.1 404 Not Found\r\n\r\n"
                        }
                    },
                    None => "HTTP/1.1 404 Not Found\r\n\r\n",
                };

                _stream.write_all(response_line.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
