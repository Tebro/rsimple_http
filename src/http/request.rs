use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub fn parse(buf: &mut BufReader<&mut TcpStream>) -> Result<Request, String> {
    let mut status_line = String::new();
    let Ok(_n) = buf.read_line(&mut status_line) else {
        return Err("could not read status line".to_string());
    };
    let mut split = status_line.split_whitespace();

    let method = split.next();
    let method = match method {
        Some("GET") => Method::GET,
        Some("POST") => Method::POST,
        _ => return Err("unsupported method".to_string()),
    };

    let Some(path) = split.next() else {
        return Err("invalid format".to_string());
    };

    let mut header_pairs = vec![];

    loop {
        let mut line = String::new();
        let Ok(n) = buf.read_line(&mut line) else {
            return Err("could not read header line".to_string());
        };
        if n < 3 {
            break; // end of headers
        }
        header_pairs.push(line);
    }
    let headers: HashMap<String, String> = header_pairs
        .iter()
        .map(|line| line.split_once(":").unwrap())
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();

    let content_length = headers.get("Content-Length").unwrap(); // TODO: bad request
    let size = content_length.parse::<usize>().unwrap();

    let mut body_buf = vec![0; size];
    let _n = buf.read_exact(&mut body_buf).unwrap();
    let body = String::from_utf8(body_buf).unwrap();

    return Ok(Request {
        method,
        path: path.to_string(),
        headers,
        body,
    });
}
