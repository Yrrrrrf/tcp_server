# HTTP Module Code Documentation

This module defines types and methods for working with HTTP requests and responses.

## Submodules

### Request Module (`request.rs`)
Contains types and methods related to HTTP requests.

### Response Module (`response.rs`)
Contains types and methods related to HTTP responses.

## HTTP Status Codes

### Macro: `impl_http_status_enum!`

Defines a macro that generates methods for an HTTP status enum. It includes methods for retrieving the numerical value, associated message, and converting a numerical value into the appropriate status code variant.

### Macro: `http_status_enum!`

Defines a macro that generates an HTTP status enum. It includes standard HTTP status codes with numerical values and messages. The macro utilizes `impl_http_status_enum!` to generate methods for the enum.

### Enum: `HttpStatus`

Represents a set of HTTP status codes and their associated messages. It includes methods for retrieving the numerical code, associated message, and converting a numerical value into the appropriate status code variant.

#### Methods
- `code(&self) -> u16`: Returns the numerical value associated with the enum variant.
- `message(&self) -> &str`: Returns the message associated with the enum variant.
- `from_u16(value: u16) -> Option<Self>`: Returns the enum variant associated with the given `u16` value.

### Default Implementation
- `Default for HttpStatus`: Returns the default HTTP status code, which is `HttpStatus::_501` (Not Implemented).

### Display Implementation
- `std::fmt::Display for HttpStatus`: Allows the enum to be printed to the console. It returns the HTTP status code as a string in the format `<code> <message>`, using ANSI escape codes for colored output based on the status code type.

## HTTP Methods

### Macro: `http_methods_enum!`

Defines a macro that generates an enum for HTTP methods (`GET`, `POST`, `PUT`, `DELETE`, etc.).

### Enum: `HttpMethod`

Represents HTTP methods and provides methods for obtaining string representations and converting string representations into enum variants.

#### Methods
- `as_str(&self) -> &str`: Returns a string representation of the HTTP method.
- `from_str(method: &str) -> Option<Self>`: Returns the HTTP method from a string representation.

### Default Implementation
- `Default for HttpMethod`: Returns the default HTTP method, which is `HttpMethod::GET`.

## HTTP Versions

### Macro: `http_versions_enum!`

Defines a macro that generates an enum for HTTP versions (`HTTP/1.0`, `HTTP/1.1`, `HTTP/2.0`).

### Enum: `HttpVersion`

Represents HTTP versions and provides methods for obtaining string representations and converting string representations into enum variants.

#### Methods
- `as_str(&self) -> &str`: Returns a string representation of the HTTP version.
- `from_str(version: &str) -> Option<Self>`: Returns the HTTP version from a string representation.

### Default Implementation
- `Default for HttpVersion`: Returns the default HTTP version, which is `HttpVersion::Http1_1`.

### Display Implementation
- `std::fmt::Display for HttpVersion`: Allows the enum to be printed to the console. It returns the string representation of the HTTP version.