use std::env;

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
