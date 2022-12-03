use anyhow::Result;
use custom_utils::tls::{init_root_certs_by_path, load_pem_certs_by_path};
use rustls::client::{ServerCertVerifier, WebPkiVerifier};
use std::time::SystemTime;

fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();
    assert!(verify_server(
        "./resources/ecdsa/ca.cert",
        "./resources/ecdsa/end.fullchain",
        "localhost"
    )
    .is_ok());
    assert!(verify_server(
        "./resources/ecdsa/ca.cert",
        "./resources/ecdsa/client.fullchain",
        "localhost"
    )
    .is_err());
    Ok(())
}

fn verify_server(ca_path: &str, server_fullchain_path: &str, server_name: &str) -> Result<()> {
    let root_store = init_root_certs_by_path(ca_path)?;
    let verifier = WebPkiVerifier::new(root_store, None);
    let chain = load_pem_certs_by_path(server_fullchain_path)?;
    let (end_entity, intermediates) = chain.split_first().unwrap();
    const SCTS: &[&[u8]] = &[];
    const OCSP_RESPONSE: &[u8] = &[];
    let _result = verifier.verify_server_cert(
        end_entity,
        intermediates,
        &server_name.try_into()?,
        &mut SCTS.iter().copied(),
        OCSP_RESPONSE,
        SystemTime::now(),
    )?;
    Ok(())
}
