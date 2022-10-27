use crate::crypto_ext::setup_encryption;

#[test]
fn encryption() {
    let params = setup_encryption(Some("/test/encryption_parameters/")).unwrap();
}