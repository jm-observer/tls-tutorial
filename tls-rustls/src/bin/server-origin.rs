#![allow(unused_imports)]
use anyhow::Result;
use custom_utils::logger::LevelFilter::Trace;
use custom_utils::tls::{init_root_certs_by_path, load_pem_certs_by_path, load_pkcs8_key};
use rustls::cipher_suite::{
    TLS13_AES_128_GCM_SHA256, TLS13_AES_256_GCM_SHA384, TLS13_CHACHA20_POLY1305_SHA256,
};
use rustls::server::{AllowAnyAuthenticatedClient, NoClientAuth};
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
    custom_utils::logger::logger_stdout(Trace);
    // let root = init_root_certs_by_path("./resources/ecdsa/ca.cert")?;
    // let verifier_to_client = AllowAnyAuthenticatedClient::new(root.clone());
    let verifier_to_client = Arc::new(NoClientAuth);
    // let server_config = Arc::new(
    //     ServerConfig::builder()
    //         .with_cipher_suites(&[
    //             // TLS1.3 suites
    //             TLS13_AES_256_GCM_SHA384,
    //             TLS13_AES_128_GCM_SHA256,
    //             TLS13_CHACHA20_POLY1305_SHA256,
    //         ])
    //         .with_safe_default_kx_groups()
    //         .with_protocol_versions(&[&TLS13])?
    //         .with_client_cert_verifier(verifier_to_client)
    //         // .with_no_client_auth()
    //         .with_single_cert(
    //             load_pem_certs_by_path("./resources/ecdsa/end.fullchain")?,
    //             load_pkcs8_key("./resources/ecdsa/end.key")?,
    //         )
    //         .unwrap(),
    // );

    let server_config = Arc::new(
        ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(verifier_to_client)
            .with_single_cert(
                load_pem_certs_by_path("./resources/ecdsa/end.fullchain")?,
                load_pkcs8_key("./resources/ecdsa/end.key")?,
            )
            .unwrap(),
    );

    let mut server = ServerConnection::new(Arc::clone(&server_config)).unwrap();

    let listener = std::net::TcpListener::bind("127.0.0.1:6000").unwrap();
    println!("binded success");
    loop {
        if let Ok((mut conn, _addr)) = listener.accept() {
            println!("accept a client!");
            match server.complete_io(&mut conn) {
                Ok(val) => {
                    println!("server {} {}", val.0, val.1);
                }
                Err(e) => {
                    println!("{}", e.to_string());
                }
            }
            sleep(Duration::from_secs(5));
        }
    }
}
