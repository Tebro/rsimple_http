use std::collections::HashMap;

pub struct Response {
    code: usize,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn ok(body: String) -> Self {
        Self::with_code(200, body)
    }

    pub fn with_code(code: usize, body: String) -> Self {
        return Self {
            code,
            headers: HashMap::new(),
            body,
        };
    }

    pub fn headers(&mut self, headers: Vec<(String, String)>) {
        self.headers = headers
            .iter()
            .map(|(k, v)| (k.clone(), v.clone())) //TODO: there has to be a better way
            .collect();
    }

    fn length(&self) -> usize {
        self.body.len()
    }

    pub fn to_string(&self) -> String {
        let status_line = format!("HTTP/1.1 {} OK", self.code); // TODO
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
