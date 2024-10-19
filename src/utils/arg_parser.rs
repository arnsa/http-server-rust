use std::env;

pub fn parse_directory_from_args() -> String {
    let args: Vec<String> = env::args().collect();

    args
        .windows(2)
        .find(|window| window[0] == "directory" || window[0] == "--directory")
        .map(|window| window[1].to_string())
        .unwrap_or_else(|| "./".to_string())
}
