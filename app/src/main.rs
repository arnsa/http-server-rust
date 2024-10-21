mod router;
mod utils;

use std::net::TcpListener;

use server::thread_pool::ThreadPool;
use utils::handle_tcp_connection;

fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                pool.execute(|| {
                    let _ = handle_tcp_connection(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
