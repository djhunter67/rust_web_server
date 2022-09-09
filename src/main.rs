use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
// use std::thread;
// use std::time::Duration;
mod lib;
use lib::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
    
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();

    let get: [&[u8; 23]; 3] = [
        b"GET / HTTP/1.1\r\n       ",
        b"GET /contact HTTP/1.1\r\n",
        b"GET /about HTTP/1.1\r\n  ",
    ];

    for request in get.iter() {
        if buffer.starts_with(&request[..]) {
            let (status_line, filename) = if request == &get[0] {
                ("HTTP/1.1 200 OK", "templates/base.html")
            } else if request == &get[1] {
                ("HTTP/1.1 200 OK", "templates/contact.html")
            } else if request == &get[2] {
                ("HTTP/1.1 200 OK", "templates/about.html")
            }
            else {
                ("HTTP/1.1 404 NOT FOUND", "templates/404.html")
            };


    // let (status_line, filename) = 
    //     if buffer.starts_with(get[1]){       
    //         ("HTTP/1.1 200 OK", "templates/base.html")
    //     }
    //      else {
    //         ("HTTP/1.1 404 NOT FOUND", "templates/404.html")
    //     };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContents-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}
}
}
