use std;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Response {
    pub http_version: String,
    pub status_code: u16,
    pub status_message: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl Response {
    pub fn new(headers: &str, body: &str) -> Response {
        let mut http_version = String::new();
        let mut status_code: u16 = 0;
        let mut status_message = String::new();
        let mut refined_headers: HashMap<String, String> = HashMap::new();
        let lines: Vec<&str> = headers.split("\r\n").collect();
        let mut index = 0;
        for line in lines.iter() {
            index += 1;

            if index == 1 {
                let items: Vec<&str> = line.split(" ").collect();
                if items.len() < 2 { continue; }
                http_version = items[0].to_string();
                status_code = match std::str::FromStr::from_str(items[1]) {
                    Ok(i) => i,
                    Err(_) => 0
                };
                status_message = Response::get_status_message(status_code);
                continue;
            }

            let items: Vec<&str> = line.split(": ").collect();
            if items.len() != 2 { continue; }
            let key = items[0].to_string();
            let value = items[1].to_string();
            refined_headers.insert(key, value);
        }
        let response = Response {
            http_version: http_version,
            status_code: status_code,
            status_message: status_message,
            headers: refined_headers,
            body: body.to_string()
        };
        return response;
    }

    fn get_status_message(status_code: u16) -> String {
        let status_message = match status_code {

            // 1xx
            100 => "Continue",
            101 => "Switching Protocols",
            103 => "Checkpoint",

            // 2xx
            200 => "OK",
            201 => "Crated",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",

            // 3xx
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            306 => "Switch Proxy",
            307 => "Temporary Redirect",
            308 => "Resume Incomplete",

            // 4xx
            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Request Entity Too Large",
            414 => "Request-URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Requested Range Not Satisfiable",
            417 => "Expectation Failed",

            // 5xx
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            511 => "Network Authentication Required",
            
            // error
            _ => ""
        };
        return status_message.to_string();
    }
}
