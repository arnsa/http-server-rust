use std::{env, str};

pub fn parse_directory_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let mut directory = String::from("./");
    let directory_arg_idx = args
        .iter()
        .position(|arg| arg == "directory" || arg == "--directory");

    if directory_arg_idx.is_some() && directory_arg_idx.unwrap() < args.len() {
        directory = String::from(&args[directory_arg_idx.unwrap() + 1]);
    }

    directory
}

pub fn parse_file_name_from_url(url: &str, prefix: &str) -> String {
    String::from(&url[prefix.len()..url.len()])
}
