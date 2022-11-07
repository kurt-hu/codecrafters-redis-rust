#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::io::Write;
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    // match listener.accept() {
    //     Ok((stream, addr)) => {
    //         handle_accept_client(stream, addr);
    //     },
    //     Err(e) => println!("couldn't accept client: {:?}", e),
    // }
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    }
}

// fn handle_accept_client(stream:TcpStream, addr: std::net::SocketAddr) {
//     println!("accepted new client: {:?}", addr);
// }

fn handle_connection(mut stream: TcpStream) {
    let response: &str = "+PONG\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}