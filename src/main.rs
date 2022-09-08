use std::net::TCPListener;
use std::net::TCPStream;


fn main() {
    let listener = TCPListener::bind(addr: "127.0.0.1:7878").unwrap();

    for stream: Result<TCPStream, Error> in listener.incoming() {
        let stream: TCPStream = stream.unwrap();

        println!("Connection established!");
    }
}
