use std::collections::HashMap;

pub struct Response {
    code: usize,
    headers: HashMap<String, String>,
    body: String,
}

fn code_name(code: usize) -> Option<&'static str> {
    match code {
        200 => Some("OK"),
        201 => Some("Created"),
        202 => Some("Accepted"),
        203 => Some("Non-Authoritative Information"),
        204 => Some("No Content"),
        205 => Some("Reset Content"),
        206 => Some("Partial Content"),
        300 => Some("Multiple Choices"),
        301 => Some("Moved Permanently"),
        302 => Some("Found"),
        303 => Some("See Other"),
        304 => Some("Not Modified"),
        307 => Some("Temporary Redirect"),
        308 => Some("Permanent Redirect"),
        400 => Some("Bad Request"),
        401 => Some("Unauthorized"),
        403 => Some("Forbidden"),
        404 => Some("Not Found"),
        405 => Some("Method Not Allowed"),
        406 => Some("Not Acceptable"),
        407 => Some("Proxy Authentication Required"),
        408 => Some("Request Timeout"),
        409 => Some("Conflict"),
        410 => Some("Gone"),
        411 => Some("Length Required"),
        // TODO: more?
        500 => Some("Internal Server Error"),
        501 => Some("Not Implemented"),
        502 => Some("Bad Gateway"),
        503 => Some("Service Unavailable"),
        // TODO: more?
        _ => None,
    }
}

impl Response {
    /// Helper to create a 200 OK response
    pub fn ok(body: String) -> Self {
        Self::with_code(200, body)
    }

    /// Creates a Response for the given code and body
    pub fn with_code(code: usize, body: String) -> Self {
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

    /// Converts the Response into a raw HTTP response string
    pub fn to_string(&self) -> String {
        let code_name = code_name(self.code);
        let status_line = match code_name {
            // TODO cleanup
            Some(name) => format!("HTTP/1.1 {} {}", self.code, name),
            None => {
                println!("Error: invalid status code in response: {}", self.code);
                "HTTP/1.1 500 Internal Server Error".to_string()
            }
        };
        let length = self.length();
        if self.headers.len() > 0 {
            let headers = self
                .headers
                .iter()
                .map(|(k, v)| format!("{k}: {v}"))
                .collect::<Vec<String>>()
                .join("\r\n");

            return format!(
                "{status_line}\r\n{headers}\r\nContent-Length: {length}\r\n\r\n{}",
                self.body
            );
        }
        return format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{}",
            self.body
        );
    }
}
