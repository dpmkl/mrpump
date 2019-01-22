#[macro_use]
extern crate failure;

mod client;
pub use crate::client::client as spawn_client;
mod server;
pub use crate::server::server as spawn_server;

mod load_helper;

pub mod util {
    pub use crate::load_helper::{load_certs, load_private_key, TlsConfigError};
}
