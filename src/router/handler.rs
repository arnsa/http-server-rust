use anyhow::Context;

use crate::{
    http::{code::HttpCode, method::HttpMethod},
    request::Request,
    response::Response,
};

use super::{
    handlers::{
        handle_get_echo, handle_get_files, handle_get_root, handle_get_user_agent,
        handle_post_files,
    },
    models::{Paths, Route, RouteReturn},
};

pub fn handle_route(request: &Request) -> RouteReturn {
    let method = request
        .method
        .as_ref()
        .context("Failed to get HTTP method")?;

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
