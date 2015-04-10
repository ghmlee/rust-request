extern crate request;

fn main() {
    match request::post("http://cosmos.cosmos.io/") {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
        }
    };
}
