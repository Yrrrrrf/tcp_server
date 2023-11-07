//! This file contains the HTTP status enum and associated methods.
//! 
//! # HTTP Status Codes
//! 
//! ![Server](https://www.tutorialspoint.com/http/http_requests.htm)

/// Define a macro called 'impl_http_status_enum' that takes two arguments:
/// 1. $enum_name: the name of an enum type, and
/// 2. a sequence of 'variant' definitions, each containing:
///    - $variant: the variant name,
///    - $value: the numerical value associated with the variant,
///    - $message: a string message associated with the variant.
macro_rules! impl_http_status_enum {
    ($enum_name:ident; $($variant:ident => ($value:expr, $message:expr)),* $(,)?) => {
        impl $enum_name {  // Create a set of methods for the specified enum type ($enum_name).

            /// Returns the numerical value associated with the enum variant.
            /// 
            /// # Returns
            /// 
            /// - [u16] - The numerical value associated with the enum variant.
            pub fn code(&self) -> u16 {  // Define a 'code' method that returns a u16.
                // Match the value of 'self' (an instance of the enum) to its variants.
                match self {$($enum_name::$variant => $value,)*}
            }


            /// Returns the message associated with the enum variant.
            /// 
            /// # Returns
            /// 
            /// - `&str` - The message associated with the enum variant.
            /// 
            /// # Returns
            /// 
            /// - `&str` - The message associated with the enum variant.
            pub fn message(&self) -> &str {  // Define a 'message' method that returns a string message.
                // Match the value of 'self' to its variants and return the associated $message.
                match self {$($enum_name::$variant => $message,)*}
            }

            /// Returns the enum variant associated with the given [u16] value.
            /// 
            /// # Arguments
            /// 
            /// - `value` - The [u16] value to match against the enum variants.
            /// 
            /// # Returns
            /// 
            /// - `Some(enum_variant)` - If the value matches one of the enum variants.
            pub fn from_u16(value: u16) -> Option<Self> {  // Define a 'from_u16' method that takes a u16 and returns an Option of the enum.
                match value {  // Match the provided 'value' to the $value associated with each variant.
                    $($value => Some($enum_name::$variant),)*  // If there's a match, return Some(enum_variant).
                    _ => None,  // If there's no match, return None.
                }
            }

        }
    };
}


// Define a macro called 'http_status_enum' that takes a sequence of 'variant' definitions,
// each containing the variant name ($variant), the numerical value ($value), and a message ($message).
macro_rules! http_status_enum {
    ($($variant:ident => ($value:expr, $message:expr)),* $(,)?) => {
        /// Represents a set of HTTP status codes and their associated messages.
        ///
        /// This enum allows you to work with standard HTTP status codes and their corresponding
        /// messages. It provides methods to retrieve the numerical code, the associated message,
        /// and convert a numerical value into the appropriate status code variant.
        ///
        /// # Example
        ///
        /// ```
        /// pub mod http;
        /// use http::HttpStatus;
        ///
        /// let status = HttpStatus::_200;
        /// assert_eq!(status.code(), 200);
        /// assert_eq!(status.message(), "OK");
        ///
        /// let value = 404;
        /// match HttpStatus::from_u16(value) {
        ///     Some(status) => println!("HTTP Status from u16: {:?}", status),
        ///     None => println!("Invalid HTTP Status Code"),
        /// }
        /// ```
        #[derive(
            Debug,  // Allow writting the enum to the console. (e.g. println!("{:?}", HttpStatus::_200))
            PartialEq,  // Allow the comparison of enum variants using the '==' operator.
            Clone  // Allow the cloning of enum variants.
        )]
        pub enum HttpStatus {
            // List the enum variants ($variant).
            $($variant,)*
        }

        impl_http_status_enum!(HttpStatus; $($variant => ($value, $message)),*);
    };
}


// Define the HTTP status enum using the 'http_status_enum' macro.
http_status_enum!(
    // Define variants with numerical values and messages.
    // * 1XX: Informational - Request received, continuing process
    _100 => (100, "Continue"),
    _101 => (101, "Switching Protocols"),
    _102 => (102, "Processing"),

    // * 2XX: Success - The action was successfully received, understood, and accepted
    _200 => (200, "OK"),
    _201 => (201, "Created"),
    _202 => (202, "Accepted"),
    _203 => (203, "Non-Authoritative Information"),
    _204 => (204, "No Content"),
    _205 => (205, "Reset Content"),

    // * 3XX: Redirection - Further action must be taken in order to complete the request
    _300 => (300, "Multiple Choices"),
    _301 => (301, "Moved Permanently"),
    _302 => (302, "Found"),

    // * 4XX: Client Error - The request contains bad syntax or cannot be fulfilled
    _400 => (400, "Bad Request"),
    _401 => (401, "Unauthorized"),
    // _402 => (402, "Payment Required"),
    _403 => (403, "Forbidden"),
    _404 => (404, "Not Found"),
    _405 => (405, "Method Not Allowed"),
    _406 => (406, "Not Acceptable"),
    _408 => (408, "Request Timeout"),
    _409 => (409, "Conflict"),
    _410 => (410, "Gone"),

    // * 5XX: Server Error - The server failed to fulfill an apparently valid request
    _500 => (500, "Internal Server Error"),
    _501 => (501, "Not Implemented"),
    _502 => (502, "Bad Gateway"),
    _503 => (503, "Service Unavailable"),
    _504 => (504, "Gateway Timeout"),
    // _505 => (505, "HTTP Version Not Supported"),
    // _506 => (506, "Variant Also Negotiates"),
    _507 => (507, "Insufficient Storage"),
    // _508 => (508, "Loop Detected"),
    // _510 => (510, "Not Extended"),
    _511 => (511, "Network Authentication Required"),
    _599 => (599, "Network Connect Timeout Error"),
);

impl HttpStatus {
    /// Returns the enum variant associated with the given [u16] value.
    /// 
    /// # Arguments
    /// 
    /// - `value` - The [u16] value to match against the enum variants.
    /// 
    /// # Returns
    /// 
    /// - `Some(enum_variant)` - If the value matches one of the enum variants.
    pub fn new(value: u16) -> Option<Self> {
        Self::from_u16(value)
    }


    /// Returns the HTTP status code as a [`String`] in the format: `HTTP/1.1 <code> <message>`.
    /// 
    /// # Returns
    /// 
    /// - [`String`] - The HTTP status code as a [`String`].
    /// 
    /// # Example
    /// 
    /// ```
    /// pub mod http;
    /// use http::HttpStatus;
    /// 
    /// let status = HttpStatus::_200;
    /// assert_eq!(status.get_as_response(), "HTTP/1.1 200 OK\r\n\r\n");
    /// ```
    pub fn get_as_response(&self) -> String {
        format!("HTTP/1.1 {}\r\n\r\n", self.message())
    }
}

/// Returns the default HTTP status code.
/// 
/// The default HTTP status code is [`HttpStatus::_501`] (Not Implemented).
/// This because could be used as a default response if the server is unable to handle the request.
/// 
/// # Returns
/// 
/// - [`HttpStatus`] - The default HTTP status code.
impl Default for HttpStatus {
    fn default() -> Self {
        HttpStatus::_501  // Not Implemented
    }
}

impl std::fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
