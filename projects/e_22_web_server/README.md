# Final Project: Building a Multithreaded Web Server

This project is only for learning purposes. It is not intended to be used in production environments.

## Roundabouts

- `HTTP` (HyperText Transfer Protocol) and `TCP` (Transmission Control Protocol) are the protocols used to communicate between web servers and clients.
  - `HTTP` is a high-level protocol that defines how web servers and clients communicate.
  - `TCP` is a low-level protocol that defines how data is sent between computers.

## Setting Up a Simple Web Server

- Rust offers the `std::net` module to create a web server.
- The `std::net` module provides the `TcpListener` type to listen for incoming TCP connections.

  ```rust
  use std::{
      io::{prelude::*, BufReader},
      net::{TcpListener, TcpStream},
  };

  fn main() {
      let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // bind to the local address

      for stream in listener.incoming() {
          let stream = stream.unwrap(); // handle the stream

          handle_connection(stream); // handle the connection
      }
  }

  fn handle_connection(mut stream: TcpStream) {
      let buf_reader = BufReader::new(&mut stream); // read the stream
      let http_request: Vec<_> = buf_reader
          .lines()
          .map(|result| result.unwrap())
          .take_while(|line| !line.is_empty())
          .collect(); // collect the lines

      println!("Request: {:#?}", http_request);
  }
  ```

  - Output:

    ```
        Running `target/debug/hello`
    Request: [
        "GET / HTTP/1.1",
        "Host: 127.0.0.1:7878",
        "Sec-Fetch-Site: none",
        "Connection: keep-alive",
        "Upgrade-Insecure-Requests: 1",
        "Sec-Fetch-Mode: navigate",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15",
        "Accept-Language: en-US,en;q=0.9",
        "Sec-Fetch-Dest: document",
        "Accept-Encoding: gzip, deflate",
    ]
    ```

  - We use `7878` as the port number because it is not a well-known port number, if we use a well-known port number like `80` we need to run the server with `sudo` permissions.
  - `bind` returns a `Result` type, so we use `unwrap` to panic if the result is an `Err`.

- The `incoming` method returns an iterator that yields a new `TcpStream` for each connection.

## HTTP Requests/Responses

- Every HTTP request has a request line, headers, and a body.
  - First line: `GET / HTTP/1.1` (method, URI [uniform resource identifier], and version).
    - This ends with a CRLF (carriage return and line feed).
    - URL (Uniform Resource Locator) is not the same as URI, URL is a subset of URI.
  - Headers: Key-value pairs that provide additional information about the request, like `Host`, `User-Agent`, etc.
    - This ends with a CRLF as well.
  - Body: The body of the request, which is optional.
- HTTP responses have a status line, headers, and a body.
  - Status line: `HTTP/1.1 200 OK` (version, status code, and reason phrase).
  - Headers: Key-value pairs that provide additional information about the response.
  - Body: The body of the response, which is optional.

## Handle concurrent requests

- Thread Pool: A group of threads that are ready to handle incoming requests.
  - Limiting the number of threads in the pool can prevent the server from crashing due to too many requests (denial of service attack).
- Fork-join model: The main thread creates a pool of threads, and each thread handles a request.
- Single-threaded asynchronous model: The main thread handles all requests asynchronously.
- Multi-threaded asynchronous model: The main thread creates a pool of threads, and each thread handles requests asynchronously.
