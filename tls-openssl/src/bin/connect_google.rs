/// This is the simplest possible client using rustls that does something useful:
/// it accepts the default configuration, loads some root certs, and then connects
/// to google.com and issues a basic HTTP request.  The response is printed to stdout.
///
/// It makes use of rustls::Stream to treat the underlying TLS connection as a basic
/// bi-directional stream -- the underlying IO is performed transparently.
///
/// Note that `unwrap()` is used to deal with networking errors; this is not something
/// that is sensible outside of example code.
use std::sync::Arc;

use openssl::ssl;
use openssl::x509::store::X509StoreBuilder;
use std::convert::TryInto;
use std::io::{stdout, Read, Write};
use std::net::TcpStream;

fn main() -> anyhow::Result<()> {
    let mut builder = X509StoreBuilder::new().unwrap();
    let mut builder = ssl::SslConnector::builder(ssl::SslMethod::tls())?;
    builder.set_verify(ssl::SslVerifyMode::NONE);
    let connector = builder.build();
    let mut sock = TcpStream::connect("google.com:443").unwrap();
    let mut tls = connector.connect("google.com", sock)?;
    tls.write_all(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: google.com\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
        )
        .as_bytes(),
    )
    .unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    stdout().write_all(&plaintext).unwrap();

    Ok(())
}
