extern crate mrpump;
extern crate rustls;
mod common;

use mrpump::{AccessControl, spawn_server, util::rcgen_self_signed};
use rustls::{NoClientAuth, ServerConfig};


#[test]
fn server_already_bound() {
    let addr = common::get_loopback_v4_sock_addr_base();
    let alt_names = common::get_subject_alt_names();
    let (cert, key) = rcgen_self_signed(alt_names).unwrap();
    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(vec!(cert), key).unwrap();
    spawn_server(addr, config, AccessControl::None, |_stream| Ok(()));
}
