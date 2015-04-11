# Request

[![Build Status](https://travis-ci.org/ghmlee/rust-request.svg?branch=master)](https://travis-ci.org/ghmlee/rust-request)

## Quick start

```rust
extern crate request;

use std::collections::HashMap;

let url = "https://github.com/ghmlee";
let mut headers: HashMap<String, String> = HashMap::new();
headers.insert("Connection".to_string(), "close".to_string());
let body = "";

let res = match request::get(&url, &mut headers, &body) {
    Ok(res) => res,
    Err(e) => { println!("{}", e); return; }
};

println!("{}", res.http_version);
println!("{}", res.status_code);
println!("{}", res.status_message);
println!("{}", res.body);
```