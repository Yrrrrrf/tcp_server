//! This file contains the struct for a HTTP response.
//! 
//! # Example
//! ```
//! HTTP/1.1 / 200 OK
//! Date: Mon, 27 Jul 2009 12:28:53 GMT
//! Server: Apache/2.2.14 (Win32)
//! Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT
//! Content-Length: 88
//! Content-Type: text/html
//! Connection: Closed
//! ```
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

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


    /// Returns the current date and time in the format: 2021-08-01 16:00:00
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use http_response::HttpResponse;
    /// 
    /// let response = HttpResponse::new_1_1(HttpStatus::Ok, "Hello World!".to_string());
    /// println!("{}", response.to_string());
    /// ```
    pub fn now_hour_minute_second() -> String {
        // todo: Improve or create the now() fn in the datetime module (dev_utils)
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u64;
        timestamp -= 6 * 3600;  // remove 6 hours from the timestamp
        let (days, hours, minutes, seconds) = calculate_hour_minute_second(timestamp);
        let (years, months, days) = calculate_year_month_day(days);

        // Console out: 2021-08-01 16:00:00
        format!("{:4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}", years, months, days, hours, minutes, seconds)

        // todo: Change the console out to Mon, 27 Jul 2009 12:28:53 GMT (RFC 1123)
        // todo: Check https://learn.microsoft.com/en-us/dotnet/api/system.globalization.datetimeformatinfo.rfc1123pattern?view=net-7.0
        /*
        format!(
            "{:0>2} {:0>2} {:0>2} {:0>2} {:0>2} {:0>2}",
             years, months, days, hours, minutes, seconds
        )
        */
    }


    /// Returns the HTTP response as a string.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use http_response::HttpResponse;
    /// 
    /// let response = HttpResponse::new_1_1(HttpStatus::Ok, "Hello World!".to_string());
    /// println!("{}", response.to_string());
    /// 
    /// // HTTP/1.1 200 OK
    /// // Date: Mon, 27 Jul 2009 12:28:53 GMT
    /// // Server: Apache/2.2.14 (Win32)
    /// // Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT
    /// // Content-Length: 88
    /// // Content-Type: text/html
    /// // Connection: Closed
    /// ```
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



// todo: move this to a separate file (also used on the client)

// Define the HTTP versions using a macro
macro_rules! http_versions_enum {
    ($($variant:ident),*) => {
        /// Represents HTTP versions (HTTP/1.0, HTTP/1.1, HTTP/2.0).
        ///
        /// This enum provides a set of HTTP versions that can be used to specify the desired version
        /// when making HTTP requests. It allows you to work with standard HTTP versions and convert
        /// between their enum representation and string representation.
        #[derive(Debug, PartialEq, Clone)]
        pub enum HttpVersion {
            $($variant,)*
        }

        impl HttpVersion {
            /// Returns a string representation of the HTTP version.
            ///
            /// # Returns
            ///
            /// - `&str` - A string representation of the HTTP version.
            pub fn as_str(&self) -> &str {
                match self {
                    $(HttpVersion::$variant => stringify!($variant),)*
                }
            }

            /// Returns the HTTP version from a string representation.
            /// Returns None for unsupported versions.
            ///
            /// # Arguments
            ///
            /// - `version` - A string representation of the HTTP version.
            ///
            /// # Returns
            ///
            /// - `Some(HttpVersion)` - If the string represents a valid HTTP version.
            /// - `None` - If the string does not match any supported HTTP version.
            pub fn from_str(version: &str) -> Option<Self> {
                match version {
                    $(stringify!($variant) => Some(HttpVersion::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

http_versions_enum!(Http1_0, Http1_1, Http2_0);

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/{}", match self {
            HttpVersion::Http1_0 => "1.0",
            HttpVersion::Http1_1 => "1.1",
            HttpVersion::Http2_0 => "2.0",
        })
    }
}

impl Default for HttpVersion {
    /// Returns the default HTTP version, which is [`HttpVersion::Http1_1`].
    ///
    /// The default version is typically used when an HTTP request is made without explicitly
    /// specifying a version.
    fn default() -> Self {
        HttpVersion::Http1_1
    }
}
