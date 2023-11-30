# HTTP Request Struct Documentation

## Overview
This Rust module defines a struct for representing HTTP requests. The `HttpRequest` struct encapsulates information about the request method, HTTP version, URL, and body. It allows users to create and format HTTP requests easily.

### Features
- Represents HTTP requests with method, version, URL, and body.
- Provides a convenient method for creating HTTP requests.
- Offers a `ToString` implementation for converting the request to a formatted string.

## Usage
To use the `HttpRequest` struct, create an instance using the provided constructor and then format it as needed. The struct provides a `ToString` implementation for easily converting the request to a string.

---

## Struct Definition

### Struct Fields
- `method`: Represents the HTTP method of the request (`HttpMethod` enum).
- `http_version`: Represents the HTTP version of the request (`HttpVersion` enum).
- `url`: Represents the URL of the HTTP request.
- `body`: Represents the body of the HTTP request.

### Constructor
- `new`: Creates a new `HttpRequest` with the specified method, HTTP version, URL, and body.

### Methods
- `to_string`: Converts the `HttpRequest` to a formatted string.

---

## Example

```rust
use http_request::HttpRequest;
use http_method::HttpMethod;

let request = HttpRequest::new(HttpMethod::GET, HttpVersion::Http1_1, "/index.html", "Hello, Rust!".to_string());
println!("{}", request.to_string());
```

Output:
```
GET /index.html HTTP/1.1
Host: www.tutorialspoint.com
Accept-Language: en-us

Hello, Rust!
```

---

## Conclusion
This Rust module provides a straightforward `HttpRequest` struct for representing and formatting HTTP requests. Developers can use this struct as part of their projects to easily create HTTP requests for communication between clients and servers.