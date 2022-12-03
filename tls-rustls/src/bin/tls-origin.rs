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
    let inter_ca = load_pem_certs_by_path("./resources/ecdsa/inter.cert")?;
    let mut _inter = RootCertStore::empty();
    for ca in inter_ca {
        _inter.add(&ca)?;
    }

    let verifier_to_client = AllowAnyAuthenticatedClient::new(root.clone());
    let server_config = Arc::new(
        ServerConfig::builder()
            .with_cipher_suites(&[
                // TLS1.3 suites
                TLS13_AES_256_GCM_SHA384,
                TLS13_AES_128_GCM_SHA256,
                TLS13_CHACHA20_POLY1305_SHA256,
            ])
            .with_safe_default_kx_groups()
            .with_protocol_versions(&[&TLS13])?
            .with_client_cert_verifier(verifier_to_client)
            // .with_no_client_auth()
            .with_single_cert(
                load_pem_certs_by_path("./resources/ecdsa/end.fullchain")?,
                load_pkcs8_key("./resources/ecdsa/end.key")?,
            )
            .unwrap(),
    );

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

    let client = ClientConnection::new(Arc::clone(&client_config), dns_name("localhost")).unwrap();
    let server = ServerConnection::new(Arc::clone(&server_config)).unwrap();

    // do_handshake_until_error(&mut client, &mut server)?;
    // transfer_tcp(client, server).await;
    transfer_tcp_std(client, server);
    Ok(())
}

pub fn dns_name(name: &'static str) -> rustls::ServerName {
    name.try_into().unwrap()
}

pub fn do_handshake_until_error(
    client: &mut ClientConnection,
    server: &mut ServerConnection,
) -> Result<()> {
    while server.is_handshaking() || client.is_handshaking() {
        transfer(client, server);
        server.process_new_packets()?;
        transfer(server, client);
        client.process_new_packets()?;
    }
    Ok(())
}

pub fn transfer(
    left: &mut (impl DerefMut + Deref<Target = ConnectionCommon<impl SideData>>),
    right: &mut (impl DerefMut + Deref<Target = ConnectionCommon<impl SideData>>),
) -> usize {
    let mut buf = [0u8; 262144];
    let mut total = 0;

    while left.wants_write() {
        let sz = {
            let into_buf: &mut dyn io::Write = &mut &mut buf[..];
            left.write_tls(into_buf).unwrap()
        };
        total += sz;
        if sz == 0 {
            return total;
        }

        let mut offs = 0;
        loop {
            let from_buf: &mut dyn io::Read = &mut &buf[offs..sz];
            offs += right.read_tls(from_buf).unwrap();
            if sz == offs {
                break;
            }
        }
    }

    total
}

fn transfer_tcp_std(mut client: ClientConnection, mut server: ServerConnection) {
    println!("transfer_tcp");
    thread::spawn(move || {
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
    });

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
}
