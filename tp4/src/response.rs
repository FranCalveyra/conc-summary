use std::fmt::{Display, Formatter};

pub struct Response {
    pub status_code: i32,
    pub body: String,
}

impl Response {
    pub fn new(status_code: i32, body: String) -> Self {
        Response { status_code, body }
    }

    pub fn from_status(status_code: i32) -> Self {
        Response {
            status_code,
            body: Self::_status(status_code),
        }
    }

    fn status(&self) -> String {
        Self::_status(self.status_code)
    }
    fn _status(status_code: i32) -> String {
        let status_string = match status_code {
            200 => "OK",
            201 => "Created",
            429 => "Too Many Requests",
            400 => "Bad Request",
            500 => "Internal Server Error",
            _ => "Unknown Error",
        };
        String::from(status_string)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
            self.status_code,
            self.status(),
            self.body.len(),
            self.body,
        );
        write!(f, "{}", string)
    }
}
