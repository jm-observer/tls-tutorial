use anyhow::Result;
use openssl::ssl;
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut builder = ssl::SslConnector::builder(ssl::SslMethod::tls())?;
    builder.set_verify(ssl::SslVerifyMode::PEER);
    builder.set_ca_file("./resources/emqx-mqtt/broker.emqx.io-ca.crt")?;
    // connect to server
    let connector = builder.build();
    let s = TcpStream::connect("broker-cn.emqx.io:8883")?;
    connector.connect("broker-cn.emqx.io", s)?;
    Ok(())
}
