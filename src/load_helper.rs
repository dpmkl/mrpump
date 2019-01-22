use failure::Error;
use rustls::{Certificate, PrivateKey};
use std::{
    fs,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Fail)]
pub enum TlsConfigError {
    #[fail(display = "Could not read certificate file '{}'!", file_name)]
    CertificateParsingError { file_name: String },
    #[fail(display = "Could not read private key file '{}'!", file_name)]
    PrivateKeyParsingError { file_name: String },
}

fn load_certs_wrapped<R: BufRead>(
    file_name: &str,
    mut reader: &mut R,
) -> Result<Vec<Certificate>, TlsConfigError> {
    match rustls::internal::pemfile::certs(&mut reader) {
        Ok(certs) => Ok(certs),
        Err(_) => Err(TlsConfigError::CertificateParsingError {
            file_name: String::from_str(file_name).unwrap(),
        }),
    }
}

pub fn load_certs(file_name: &str) -> Result<Vec<Certificate>, Error> {
    let cert_file = fs::File::open(file_name)?;
    let mut reader = BufReader::new(cert_file);
    let certs = load_certs_wrapped(file_name, &mut reader)?;
    Ok(certs)
}

fn load_private_key_rsa<R: BufRead>(
    file_name: &str,
    mut reader: &mut R,
) -> Result<Vec<PrivateKey>, TlsConfigError> {
    match rustls::internal::pemfile::rsa_private_keys(&mut reader) {
        Ok(keys) => Ok(keys),
        Err(_) => Err(TlsConfigError::PrivateKeyParsingError {
            file_name: String::from_str(file_name).unwrap(),
        }),
    }
}

fn load_private_key_pkcs8<R: BufRead>(
    file_name: &str,
    mut reader: &mut R,
) -> Result<Vec<PrivateKey>, TlsConfigError> {
    match rustls::internal::pemfile::pkcs8_private_keys(&mut reader) {
        Ok(keys) => Ok(keys),
        Err(_) => Err(TlsConfigError::PrivateKeyParsingError {
            file_name: String::from_str(file_name).unwrap(),
        }),
    }
}

pub fn load_private_key(file_name: &str) -> Result<PrivateKey, Error> {
    let rsa_keys = {
        let key_file = fs::File::open(file_name)?;
        let mut reader = BufReader::new(key_file);
        load_private_key_rsa(file_name, &mut reader)?
    };

    let pkcs8_keys = {
        let key_file = fs::File::open(file_name)?;
        let mut reader = BufReader::new(key_file);
        load_private_key_pkcs8(file_name, &mut reader)?
    };

    if pkcs8_keys.is_empty() {
        Ok(rsa_keys[0].clone())
    } else {
        Ok(pkcs8_keys[0].clone())
    }
}
