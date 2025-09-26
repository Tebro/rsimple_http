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

fn find_content_length_header(header_keys: Vec<String>) -> Option<String> {
    for key in &header_keys {
        if key.to_lowercase() == "content-length" {
            return Some(key.clone());
        }
    }
    return None;
}

fn get_content_length(headers: &HashMap<String, String>) -> Option<usize> {
    let content_length_header = find_content_length_header(headers.keys().cloned().collect())?;
    let content_length = headers.get(content_length_header.as_str());
    if content_length.is_none() {
        return None;
    }
    let content_length = content_length.unwrap();
    if let Ok(size) = content_length.parse::<usize>() {
        return Some(size);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_length_header_exact_match() {
        let headers = vec!["Content-Length".to_string(), "Host".to_string()];
        let result = find_content_length_header(headers);
        assert_eq!(result, Some("Content-Length".to_string()));
    }

    #[test]
    fn test_find_content_length_header_case_insensitive() {
        let headers = vec!["content-length".to_string(), "Host".to_string()];
        let result = find_content_length_header(headers);
        assert_eq!(result, Some("content-length".to_string()));
    }

    #[test]
    fn test_find_content_length_header_mixed_case() {
        let headers = vec!["Content-LENGTH".to_string(), "Host".to_string()];
        let result = find_content_length_header(headers);
        assert_eq!(result, Some("Content-LENGTH".to_string()));
    }

    #[test]
    fn test_find_content_length_header_not_found() {
        let headers = vec!["Host".to_string(), "User-Agent".to_string()];
        let result = find_content_length_header(headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_content_length_header_empty() {
        let headers = vec![];
        let result = find_content_length_header(headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_content_length_valid() {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), "42".to_string());
        let result = get_content_length(&headers);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_get_content_length_case_insensitive() {
        let mut headers = HashMap::new();
        headers.insert("content-length".to_string(), "123".to_string());
        let result = get_content_length(&headers);
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_get_content_length_zero() {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), "0".to_string());
        let result = get_content_length(&headers);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_get_content_length_invalid_number() {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), "not_a_number".to_string());
        let result = get_content_length(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_content_length_negative_number() {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), "-1".to_string());
        let result = get_content_length(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_content_length_header_not_present() {
        let headers = HashMap::new();
        let result = get_content_length(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_content_length_empty_value() {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), "".to_string());
        let result = get_content_length(&headers);
        assert_eq!(result, None);
    }
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
        .map(|line| line.split_once(':').unwrap())
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();

    let size = get_content_length(&headers).unwrap_or(0);
    // TODO: Bad Request for POST if size is missing?

    let body = if size > 0 {
        let mut body_buf = vec![0; size];
        buf.read_exact(&mut body_buf).unwrap();
        String::from_utf8(body_buf).unwrap()
    } else {
        String::new()
    };

    Ok(Request {
        method,
        path: path.to_string(),
        headers,
        body,
    })
}
