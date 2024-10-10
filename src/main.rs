mod http;
mod request;
mod response;
mod router;
mod utils;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

use request::Request;
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

    let (response_line, data) = router::handle_route(&request);

    stream.write_all(response_line.as_bytes()).unwrap();

    if let Some(d) = data {
        stream.write_all(&d).unwrap();
    }
}
