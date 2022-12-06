use anyhow::Result;
use custom_utils::tls::{init_root_certs_by_path, load_pem_certs_by_path, load_pkcs8_key};
use rustls::cipher_suite::{
    TLS13_AES_128_GCM_SHA256, TLS13_AES_256_GCM_SHA384, TLS13_CHACHA20_POLY1305_SHA256,
};
use rustls::server::AllowAnyAuthenticatedClient;
use rustls::version::TLS13;
use rustls::{
    ClientConfig, ClientConnection, ConnectionCommon, RootCertStore, ServerConfig,
    ServerConnection, SideData,
};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{io, thread};
use tls_rustls::NoCertificateVerification;

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() -> Result<()> {
    let root = init_root_certs_by_path("./resources/ecdsa/ca.cert")?;
    let _verifier = Arc::new(NoCertificateVerification);
    let client_config = Arc::new(
        ClientConfig::builder()
            .with_cipher_suites(&[
                // TLS1.3 suites
                TLS13_AES_256_GCM_SHA384,
                TLS13_AES_128_GCM_SHA256,
                TLS13_CHACHA20_POLY1305_SHA256,
            ])
            .with_safe_default_kx_groups()
            .with_protocol_versions(&[&TLS13])?
            // .with_custom_certificate_verifier(_verifier)
            .with_root_certificates(root)
            .with_single_cert(
                load_pem_certs_by_path("./resources/ecdsa/client.fullchain")?,
                load_pkcs8_key("./resources/ecdsa/client.key")?,
            )?, // .with_no_client_auth(),
    );

    let mut client =
        ClientConnection::new(Arc::clone(&client_config), dns_name("localhost")).unwrap();

    let mut client_stream = std::net::TcpStream::connect("127.0.0.1:6000").unwrap();
    println!("client connected success!");
    match client.complete_io(&mut client_stream) {
        Ok(val) => {
            println!("client {} {}", val.0, val.1);
        }
        Err(e) => {
            println!("{}", e.to_string());
        }
    }
    client_stream.write(&[0, 0]).unwrap();
    sleep(Duration::from_secs(100));
    Ok(())
}

pub fn dns_name(name: &'static str) -> rustls::ServerName {
    name.try_into().unwrap()
}
