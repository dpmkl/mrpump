#[allow(dead_code)]
use crate::load_helper::load_certs_wrapped;
use failure::Error;
use rcgen::generate_simple_self_signed;
use rustls::{Certificate, PrivateKey};
use std::io::BufReader;

pub fn rcgen_self_signed(
    subject_alt_names: Vec<String>,
) -> Result<(Certificate, PrivateKey), Error> {
    let x509 = generate_simple_self_signed(subject_alt_names);
    let cert = x509.serialize_pem();
    let mut cert = BufReader::new(cert.as_bytes());
    let mut certs = load_certs_wrapped("rcgen", &mut cert)?;
    let cert = certs.remove(0);

    let key = x509.serialize_private_key_pem();
    let mut key = BufReader::new(key.as_bytes());
    let key_rsa = match rustls::internal::pemfile::rsa_private_keys(&mut key) {
        Ok(keys) => Some(keys[0].clone()),
        Err(_) => None,
    };

    let key_pkcs8 = match rustls::internal::pemfile::pkcs8_private_keys(&mut key) {
        Ok(keys) => Some(keys[0].clone()),
        Err(_) => None,
    };

    if key_rsa.is_some() {
        return Ok((cert, key_rsa.unwrap()));
    } else {
        return Ok((cert, key_pkcs8.unwrap()));
    }
}
