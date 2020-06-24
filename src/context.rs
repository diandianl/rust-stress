extern crate reqwest;

use reqwest::blocking::Client;

pub struct Context {
    pub http: Client
}

impl Context {
    pub fn new() -> Self {
        let http = Client::builder().build().unwrap();
        Context {
            http
        }
    }
}