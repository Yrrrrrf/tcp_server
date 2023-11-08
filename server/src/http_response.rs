//! This file contains the struct for a HTTP response.
//! 
//! # HTTP Response
//! 
//! ![Server](https://www.tutorialspoint.com/http/http_responses.htm)
//! 
//! # Example
//! ```
//! HTTP/1.1 200 OK
//! Date: Mon, 27 Jul 2009 12:28:53 GMT
//! Server: Apache/2.2.14 (Win32)
//! Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT
//! Content-Length: 88
//! Content-Type: text/html
//! Connection: Closed
//! ```
use crate::http_status::HttpStatus;
use dev_utils::conversion::datetime;


// ? Structs ------------------------------------------------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
)]
pub struct Response {
    pub status: HttpStatus,
    pub headers: Vec<String>,
    pub body: String,
}

impl Response {
    pub fn new(status: HttpStatus, headers: Vec<String>, body: String) -> Response {
        Response {status, headers, body}
    }

    pub fn to_string(&self) -> String {
        let mut response = String::new();
        response.push_str(&format!("HTTP/1.1 {} {}\r\n", self.status.code(), self.status.message()));
        // response.push_str(&format!("Date: {}\r\n", datetime::now()));
        response.push_str(&format!("Server: {}\r\n", "Rust Server"));
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        response.push_str(&format!("Content-Type: {}\r\n", "text/html"));
        response.push_str(&format!("Connection: {}\r\n", "Closed"));
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }

}
