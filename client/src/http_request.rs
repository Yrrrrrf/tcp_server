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

    pub fn new_1_1_get(url: impl Into<String> + Display) -> Request {
        Request {
            method: HttpMethod::GET, 
            url: url.to_string(),
            headers: Vec::new(), 
            body: String::new()
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





// todo: move this to a separate file (also used on the server)






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
