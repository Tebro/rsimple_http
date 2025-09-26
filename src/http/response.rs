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
        Self {
            code,
            headers: HashMap::new(),
            body,
        }
    }

    /// Set the headers on the response
    ///
    /// Note that Content-Length will be set automatically if using the rsimple_http server
    pub fn headers(&mut self, headers: Vec<(String, String)>) {
        self.headers = headers.iter().cloned().collect();
    }

    fn length(&self) -> usize {
        self.body.len()
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_line = format!("HTTP/1.1 {}", self.code);
        let length = self.length();
        if !self.headers.is_empty() {
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers_method() {
        let mut response = Response::ok("test body".to_string());

        let headers = vec![
            ("Content-Type".to_string(), "text/html".to_string()),
            ("X-Custom-Header".to_string(), "custom-value".to_string()),
        ];

        response.headers(headers.clone());

        // Check that headers were set correctly
        assert_eq!(response.headers.len(), 2);
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"text/html".to_string())
        );
        assert_eq!(
            response.headers.get("X-Custom-Header"),
            Some(&"custom-value".to_string())
        );

        // Test that setting new headers replaces old ones
        let new_headers = vec![("Authorization".to_string(), "Bearer token".to_string())];

        response.headers(new_headers);

        assert_eq!(response.headers.len(), 1);
        assert_eq!(
            response.headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
        assert_eq!(response.headers.get("Content-Type"), None);
    }
}
