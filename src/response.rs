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
                if items.len() < 3 { continue; }
                http_version = items[0].to_string();
                status_code = match std::str::FromStr::from_str(items[1]) {
                    Ok(i) => i,
                    Err(_) => 0
                };
                let mut j = 0;
                for x in items.iter() {
                    j += 1;
                    if j < 3 { continue; }
                    if j == 3 {
                        status_message.push_str(x);
                    } else {
                        status_message.push_str(&format!(" {}", x));
                    }
                }
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
}
