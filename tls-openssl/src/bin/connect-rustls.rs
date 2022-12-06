use anyhow::Result;
use custom_utils::logger::LevelFilter::Trace;
use openssl::ssl;
use std::net::TcpStream;

fn main() -> Result<()> {
    custom_utils::logger::logger_stdout(Trace);
    let mut builder = ssl::SslConnector::builder(ssl::SslMethod::tls())?;
    builder.set_verify(ssl::SslVerifyMode::PEER);
    builder.set_ca_file("./resources/ecdsa/ca.cert")?;
    // connect to server
    let connector = builder.build();
    let s = TcpStream::connect("127.0.0.1:6000")?;
    let config = connector.configure()?;
    let config = config
        .verify_hostname(false)
        .use_server_name_indication(false);
    config.connect("", s)?;
    Ok(())
}
