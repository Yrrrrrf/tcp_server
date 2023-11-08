//! This module defines a collection of HTTP methods often used in web applications.
//!
//! It provides an enum `HttpMethod` that represents various HTTP methods, including GET, POST,
//! PUT, DELETE, and more. The module allows you to work with these methods in a type-safe manner,
//! making it easier to handle HTTP requests and routes.
//!
//! The `HttpMethod` enum includes common HTTP methods and can be used in conjunction with
//! web frameworks or routing libraries to specify the desired HTTP method for handling requests.
//!
//! # Example:
//! ```rust
//! use http_methods::HttpMethod;
//!
//! let method = HttpMethod::GET;
//! if method == HttpMethod::GET {
//!     println!("Received a GET request.");
//! }
//! ```
//!
//! The module provides a convenient way to work with HTTP methods in Rust, promoting code clarity
//! and reducing the risk of method-related bugs.


/// Define a macro called 'http_methods_enum' that generates an enum for HTTP methods.
macro_rules! http_methods_enum {
    ($($variant:ident),*) => {
        /// Represents HTTP methods (GET, POST, PUT, DELETE, etc.).
        ///
        /// This enum provides a set of HTTP methods that can be used to specify the desired action
        /// when making HTTP requests. It allows you to work with standard HTTP methods and convert
        /// between their enum representation and string representation.
        #[derive(Debug, PartialEq, Clone)]
        pub enum HttpMethod {
            $($variant,)*
        }

        impl HttpMethod {
            /// Returns a string representation of the HTTP method.
            ///
            /// # Returns
            ///
            /// - `&str` - A string representation of the HTTP method.
            pub fn as_str(&self) -> &str {
                match self {
                    $(HttpMethod::$variant => stringify!($variant),)*
                }
            }

            /// Returns the HTTP method from a string representation.
            /// Returns None for unsupported methods.
            ///
            /// # Arguments
            ///
            /// - `method` - A string representation of the HTTP method.
            ///
            /// # Returns
            ///
            /// - `Some(HttpMethod)` - If the string represents a valid HTTP method.
            /// - `None` - If the string does not match any supported HTTP method.
            pub fn from_str(method: &str) -> Option<Self> {
                match method {
                    $(stringify!($variant) => Some(HttpMethod::$variant),)*
                    _ => None,
                }
            }
        }
    };
}


// Generate the HTTP methods enum and methods using the macro.
http_methods_enum!(GET, POST, PUT, DELETE);

impl Default for HttpMethod {
    /// Returns the default HTTP method, which is [`HttpMethod::GET`].
    ///
    /// The default method is typically used when an HTTP request is made without explicitly
    /// specifying a method.
    ///
    /// # Returns
    ///
    /// - [`HttpMethod`] - The default HTTP method.
    fn default() -> Self {
        HttpMethod::GET
    }
}
