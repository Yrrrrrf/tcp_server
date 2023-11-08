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
use crate::http_methods::HttpMethod;
// use crate::http_status::HttpStatus;

// ? Structs ------------------------------------------------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl Request {
    
}