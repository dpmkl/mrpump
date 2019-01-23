# Mr Pump #

MrPump is a set of two functions that spawn a tokio-rustls based client or server.
Each runs the handler, passed as argument, for every accepted server session or the client session.
The handler returns directly to the underlying future.
```rust
    pub fn server<F>(addr: SocketAddr, tls_config: ServerConfig, acl: AccessControl, handler: F)
    where
        F: Fn(TlsStream<TcpStream, ServerSession>) -> Result<(), std::io::Error>
            + Send
            + Sync
            + 'static
```

 :warning: This is personal playground toy project, nothing but bugs can be guaranteed! :warning:

