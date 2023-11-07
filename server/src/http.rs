//! This module contains the configuration of the server.
//! 
//! ![Server](https://www.tutorialspoint.com/http/http_requests.htm)
//! ![HTTP Status Codes](.\\resources\\http_status_codes.png)
/// The IP address of the server.
pub const SERVER_IP: &str = "127.0.0.1";
/// The port that the server listens on.
pub const SERVER_PORT: &str = "8080";


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
        // Define an enum called 'HttpStatus' with associated attributes for debugging, comparisons, and cloning.
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

    // * 3XX: Redirection - Further action must be taken in order to complete the request
    _300 => (300, "Multiple Choices"),
    _301 => (301, "Moved Permanently"),
    _302 => (302, "Found"),

    // * 4XX: Client Error - The request contains bad syntax or cannot be fulfilled
    _404 => (404, "Not Found"),
    _405 => (405, "Method Not Allowed"),

    // * 5XX: Server Error - The server failed to fulfill an apparently valid request
    _500 => (500, "Internal Server Error"),
    _501 => (501, "Not Implemented"),
    _599 => (599, "Network Connect Timeout Error"),
);

impl HttpStatus {
    pub fn new(value: u16) -> Option<Self> {
        Self::from_u16(value)
    }

    pub fn get_as_response(&self) -> String {
        format!("HTTP/1.1 {}\r\n\r\n", self.message())
    }
}

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
