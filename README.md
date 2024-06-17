<h1 align="center">
    <img src="./resources/img/server.png" alt="Space Ship" width="128">
    <div align="center">TCP Server</div>
</h1>

This project is a simple TCP server that listens on a specific IP address and port.

It allows a client to connect to it and send a message. The server will then print the message to the console.

The main purpose of this project is to learn how to create a TCP server in Rust without using any external crates. **Only the Rust standard library is used**.

## Setup

### Prerequisites
- Check the [`Cargo.toml`](./Cargo.toml) file to see the dependencies.
```toml
[dependencies]
log = "0.4.*"
dev_utils = "0.*"  # most recent version
```
- Modify the [`keys.toml`](./resources/keys/keys.toml) file to set the IP address and port of the server.
```toml
[ip_address]
ip = "127.0.0.1"  # localhost
port = "8080"  # default port
```

### Running the Project
- Use `cargo build` to build the project. Then execute these commands to run the server and the client.
```bash
cargo run --example server  # Initialize the server on the specified IP address and port
cargo run --example client  # Test the connection with the client
# Maybe this (all of these) will be abstracted in a module for dev_utils crate
```

## Features
- [X] Set an IP address and port. [(`keys.toml`)](./resources/keys/keys.toml)
- [X] Error Handling
- [X] Impl a basic Web Socket Server
  - [X] Logging and Monitoring
  - [X] Connection Establishment between Client and Server
  - [X] Add multitreading support 
  - [X] Data Serialization
  - [ ] Message Framing: Establish a protocol for framing messages, so the receiver knows where one message ends and the next one begins. Common techniques include using fixed-length messages or delimiters.
- [X] Testing

## References
- [TCP Server (TCPListener)](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
- [TCP Client (TCPStream)](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
- [Single Threaded TCP Server](https://doc.rust-lang.org/stable/book/ch20-01-single-threaded.html)
- [Multi Threaded TCP Server](https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html)

----
## [License](./../LICENSE.md)
This project is licensed under the terms of the [MIT License](LICENSE.md).

## Attributions
This project uses an icon from [flaticon.com](https://www.flaticon.com/). The individual attributions are in the [attributions.md](./resources/img/attributions.md) file.
