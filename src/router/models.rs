use std::collections::HashMap;

use crate::request::Request;

pub enum Paths {
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

pub type RouteReturn = Result<(String, Option<Vec<u8>>), anyhow::Error>;

pub struct Route {
    pub path: String,
    pub handler: fn(&Request, HashMap<String, String>) -> RouteReturn,
}
