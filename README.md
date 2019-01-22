# Mr Pump #

MrPump is a set of two functions that spawn a tokio-rustls based client or server.
Each runs the handler, passed as argument, for every accepted server session or the client session.
The handler returns directly to the underlying future.
```rust
    Fn(TlsStream<TcpStream, ServerSession>) -> Result<(), std::io::Error>
```

