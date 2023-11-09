//! This file contains the struct for a HTTP response.
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
use std::{fmt::Display, time::{SystemTime, UNIX_EPOCH}};

use crate::http_status::HttpStatus;
use dev_utils::conversion::datetime::{self, calculate_hour_minute_second, calculate_year_month_day};


#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub http_version: HttpVersion,
    // todo: Add the headers to the response
    // pub headers: Vec<String>,
    pub body: String,
}

impl HttpResponse {
    // pub fn new(status: HttpStatus, http_version: HttpVersion, headers: Vec<String>, body: String) -> HttpResponse {
        // HttpResponse {status, http_version, headers, body}
    pub fn new(status: HttpStatus, http_version: HttpVersion, body: String) -> HttpResponse {
        HttpResponse {status, http_version, body}
    }


    // pub fn new_1_1(status: HttpStatus, headers: Vec<String>, body: String) -> HttpResponse {
    //     HttpResponse {status, http_version: HttpVersion::Http1_1, headers, body}
    pub fn new_1_1(status: HttpStatus, body: String) -> HttpResponse {
        HttpResponse {status, http_version: HttpVersion::Http1_1, body}
    }


    pub fn now_hour_minute_second() -> String {
        // todo: Improve or create the now() fn in the datetime module (dev_utils)
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u64;
        timestamp -= 6 * 3600;  // remove 6 hours from the timestamp
        let (days, hours, minutes, seconds) = calculate_hour_minute_second(timestamp);
        let (years, months, days) = calculate_year_month_day(days);

        // Console out: 2021-08-01 16:00:00
        format!("{:4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}", years, months, days, hours, minutes, seconds)
        // todo: Change the console out to Mon, 27 Jul 2009 12:28:53 GMT (RFC 1123)
        /*
        format!(
            "{:0>2} {:0>2} {:0>2} {:0>2} {:0>2} {:0>2}",
             years, months, days, hours, minutes, seconds
        )
        */
    }


    pub fn to_string(&self) -> String {
        format!("{} {} {}\r\nDate: {}\r\nServer: {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: {}\r\n\r\n{}", 
            self.http_version, self.status.code(), self.status.message(), // HTTP/1.1 200 OK
            Self::now_hour_minute_second(),  // Date: Mon, 27 Jul 2009 12:28:53 GMT
            "Rust Server",  // Server: Apache/2.2.14 (Win32)
            self.body.len(),  // Content-Length: 88
            "text/html",  // Content-Type: text/html
            "Closed",  // Represents the connection type
            self.body
        )
    }

}


#[derive(
    Debug,
    Clone,
    Default,
)]
pub enum HttpVersion {
    Http1_0,
    // Add the comment to make Http1_1 the default
    #[default]
    Http1_1,
    Http2_0,
}

// impl HttpVersion {
// }


impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/{}", match self {
            HttpVersion::Http1_0 => "1.0",
            HttpVersion::Http1_1 => "1.1",
            HttpVersion::Http2_0 => "2.0",
        })
    }
}
