//! This module contains the configuration of the server.
//! 
//! ![Server](https://www.tutorialspoint.com/http/http_requests.htm)
//! ![HTTP Status Codes](.\\resources\\http_status_codes.png)
/// The IP address of the server.
pub const SERVER_IP: &str = "127.0.0.1";
/// The port that the server listens on.
pub const SERVER_PORT: &str = "8080";


macro_rules! generate_http_status_to_u16 {
    ($enum_name:ident; $($variant:ident => $value:expr),* $(,)? ; $default:expr) => {
        impl $enum_name {
            pub fn code(&self) -> u16 {
                match self {
                    $($enum_name::$variant => $value,)*
                    _ => $default,
                }
            }
        }
    };
}

macro_rules! http_status_enum {
    ($($variant:ident => $value:expr),* $(,)? ; $default:expr) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum HttpStatus {
            $($variant,)*
        }

        generate_http_status_to_u16!(HttpStatus; $($variant => $value),* ; $default);
    };
}

http_status_enum!(
    _100 => 100,
    _101 => 101,
    _102 => 102,
    _200 => 200,
    _404 => 404,
    _500 => 500,
    _501 => 501,
    _505 => 505;
    501
);












// /// The HTTP status codes.
// /// 
// /// This enum defines the HTTP status codes.
// #[derive(
//     Debug,  // Allow to debug print the enum values (e.g. with {:?} or {:#?})
//     Clone,  // Allow to clone the enum values
//     Copy,  // Allow to copy the enum values
//     PartialEq  // Allow to compare the enum values with ==
// )]
// pub enum HttpStatus {
//     _100,  // Continue
//     _101,  // Switching Protocols
//     _102,  // Processing

//     _200,  // OK

//     _404,  // Not Found

//     _500,  // Internal Server Error
//     _501,  // Not Implemented
//     _505,  // HTTP Version Not Supported
// }

// // u16: 0..=65535 == 0..=2^16-1
// impl HttpStatus {
//     pub fn code(&self) -> u16 {
//         match self {
//             HttpStatus::_100 => 100,
//             HttpStatus::_101 => 101,
//             HttpStatus::_102 => 102,

//             HttpStatus::_200 => 200,

//             HttpStatus::_404 => 404,

//             HttpStatus::_500 => 500,
//             HttpStatus::_501 => 501,
//             HttpStatus::_505 => 505,

//             // ^ for all other cases
//             _ => 501,  // Same as 501
//         }
//     }


//     pub fn from_code(code: u16) -> Self {
//         match code {
//             100 => HttpStatus::_100,
//             101 => HttpStatus::_101,
//             102 => HttpStatus::_102,

//             200 => HttpStatus::_200,

//             404 => HttpStatus::_404,

//             500 => HttpStatus::_500,
//             501 => HttpStatus::_501,
//             505 => HttpStatus::_505,

//             // ^ for all other cases
//             _ => HttpStatus::_404,  // Same as 501
//         }
//     }

// }

// impl std::fmt::Display for HttpStatus {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "HTTP/1.1 {}\r\n\r\n", match self {
            
//             HttpStatus::_100 => "100 Continue",
//             HttpStatus::_101 => "101 Switching Protocols",
//             HttpStatus::_102 => "102 Processing",
            
//             HttpStatus::_200 => "200 OK",
            
//             HttpStatus::_404 => "404 Not Found",

//             HttpStatus::_500 => "500 Internal Server Error",
//             HttpStatus::_501 => "501 Not Implemented",
//             HttpStatus::_505 => "505 HTTP Version Not Supported",

//             // ^ for all other cases
//             _ => "Not Implemented", // Same as 501
//         })
//     }
// }
