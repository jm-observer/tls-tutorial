use anyhow::Result;
use std::net::TcpStream;
use std::sync::Arc;
use tls_rustls::complete_prior_io;

pub fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();
    let root_store =
        custom_utils::tls::init_root_certs_by_path("./resources/emqx-mqtt/broker.emqx.io-ca.crt")?;
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let server_name = "broker-cn.emqx.io".try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
    let mut sock = TcpStream::connect("broker-cn.emqx.io:8883").unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    complete_prior_io(&mut tls)?;

    if let Some(certs) = tls.conn.peer_certificates() {
        let mut index = 0;
        for cert in certs {
            std::fs::write(
                format!("./resources/peer/cert{}.crt", index),
                cert.0.as_slice(),
            )?;
            index += 1;
        }
    }
    Ok(())
}
