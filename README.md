# MrPump #

## Overview ##

MrPump is a set of two small functions, for client and server, that spawn a tokio-rustls based client or server
running the handler passed as argument for every server session or for the client session.
The handler returns directly to the underlying future.
```rust
    Fn(TlsStream<TcpStream, ServerSession>) -> Result<(), std::io::Error>
```

