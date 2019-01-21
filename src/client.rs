#![allow(dead_code)]
use std::{net::SocketAddr, sync::Arc};
use tokio::{io, prelude::Future};
use tokio_rustls::{
    rustls::{ClientConfig, ClientSession},
    TlsConnector, TlsStream,
};
use tokio_tcp::TcpStream;

fn test() {
    let addr = "0.0.0.0:7337".parse::<SocketAddr>().unwrap();
    let config = ClientConfig::new();
    client(addr, config, |_stream| Ok(()));
}

fn client<F>(addr: SocketAddr, tls_config: ClientConfig, handler: F)
where
    F: Fn(TlsStream<TcpStream, ClientSession>) -> Result<(), io::Error> + Send + Sync + 'static,
{
    let handler = Arc::new(handler);
    let tcp = TcpStream::connect(&addr);
    let tls = TlsConnector::from(Arc::new(tls_config));
    let task = tcp
        .and_then(move |stream| {
            let domain = webpki::DNSNameRef::try_from_ascii_str("domain").unwrap();
            tls.connect(domain, stream)
        })
        .and_then(move |stream| handler(stream))
        .map_err(|err| eprint!("{:?}", err));
    tokio::run(task.map_err(drop));
}
