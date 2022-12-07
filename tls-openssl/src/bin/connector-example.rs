use anyhow::Result;
use custom_utils::logger::info;
use custom_utils::logger::LevelFilter::Trace;
use openssl::ssl;
use std::net::TcpStream;

fn main() -> Result<()> {
    custom_utils::logger::logger_stdout(Trace);
    let mut builder = ssl::SslConnector::builder(ssl::SslMethod::tls())?;
    builder.set_verify(ssl::SslVerifyMode::PEER);
    builder.set_ca_file("./resources/emqx-mqtt/broker.emqx.io-ca.crt")?;
    // connect to server
    let connector = builder.build();
    let s = TcpStream::connect("broker-cn.emqx.io:8883")?;
    connector.connect("broker-cn.emqx.io", s)?;

    info!("connect success");
    Ok(())
}
