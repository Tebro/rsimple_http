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

fn get_content_length(headers: &HashMap<String, String>) -> Result<usize, String> {
    let Some(content_length) = headers.get("Content-Length") else {
        // TODO: bad request
        return Err("Content-Length missing".to_string());
    };
    let Ok(size) = content_length.parse::<usize>() else {
        // TODO: bad request
        return Err("Content-Length not integer".to_string());
    };
    Ok(size) 
}

pub fn parse(buf: &mut BufReader<&mut TcpStream>) -> Result<Request, String> {
    // TODO could this be done with a lines() iteration with take_while line > 2 and still get the body
    // after that.
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

    let size = get_content_length(&headers)?;

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
