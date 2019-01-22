#![allow(dead_code)]
use super::util::{load_certs, load_private_key};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::TcpListener,
    prelude::{Future, Stream},
};
use tokio_rustls::{
    rustls::{NoClientAuth, ServerConfig, ServerSession},
    TlsAcceptor, TlsStream,
};
use tokio_tcp::TcpStream;

fn test() {
    let addr = "0.0.0.0:7337".parse::<SocketAddr>().unwrap();
    let mut config = ServerConfig::new(NoClientAuth::new());
    config
        .set_single_cert(load_certs("").unwrap(), load_private_key("").unwrap())
        .expect("invalid key or certificate");
    server(addr, config, |_stream| Ok(()));
}

pub fn server<F>(addr: SocketAddr, tls_config: ServerConfig, handler: F)
where
    F: Fn(TlsStream<TcpStream, ServerSession>) -> Result<(), std::io::Error>
        + Send
        + Sync
        + 'static,
{
    let handler = Arc::new(handler);
    let tcp = TcpListener::bind(&addr).unwrap();
    let tls = TlsAcceptor::from(Arc::new(tls_config));
    let task = tcp.incoming().for_each(move |stream| {
        let handler = handler.clone();
        let ssl = tls
            .accept(stream)
            .and_then(move |stream| handler(stream))
            .map_err(move |err| println!("Error: {:?} - {:?}", err, addr));
        tokio::spawn(ssl);

        Ok(())
    });
    tokio::run(task.map_err(drop));
}
