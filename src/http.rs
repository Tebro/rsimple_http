pub mod request;
pub mod response;

use request::{parse, Request};
use response::Response;
use std::{
    io::{self, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_connection(mut stream: TcpStream, handler: &fn(Request) -> Response) {
    let mut buf_reader = BufReader::new(&mut stream);

    let Ok(parsed) = parse(&mut buf_reader) else {
        println!("Got bad request!");
        // TODO match different errors and handle properly eg. missing Content-Length
        let _ = stream.write_all("HTTP/1.1 400 BAD REQUEST".as_bytes());
        return;
    };

    let response = handler(parsed);
    let raw = response.to_string();
    let _ = stream.write_all(raw.as_bytes());
}

/// Start a TCP listener on `address`
/// Connections to the listener will have their messages parsed as HTTP and converted to Request
///
/// The request_handler should handle all kinds of routing etc. based on the request.
pub fn start_server(address: &str, request_handler: fn(Request) -> Response) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            handle_connection(stream, &request_handler);
        });
    }

    return Ok(());
}
