mod http;
mod request;
mod response;
mod router;
mod utils;
mod url;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};
use anyhow::{Context, Result};

use http::HttpCode;
use request::Request;
use response::Response;
fn main() {
    let mut threads = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                threads.push(thread::spawn(|| {
                    let _ = handle_connection(_stream);
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

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let request = Request::new(&mut stream).context("Failed to parse the request")?;
    let (response_line, data) = router::handle_route(&request).unwrap_or((
        Response {
            status_code: HttpCode::InternalServerError,
            status_text: HttpCode::InternalServerError.to_string(),
            http_version: request.http_version.to_string(),
            body: None,
            headers: None,
        }.to_string(),
        None
    ));

    stream.write_all(response_line.as_bytes()).context("Failed to write response line to stream")?;

    if let Some(d) = data {
        stream.write_all(&d).context("Failed to write response line to stream")?;
    }

    Ok(())
}
