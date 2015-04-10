#[allow(dead_code)]
pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT
}

pub fn to_string(method: Method) -> String {
    return match method {
        Method::OPTIONS => "OPTIONS".to_string(),
        Method::GET => "GET".to_string(),
        Method::HEAD => "HEAD".to_string(),
        Method::POST => "POST".to_string(),
        Method::PUT => "PUT".to_string(),
        Method::DELETE => "DELETE".to_string(),
        Method::TRACE => "TRACE".to_string(),
        Method::CONNECT => "CONNECT".to_string()
    };
}
