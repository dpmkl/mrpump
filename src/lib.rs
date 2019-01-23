#[macro_use]
extern crate failure;

mod acl;
pub use crate::acl::AccessControl;

mod client;
pub use crate::client::client as spawn_client;
mod server;
pub use crate::server::server as spawn_server;

mod load_helper;
mod rcgen_helper;

pub mod util {

    #[derive(Debug, Fail)]
    pub enum TlsConfigError {
        #[fail(display = "Could not read certificate file '{}'!", file_name)]
        CertificateParsingError { file_name: String },
        #[fail(display = "Could not read private key file '{}'!", file_name)]
        PrivateKeyParsingError { file_name: String },
    }

    pub use crate::load_helper::{load_certs, load_private_key};
    pub use crate::rcgen_helper::rcgen_self_signed;
}
