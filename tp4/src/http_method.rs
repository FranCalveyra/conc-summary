#[derive(Eq, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub fn from_str(string: &str) -> Self {
        match string {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::GET, //Default => Get
        }
    }
}
