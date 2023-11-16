use std::collections::HashMap;

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

pub mod util {
    use regex::Regex;
    pub fn is_header(s: &String) -> bool {
        let header_re = Regex::new(r"^\S+:.*$").unwrap();
        header_re.is_match(s)
    }
}

pub fn parse(raw: Vec<String>) -> Result<Request, String> {
    let mut it = raw.iter();
    let Some(status_line) = it.next() else {
        return Err("no status line".to_string());
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

    let headers: HashMap<String, String> = it
        .take_while(|line| util::is_header(line))
        .map(|line| line.split_once(":").unwrap())
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let body = raw[headers.len() + 1..]
        .iter()
        .fold(String::new(), |a, b| a + b + "\n");

    return Ok(Request {
        method,
        path: path.to_string(),
        headers,
        body,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input: Vec<String> = vec![
            "GET / HTTP1.0".to_string(),
            "Foo: bar".to_string(),
            "Bar: bat".to_string(),
            "Body starts here".to_string(),
            "And ends after this".to_string(),
        ];

        let req = parse(input).unwrap();
        assert_eq!(req.method, Method::GET);
        assert_eq!(req.path, "/");
        assert_eq!(req.headers.len(), 2);
        assert_eq!(req.body, "Body starts here\nAnd ends after this\n");
    }
}
