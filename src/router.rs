use anyhow::{self, Context};
use flate2::{write::GzEncoder, Compression};
use std::collections::HashMap;
use std::{fs, io::Write};

use crate::http::{HttpCode, HttpHeader, HttpMethod};
use crate::request::Request;
use crate::response::Response;
use crate::utils::parse_directory_from_args;

enum Paths {
    Root,
    UserAgent,
    Echo,
    Files,
}

impl Paths {
    pub fn as_str(&self) -> &str {
        match self {
            &Paths::Root => "/",
            &Paths::UserAgent => "/user-agent",
            &Paths::Echo => "/echo/:str",
            &Paths::Files => "/files/:file_name",
        }
    }
}

type RouteReturn = Result<(String, Option<Vec<u8>>), anyhow::Error>;

struct Route {
    path: String,
    handler: fn(&Request, HashMap<String, String>) -> RouteReturn,
}

fn handle_get_root(request: &Request, _: HashMap<String, String>) -> RouteReturn {
    Ok((
        Response {
            status_code: HttpCode::Ok,
            status_text: HttpCode::Ok.to_string(),
            http_version: request.http_version.to_string(),
            body: None,
            headers: None,
        }
        .to_string(),
        None,
    ))
}

fn handle_get_user_agent(request: &Request, _: HashMap<String, String>) -> RouteReturn {
    let user_agent = &request.user_agent;
    let length = user_agent.as_ref().map(|ua| ua.len()).unwrap_or(0);
    let headers = Some(Vec::from([
        HttpHeader::ContentType("text/plain".to_string()),
        HttpHeader::ContentLength(length),
    ]));

    Ok((
        Response {
            status_code: HttpCode::Ok,
            status_text: HttpCode::Ok.to_string(),
            http_version: request.http_version.to_string(),
            headers,
            body: user_agent.to_owned(),
        }
        .to_string(),
        None,
    ))
}

fn handle_get_echo(request: &Request, params: HashMap<String, String>) -> RouteReturn {
    let echo_str = params
        .get("str")
        .context("Missing 'str' parameter in request parameters")?
        .to_string();

    match &request.accept_encoding {
        Some(encoding) => {
            if encoding.contains("gzip") {
                let mut encoder = GzEncoder::new(vec![], Compression::default());
                let _ = encoder.write_all(&echo_str.as_bytes());
                let compressed_data = encoder.finish()?;

                let headers = Some(Vec::from([
                    HttpHeader::ContentType("text/plain".to_string()),
                    HttpHeader::ContentEncoding("gzip".to_string()),
                    HttpHeader::ContentLength(compressed_data.len()),
                ]));

                return Ok((
                    Response {
                        status_code: HttpCode::Ok,
                        status_text: HttpCode::Ok.to_string(),
                        http_version: request.http_version.to_string(),
                        headers,
                        body: None,
                    }
                    .to_string(),
                    Some(compressed_data),
                ));
            } else {
                let length = echo_str.len();
                let headers = Some(Vec::from([
                    HttpHeader::ContentType("text/plain".to_string()),
                    HttpHeader::ContentLength(length),
                ]));

                return Ok((
                    Response {
                        status_code: HttpCode::Ok,
                        status_text: HttpCode::Ok.to_string(),
                        http_version: request.http_version.to_string(),
                        headers,
                        body: Some(echo_str.to_string()),
                    }
                    .to_string(),
                    None,
                ));
            }
        }
        None => {
            let length = echo_str.len();
            let headers = Some(Vec::from([
                HttpHeader::ContentType("text/plain".to_string()),
                HttpHeader::ContentEncoding("gzip".to_string()),
                HttpHeader::ContentLength(length),
            ]));

            return Ok((
                Response {
                    status_code: HttpCode::Ok,
                    status_text: HttpCode::Ok.to_string(),
                    http_version: request.http_version.to_string(),
                    headers,
                    body: Some(echo_str.to_string()),
                }
                .to_string(),
                None,
            ));
        }
    }
}

fn handle_get_files(request: &Request, params: HashMap<String, String>) -> RouteReturn {
    let file_name = params
        .get("file_name")
        .context("Missing file_name parameter")?
        .to_string();
    let directory = parse_directory_from_args();
    let file_contents = fs::read_to_string(format!("{}{}", directory, file_name));

    match file_contents {
        Ok(contents) => {
            let length = contents.len();
            let headers = Some(Vec::from([
                HttpHeader::ContentType("application/octet-stream".to_string()),
                HttpHeader::ContentLength(length),
            ]));

            return Ok((
                Response {
                    status_code: HttpCode::Ok,
                    status_text: HttpCode::Ok.to_string(),
                    http_version: request.http_version.to_string(),
                    headers,
                    body: Some(contents),
                }
                .to_string(),
                None,
            ));
        }
        Err(_) => {
            return Ok((
                Response {
                    status_code: HttpCode::NotFound,
                    status_text: HttpCode::NotFound.to_string(),
                    http_version: request.http_version.to_string(),
                    headers: None,
                    body: None,
                }
                .to_string(),
                None,
            ))
        }
    }
}

fn handle_post_files(request: &Request, params: HashMap<String, String>) -> RouteReturn {
    let file_name = params
        .get("file_name")
        .context("Missing file_name parameter")?
        .to_string();
    let directory = parse_directory_from_args();

    if let Some(cl) = request.content_length {
        let body = &request.request[request.request.len() - 1][0..cl];

        if let Err(_) = fs::write(format!("{}{}", directory, file_name), body) {
            return Ok((
                Response {
                    status_code: HttpCode::InternalServerError,
                    status_text: HttpCode::InternalServerError.to_string(),
                    http_version: request.http_version.to_string(),
                    headers: None,
                    body: None,
                }
                .to_string(),
                None,
            ));
        } else {
            return Ok((
                Response {
                    status_code: HttpCode::Created,
                    status_text: HttpCode::Created.to_string(),
                    http_version: request.http_version.to_string(),
                    headers: None,
                    body: None,
                }
                .to_string(),
                None,
            ));
        }
    } else {
        return Ok((
            Response {
                status_code: HttpCode::InternalServerError,
                status_text: HttpCode::InternalServerError.to_string(),
                http_version: request.http_version.to_string(),
                headers: None,
                body: None,
            }
            .to_string(),
            None,
        ));
    }
}

pub fn handle_route(request: &Request) -> RouteReturn {
    let method = request.method.as_ref().context("Failed to get HTTP method")?;

    if let Some(url) = &request.url {
        match &method {
            HttpMethod::GET => {
                let routes = Vec::from([
                    Route {
                        path: Paths::Root.as_str().to_string(),
                        handler: handle_get_root,
                    },
                    Route {
                        path: Paths::UserAgent.as_str().to_string(),
                        handler: handle_get_user_agent,
                    },
                    Route {
                        path: Paths::Echo.as_str().to_string(),
                        handler: handle_get_echo,
                    },
                    Route {
                        path: Paths::Files.as_str().to_string(),
                        handler: handle_get_files,
                    },
                ]);

                for route in routes {
                    if let Some(params) = url.match_path(&route.path) {
                        return (route.handler)(request, params);
                    }
                }
            }
            HttpMethod::POST => {
                let routes = Vec::from([Route {
                    path: Paths::Files.as_str().to_string(),
                    handler: handle_post_files,
                }]);

                for route in routes {
                    if let Some(params) = url.match_path(&route.path) {
                        return (route.handler)(request, params);
                    }
                }
            }
            _ => {
                return Ok((
                    Response {
                        status_code: HttpCode::NotFound,
                        status_text: HttpCode::NotFound.to_string(),
                        http_version: request.http_version.to_string(),
                        headers: None,
                        body: None,
                    }
                    .to_string(),
                    None,
                ))
            }
        }
    }

    return Ok((
        Response {
            status_code: HttpCode::NotFound,
            status_text: HttpCode::NotFound.to_string(),
            http_version: request.http_version.to_string(),
            headers: None,
            body: None,
        }
        .to_string(),
        None,
    ));
}
