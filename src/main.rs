use std::io;
use serde_json;

pub mod request_response;
use request_response::Request;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.len() == 0 {
            break;
        }
        let req: Request = serde_json::from_str(input.as_str()).expect("Unable to parse");
        let resp = serde_json::to_string(&req.respond()).expect("Unable to respond");
        println!("{}", &resp);
    }
}
