pub mod request;
pub mod response;

pub mod server {
    use super::{
        request::{parse, Request},
        response::Response,
    };
    use std::{
        io::{self, BufReader, Write},
        net::{TcpListener, TcpStream},
        thread,
    };

    fn handle_connection(mut stream: TcpStream, handler: &fn(Request) -> Response) {
        let mut buf_reader = BufReader::new(&mut stream);

        let Ok(parsed) = parse(&mut buf_reader) else {
            println!("Got bad request!");
            let _ = stream.write_all("HTTP/1.1 400 BAD REQUEST".as_bytes()); // TODO
            return;
        };

        let response = handler(parsed);
        let raw = response.to_string();
        let _ = stream.write_all(raw.as_bytes());
    }

    pub fn start_server(address: &str, request_handler: fn(Request) -> Response) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            let stream = stream.unwrap(); // TODO?
            thread::spawn(move || {
                handle_connection(stream, &request_handler);
            });
        }

        return Ok(());
    }
}
