use std::{io::Write, net::TcpStream};

use anyhow::{Context, Result};
use server::{http::code::HttpCode, request::Request, response::Response};

use crate::router;

pub fn handle_tcp_connection(mut stream: TcpStream) -> Result<()> {
    let request = Request::new(&mut stream).context("Failed to parse the request")?;
    let (response_line, data) = router::handle_route(&request).unwrap_or((
        Response {
            status_code: HttpCode::InternalServerError,
            status_text: HttpCode::InternalServerError.to_string(),
            http_version: request.http_version.to_string(),
            body: None,
            headers: None,
        }
        .to_string(),
        None,
    ));

    stream
        .write_all(response_line.as_bytes())
        .context("Failed to write response line to stream")?;

    if let Some(d) = data {
        stream
            .write_all(&d)
            .context("Failed to write response line to stream")?;
    }

    Ok(())
}
