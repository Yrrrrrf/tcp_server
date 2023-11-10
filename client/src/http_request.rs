//! HTTP Request
//! 
//! A HTTP request is a message sent from a client to a server. It contains a request line, headers, and a body.
//! 
//! # HTTP Request
//! 
//! ![Client](https://www.tutorialspoint.com/http/http_requests.htm)
//! 
//! # Example
//! ```
//! GET /index.html HTTP/1.1
//! User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
//! Host: www.tutorialspoint.com
//! Accept-Language: en-us
//! ```
use std::fmt::Display;

use crate::http_methods::HttpMethod;


#[derive(Debug, Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl Request {
    pub fn new(method: HttpMethod, url: impl Into<String> + Display
        , headers: Vec<String>, body: String) -> Request {
        Request {method, url: url.to_string(), headers, body}
    }

    pub fn new_1_1(method: HttpMethod, url: impl Into<String> + Display, body: String) -> Request {
        Request {
            method, 
            url: url.to_string(),
            headers: Vec::new(), 
            body
        }
    }

}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::new();
        for header in &self.headers {
            headers.push_str(&format!("{}\r\n", header));
        }
        write!(f, "{} {} HTTP/1.1\r\n{}\r\n{}", self.method.as_str(), self.url, headers, self.body)
    }
}

