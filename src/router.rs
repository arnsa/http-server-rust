use flate2::{write::GzEncoder, Compression};
use std::{fs, io::Write};

use crate::http::{HttpCode, HttpHeader, HttpMethod};
use crate::request::Request;
use crate::response::Response;
use crate::utils::{
    parse_file_name_from_url,
    parse_directory_from_args
};

pub fn handle_route(request: &Request) -> (String, Option<Vec<u8>>) {
    match &request.method.as_ref().unwrap() {
        HttpMethod::GET => match &request.url.as_ref().unwrap()[..] {
            "/" => (
                Response {
                    status_code: HttpCode::Ok,
                    status_text: HttpCode::Ok.to_string(),
                    http_version: request.http_version.to_string(),
                    body: None,
                    headers: None,
                }
                .to_string(),
                None,
            ),
            "/user-agent" => {
                let user_agent = &request.user_agent;
                let length = user_agent.as_ref().map(|ua| ua.len()).unwrap_or(0);
                let headers = Some(Vec::from([
                    HttpHeader::ContentType("text/plain".to_string()),
                    HttpHeader::ContentLength(length),
                ]));

                (
                    Response {
                        status_code: HttpCode::Ok,
                        status_text: HttpCode::Ok.to_string(),
                        http_version: request.http_version.to_string(),
                        headers,
                        body: user_agent.to_owned(),
                    }
                    .to_string(),
                    None,
                )
            }
            url if url.starts_with("/echo/") => {
                let echo_str = parse_file_name_from_url(&url, "/echo/");

                match &request.accept_encoding {
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

                            (
                                Response {
                                    status_code: HttpCode::Ok,
                                    status_text: HttpCode::Ok.to_string(),
                                    http_version: request.http_version.to_string(),
                                    headers,
                                    body: None,
                                }
                                .to_string(),
                                Some(compressed_data),
                            )
                        } else {
                            let length = echo_str.len();
                            let headers = Some(Vec::from([
                                HttpHeader::ContentType("text/plain".to_string()),
                                HttpHeader::ContentLength(length),
                            ]));

                            (
                                Response {
                                    status_code: HttpCode::Ok,
                                    status_text: HttpCode::Ok.to_string(),
                                    http_version: request.http_version.to_string(),
                                    headers,
                                    body: Some(echo_str),
                                }
                                .to_string(),
                                None,
                            )
                        }
                    }
                    None => {
                        let length = echo_str.len();
                        let headers = Some(Vec::from([
                            HttpHeader::ContentType("text/plain".to_string()),
                            HttpHeader::ContentEncoding("gzip".to_string()),
                            HttpHeader::ContentLength(length),
                        ]));

                        (
                            Response {
                                status_code: HttpCode::Ok,
                                status_text: HttpCode::Ok.to_string(),
                                http_version: request.http_version.to_string(),
                                headers,
                                body: Some(echo_str),
                            }
                            .to_string(),
                            None,
                        )
                    }
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

                        (
                            Response {
                                status_code: HttpCode::Ok,
                                status_text: HttpCode::Ok.to_string(),
                                http_version: request.http_version.to_string(),
                                headers,
                                body: Some(contents),
                            }
                            .to_string(),
                            None,
                        )
                    }
                    Err(_) => (
                        Response {
                            status_code: HttpCode::NotFound,
                            status_text: HttpCode::NotFound.to_string(),
                            http_version: request.http_version.to_string(),
                            headers: None,
                            body: None,
                        }
                        .to_string(),
                        None,
                    ),
                }
            }
            _ => (
                Response {
                    status_code: HttpCode::NotFound,
                    status_text: HttpCode::NotFound.to_string(),
                    http_version: request.http_version.to_string(),
                    headers: None,
                    body: None,
                }
                .to_string(),
                None,
            ),
        },
        HttpMethod::POST => match &request.url.as_ref().unwrap()[..] {
            url if url.starts_with("/files/") => {
                let file_name = parse_file_name_from_url(&url, "/files/");
                let directory = parse_directory_from_args();

                if let Some(cl) = request.content_length {
                    let body = &request.request[request.request.len() - 1][0..cl];

                    if let Err(_) = fs::write(format!("{}{}", directory, file_name), body) {
                        (
                            Response {
                                status_code: HttpCode::InternalServerError,
                                status_text: HttpCode::InternalServerError.to_string(),
                                http_version: request.http_version.to_string(),
                                headers: None,
                                body: None,
                            }
                            .to_string(),
                            None,
                        )
                    } else {
                        (
                            Response {
                                status_code: HttpCode::Created,
                                status_text: HttpCode::Created.to_string(),
                                http_version: request.http_version.to_string(),
                                headers: None,
                                body: None,
                            }
                            .to_string(),
                            None,
                        )
                    }
                } else {
                    (
                        Response {
                            status_code: HttpCode::InternalServerError,
                            status_text: HttpCode::InternalServerError.to_string(),
                            http_version: request.http_version.to_string(),
                            headers: None,
                            body: None,
                        }
                        .to_string(),
                        None,
                    )
                }
            }
            _ => (
                Response {
                    status_code: HttpCode::NotFound,
                    status_text: HttpCode::NotFound.to_string(),
                    http_version: request.http_version.to_string(),
                    headers: None,
                    body: None,
                }
                .to_string(),
                None,
            ),
        },
        _ => (
            Response {
                status_code: HttpCode::NotFound,
                status_text: HttpCode::NotFound.to_string(),
                http_version: request.http_version.to_string(),
                headers: None,
                body: None,
            }
            .to_string(),
            None,
        ),
    }
}
