use std::{collections::HashMap, io::BufRead};

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    PATCH,
    DELETE,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

fn get_content_length(headers: &HashMap<String, String>) -> Option<usize> {
    let content_length = headers.get("Content-Length")?;
    let Ok(size) = content_length.parse::<usize>() else {
        return None;
    };
    Some(size)
}

/// Reads and parses a HTTP request
pub fn parse(buf: &mut impl BufRead) -> Result<Request, String> {
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
        Some("HEAD") => Method::HEAD,
        Some("POST") => Method::POST,
        Some("PUT") => Method::PUT,
        Some("PATCH") => Method::PATCH,
        Some("DELETE") => Method::DELETE,
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

    let size = match get_content_length(&headers) {
        Some(size) => size,
        None => 0, // TODO: Bad Request for POST?
    };

    let body = if size > 0 {
        let mut body_buf = vec![0; size];
        let _n = buf.read_exact(&mut body_buf).unwrap();
        String::from_utf8(body_buf).unwrap()
    } else {
        String::new()
    };

    return Ok(Request {
        method,
        path: path.to_string(),
        headers,
        body,
    });
}
