# HTTP Response Struct Documentation

## Overview
This Rust module defines a struct for representing HTTP responses. The `HttpResponse` struct includes information about the response status, HTTP version, and body. It also provides methods for creating and formatting HTTP responses.

### Features
- Represents HTTP responses with status, version, and body.
- Generates HTTP responses with the appropriate headers.
- Provides methods for creating and formatting HTTP responses.

## Usage
To use the `HttpResponse` struct, create an instance using one of the provided constructors and then format it as needed. The struct provides methods for converting the response to a string with the necessary headers.

---

## Struct Definition

### Struct Fields
- `status`: Represents the HTTP status of the response (`HttpStatus` enum).
- `http_version`: Represents the HTTP version of the response (`HttpVersion` enum).
- `body`: Represents the body of the HTTP response.

### Constructors
- `new`: Creates a new `HttpResponse` with the specified status, HTTP version, and body.
- `new_1_1`: Creates a new `HttpResponse` with HTTP version 1.1, the specified status, and body.

### Methods
- `now_hour_minute_second`: Returns the current date and time in the format `YYYY-MM-DD HH:mm:ss`.
- `to_string`: Converts the `HttpResponse` to a formatted string, including headers and body.

---

## Example

```rust
use http_response::HttpResponse;

let response = HttpResponse::new_1_1(HttpStatus::Ok, "Hello World!".to_string());
println!("{}", response.to_string());
```

Output:
```
HTTP/1.1 200 OK
Date: Mon, 27 Jul 2009 12:28:53 GMT
Server: Rust Server
Content-Length: 12
Content-Type: text/html
Connection: Closed

Hello World!
```

---

## Conclusion
This Rust module provides a simple and flexible `HttpResponse` struct for representing and formatting HTTP responses. It can be used as part of a broader web server implementation, allowing developers to generate HTTP responses with ease.