extern crate mrpump;
extern crate rustls;

use mrpump::{spawn_server, util::rcgen_self_signed, AccessControl};
use rustls::{NoClientAuth, ServerConfig};
use std::net::SocketAddr;

fn main() {
    let addr = "0.0.0.0:7337".parse::<SocketAddr>().unwrap();
    let alt_names = vec!["test".to_string(), "localhost".to_string()];
    let (cert, key) = rcgen_self_signed(alt_names).unwrap();
    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(vec![cert], key).unwrap();
    spawn_server(addr, config, AccessControl::None, |_stream| Ok(()));
}
