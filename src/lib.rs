extern crate openssl;

mod url;
mod method;
pub mod response;

use std::io::{self, Write, Read};
use std::net::TcpStream;
use method::Method;
use url::{Protocol, Url};
use response::Response;
use openssl::ssl::{SslStream, SslContext};
use openssl::ssl::SslMethod::Sslv23;

pub fn post(url: &str) -> io::Result<Response> {
    return connect(Method::POST, &try!(Url::new(url)));
}

fn connect(method: Method, url: &Url) -> io::Result<Response> {
    let addr = format!("{}:{}", url.host, url.port);
    let host = match url.protocol {
        Protocol::HTTP => {
            match url.port {
                80 => url.host.clone(),
                _ => format!("{}:{}", url.host, url.port)
            }
        }
        Protocol::HTTPS => {
            match url.port {
                443 => url.host.clone(),
                _ => format!("{}:{}", url.host, url.port)
            }
        }
    };
    
    let http_req = format!("{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                          method::to_string(method),
                          url.path,
                          host);

    let mut stream = match TcpStream::connect(&*addr) {
        Ok(stream) => stream,
        Err(_) => {
            let err = io::Error::new(io::ErrorKind::NotConnected,
                                     "");
            return Err(err);
        }
    };
    
    let raw = match url.protocol {
        Protocol::HTTP => {
            let _ = stream.write(http_req.as_bytes());
            let raw = try!(read(&mut stream));
            raw
        }
        Protocol::HTTPS => {
            let context = match SslContext::new(Sslv23) {
                Ok(context) => context,
                Err(_) => {
                    let err = io::Error::new(io::ErrorKind::NotConnected,
                                             "");
                    return Err(err);
                }
            };

            let mut ssl_stream = match SslStream::new(&context, stream) {
                Ok(stream) => stream,
                Err(_) => {
                    let err = io::Error::new(io::ErrorKind::NotConnected,
                                             "");
                    return Err(err);
                }
            };

            let _ = ssl_stream.write(http_req.as_bytes());
            let raw = try!(read(&mut ssl_stream));
            raw
        }
    };

    let response = try!(get_response(&raw));
    // redirect
    if response.status_code / 100 == 3 {
        let location = match response.headers.get("Location") {
            Some(location) => location,
            None => {
                let err = io::Error::new(io::ErrorKind::NotConnected,
                                         "");
                return Err(err);
            }
        };
    }
    
    return Ok(response);
}

fn read<S: Read>(stream: &mut S) -> io::Result<String> {
    const BUFFER_SIZE: usize = 1024;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut raw = String::new();
    loop {
        let len = match stream.read(&mut buffer) {
            Ok(size) => size,
            Err(_) => {
                let err = io::Error::new(io::ErrorKind::NotConnected,
                                     "");
                return Err(err);
            }
        };
        
        match std::str::from_utf8(&buffer[0 .. len]) {
            Ok(buf) => raw.push_str(buf),
            Err(_) => {
                let err = io::Error::new(io::ErrorKind::NotConnected,
                                     "");
                return Err(err);
            }
        }
        
        if len < BUFFER_SIZE { break; }
    }

    return Ok(raw);
}

fn get_response(raw: &str) -> io::Result<Response> {
    let http_response: Vec<&str> = raw.split("\r\n\r\n").collect();

    if http_response.len() < 2 {
        let err = io::Error::new(io::ErrorKind::InvalidInput,
                                 "Server returns an invalid response.");
        return Err(err);
    }
    let http_header = http_response[0];
    let http_body = http_response[1];
    let chunked_content_body: Vec<&str> = http_body.split("\r\n").collect();
    let mut content_body = String::new();

    if chunked_content_body.len() == 1 {
        content_body.push_str(http_body);
    } else {
        let mut index: i64 = 0;
        for chunk in chunked_content_body.iter() {
            index = index + 1;
            if index % 2 != 0 { continue; }
            content_body.push_str(chunk);
        }
    }

    let response = Response::new(http_header, &content_body);
    return Ok(response);
}

#[test]
fn it_works() {
}
