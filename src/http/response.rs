use std::{
    collections::HashMap,
    fmt::{self, Display},
};

pub enum ResponseCode {
    OK,
    Created,
    Accepted,
    MovedPermanently,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotSupported,
    InternalServerError,
}

impl Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            ResponseCode::OK => "200 OK",
            ResponseCode::Created => "201 Created",
            ResponseCode::Accepted => "202 Accepted",
            ResponseCode::MovedPermanently => "301 Moved Permanently",
            ResponseCode::NotModified => "304 Not Modified",
            ResponseCode::BadRequest => "400 Bad Request",
            ResponseCode::Unauthorized => "401 Unauthorized",
            ResponseCode::Forbidden => "403 Forbidden",
            ResponseCode::NotFound => "404 Not Found",
            ResponseCode::MethodNotSupported => "405 Method Not Supported",
            ResponseCode::InternalServerError => "500 Internal Server Error",
        };
        write!(f, "{}", val)
    }
}

pub struct Response {
    code: ResponseCode,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    /// Helper to create a 200 OK response
    pub fn ok(body: String) -> Self {
        Self::with_code(ResponseCode::OK, body)
    }

    /// Creates a Response for the given code and body
    pub fn with_code(code: ResponseCode, body: String) -> Self {
        return Self {
            code,
            headers: HashMap::new(),
            body,
        };
    }

    /// Set the headers on the response
    ///
    /// Note that Content-Length will be set automatically if using the rsimple_http server
    pub fn headers(&mut self, headers: Vec<(String, String)>) {
        self.headers = headers
            .iter()
            .map(|(k, v)| (k.clone(), v.clone())) //TODO: there has to be a better way
            .collect();
    }

    fn length(&self) -> usize {
        self.body.len()
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_line = format!("HTTP/1.1 {}", self.code);
        let length = self.length();
        if self.headers.len() > 0 {
            let headers = self
                .headers
                .iter()
                .map(|(k, v)| format!("{k}: {v}"))
                .collect::<Vec<String>>()
                .join("\r\n");

            write!(
                f,
                "{status_line}\r\n{headers}\r\nContent-Length: {length}\r\n\r\n{}",
                self.body
            )?;
            return Ok(());
        }
        write!(
            f,
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{}",
            self.body
        )?;

        return Ok(());
    }
}
