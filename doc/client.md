# TCP Client Code Documentation

## Overview
This Rust module defines a simple TCP client that connects to a server and sends an HTTP request. The client is capable of receiving and parsing the server's response.

### Features
- Connects to a server using TCP.
- Sends an HTTP request to the server.
- Receives and parses the server's response.
- Handles response headers and body.

## Usage
To use the client, call the `main` function. The client reads its configuration from the `resources/keys/keys.toml` file, including the server's IP address and port.

### Dependencies
This module relies on external crates for various utilities:
- `log`: A flexible logging framework for Rust.
- `dev_utils`: A set of utility functions for development purposes.

---

## Module Structure

### Module Imports
The module imports various components from the standard library, external crates (`log`, `dev_utils`), and internal modules (`http`, `files`).

### Main Function
The `main` function initializes the client, reads configuration data, and connects to the server using a TCP stream. It sends an HTTP request and handles the server's response.

### Connecting to the Server
The client connects to the server using the `TcpStream::connect` function. It reads the server's IP address and port from the configuration file.

### Sending HTTP Request
The client constructs an `HttpRequest` and sends it to the server using the TCP stream.

### Receiving and Parsing Response
The client reads the server's response into a buffer, converts it to a string, and parses the response. It currently extracts the response body by splitting it after the `<!DOCTYPE html>` declaration.

### Handling Response Headers
TODO: The code includes a placeholder for handling response headers. This section needs to be implemented.

---

## Conclusion
This Rust TCP client module provides a basic foundation for connecting to a server, sending an HTTP request, and handling the server's response. It can be extended to include more sophisticated features such as handling various HTTP methods, parsing response headers, and processing different types of responses.