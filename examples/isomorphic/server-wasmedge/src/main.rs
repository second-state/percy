use std::io::Write;
use wasmedge_wasi_socket::{Shutdown, TcpListener};

mod handler;
mod mime;
mod response;

fn main() {
    let server = TcpListener::bind("127.0.0.1:3000", false).unwrap();
    println!("Server listening on 127.0.0.1:3000");

    // Simple single thread HTTP server
    // For server with Pool support, see https://github.com/second-state/wasmedge_wasi_socket/tree/main/examples/poll_http_server
    loop {
        let (mut stream, addr) = server.accept(0).unwrap();
        println!("Accepted connection from {}", addr);
        match handler::handle_req(&mut stream, addr) {
            Ok((res, binary)) => {
                let res: String = res.into();
                let bytes = res.as_bytes();
                stream.write_all(bytes).unwrap();
                if let Some(binary) = binary {
                    stream.write_all(&binary).unwrap();
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        };
        stream.shutdown(Shutdown::Both).unwrap();
    }
}
