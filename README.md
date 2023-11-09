<h1 align="center">
    <img src="./resources/img/server.png" alt="Space Ship" width="128">
    <div align="center">TCP Server</div>
</h1>

This project is a simple TCP server that listens on a specific IP address and port.

It allows a client to connect to it and send a message. The server will then print the message to the console.

The main purpose of this project is to learn how to create a TCP server in Rust without using any external crates. **Only the Rust standard library is used**.

## Setup
- Set an IP address on the [`main_client`](./client/src/main.rs) & [`main_server`](./server/src/main.rs) files to connect to the server.
- Use `cargo build` to build the project.
- Use `cargo run` to run the project. 
```bash
cargo run -p server  # Initialize the server on the specified IP address and port
cargo run -p client  # Test the connection with the client
# The clien probably will become a separate project or an struct in the server project
# Maybe this (all) will be abstracted in a module for dev_utils crate
```

## Features
- [X] This project meants to be a TCP server that listens on a specific IP address and port.
- [X] Error Handling
- [ ] Socket Programming: Use socket programming libraries and APIs (e.g., socket in Python, socket in C/C++) to create and manage network sockets for both the client and server. Sockets are essential for establishing connections and sending/receiving data.
- [X] Connection Establishment between Client and Server
- [ ] Data Serialization
- [ ] Message Framing: Establish a protocol for framing messages, so the receiver knows where one message ends and the next one begins. Common techniques include using fixed-length messages or delimiters.
- [ ] Logging and Monitoring
- [ ] Testing

## References
- [TCP Server](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
- [TCP Client](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
- [Single Threaded TCP Server](https://doc.rust-lang.org/stable/book/ch20-01-single-threaded.html)
- [Multi Threaded TCP Server](https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html)

----
## [License](./../LICENSE.md)
This project is licensed under the terms of the [MIT License](LICENSE.md).

## Attributions
This project uses an icon from [flaticon.com](https://www.flaticon.com/). The individual attributions are in the [attributions.md](./resources/img/attributions.md) file.
