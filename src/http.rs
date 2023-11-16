mod request;

pub mod server {
    use super::request::{parse, Request};
    use std::{
        io::{self, BufReader, Write},
        net::{TcpListener, TcpStream},
    };

    fn handle_connection(mut stream: TcpStream, handler: fn(Request) -> String) {
        let mut buf_reader = BufReader::new(&mut stream);

        let Ok(parsed) = parse(&mut buf_reader) else {
            println!("Got bad request!");
            let _ = stream.write_all("HTTP/1.1 400 BAD REQUEST".as_bytes());
            return;
        };
        
        let response = handler(parsed);

        let status_line = "HTTP/1.1 200 OK";
        let length = response.len();

        let raw = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{response}");

        let _ = stream.write_all(raw.as_bytes());
    }

    pub fn start_server(request_handler: fn(Request) -> String) -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:7878")?;

        for stream in listener.incoming() {
            let stream = stream.unwrap(); // TODO?
            handle_connection(stream, request_handler);
        }

        return Ok(());
    }
}
