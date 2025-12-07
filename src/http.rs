pub mod request;

use http::Response;
use request::{parse, Request};
use std::{
    io::{self, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn response_to_string(response: Response<String>) -> String {
    let status_line = format!("HTTP/1.1 {}", response.status().as_str());

    let length = response.body().len();
    let has_headers = !response.headers().is_empty();
    let mut body = response.body().to_owned();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| {
            format!("{}: {}", k.as_str(), v.to_str().unwrap_or(""))
        })
        .collect::<Vec<String>>()
        .join("\r\n");

    if body.trim().len() < 1 {
        body = response.status().canonical_reason().unwrap().to_string();
    }


    if has_headers {
        format!(
            "{status_line}\r\n{headers}\r\nContent-Length: {length}\r\n\r\n{}",
            body
        )
    } else {
        format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{}",
            body
        )
    }
}

fn handle_connection(mut stream: TcpStream, handler: &fn(Request) -> Response<String>) {
    let mut buf_reader = BufReader::new(&mut stream);

    let Ok(parsed) = parse(&mut buf_reader) else {
        println!("Got bad request!");
        // TODO match different errors and handle properly eg. missing Content-Length
        let _ = stream.write_all("HTTP/1.1 400 BAD REQUEST".as_bytes());
        return;
    };

    let response = handler(parsed);
    let raw = response_to_string(response);
    let _ = stream.write_all(raw.as_bytes());
}

/// Start a TCP listener on `address`
/// Connections to the listener will have their messages parsed as HTTP and converted to Request
///
/// The request_handler should handle all kinds of routing etc. based on the request.
pub fn start_server(
    address: &str,
    request_handler: fn(Request) -> Response<String>,
) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            handle_connection(stream, &request_handler);
        });
    }

    Ok(())
}
