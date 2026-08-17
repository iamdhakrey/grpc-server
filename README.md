# gRPC Test Server

A lightweight, high-performance gRPC server built with Rust and [Tonic](https://github.com/hyperium/tonic). 

This project serves as a reliable local test target for developing gRPC clients, API testers (like Samvad), or experimenting with HTTP/2 multiplexing. It implements both Unary and Server-Streaming RPCs.

## Features & Endpoints

The server listens on `[::1]:50051` by default and implements two Protocol Buffer services:

### 1. `echo.EchoService`
*   **`Ping` (Unary):** Accepts a message and returns it wrapped in a `Pong` with a UTC timestamp.
*   **`StreamEcho` (Server Streaming):** Accepts a message and streams back 6 sequential responses at 500ms intervals, simulating a long-lived chunked connection.

### 2. `hello_world.HelloService`
*   **`SayHello` (Unary):** Standard greeting endpoint.
*   **`SayHelloStream` (Server Streaming):** Streams a 3-part greeting sequence with 1-second delays.

## Prerequisites

*   **Rust toolchain** (latest stable)
*   **Protocol Buffers Compiler (`protoc`)**: Required by `tonic-build` to compile the `.proto` files during the cargo build step.
    *   macOS: `brew install protobuf`
    *   Linux: `sudo apt install protobuf-compiler`
    *   Arch Linux: `sudo pacman -S protobuf`

## Project Structure

```text
.
├── build.rs              # Tonic build script to compile .proto files
├── proto/
│   ├── echo.proto        # Schema for EchoService
│   └── hello.proto       # Schema for HelloService
├── src/
│   ├── main.rs           # Server initialization and network binding
│   ├── echo_handler.rs   # Implementation of EchoService trait
│   └── hello_handler.rs  # Implementation of HelloService trait
└── Cargo.toml
