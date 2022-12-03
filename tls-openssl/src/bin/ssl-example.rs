use anyhow::Result;
use openssl::ssl::{Ssl, SslContext, SslMethod, SslVerifyMode};
use openssl::x509::verify::X509CheckFlags;
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut ctx = SslContext::builder(SslMethod::tls())?;
    // ctx.set_default_verify_paths()?;
    ctx.set_verify(SslVerifyMode::PEER);
    ctx.set_ca_file("./resources/emqx-mqtt/broker.emqx.io-ca.crt")?;
    let ctx = ctx.build();
    let s = TcpStream::connect("broker-cn.emqx.io:8883")?;
    let mut ssl = Ssl::new(&ctx)?;
    // 注释下列代码则不校验hostname
    ssl.param_mut()
        .set_hostflags(X509CheckFlags::SINGLE_LABEL_SUBDOMAINS);
    ssl.param_mut().set_host("broker-cn.emqx.io")?;

    ssl.connect(s)?;
    Ok(())
}
