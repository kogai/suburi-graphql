extern crate hyper;

use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, ContentType};

fn main() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let mut s = String::new();

    client.post("http://localhost:3000/graphql")
        .headers(headers)
        .body(r#"{"query": "{ hello }"}"#)
        .send()
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();

    println!("{}", s);
}

