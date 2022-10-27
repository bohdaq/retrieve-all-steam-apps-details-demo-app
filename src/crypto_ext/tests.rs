use crate::crypto_ext::{decrypt, encrypt, setup_encryption};

extern crate openssl;

use openssl::rsa::{Rsa, Padding};

#[test]
fn encryption() {
    let params = setup_encryption(Some("/test/encryption_parameters/")).unwrap();

    //maximum 501 bytes at once to be encrypted
    let data = "Some random textSome random textSome random textSome random textSome random textSome random textSome random textSomeeSome random textSome random textSome random textSome random textSome random textSome random textSome random textSomeeSome random textSome random textSome random textSome random textSome random textSome random textSome random textSomeeSome random textSome random textSome random textSome random textSome random textSome random textSome random textSomee123textSomee123textSomee123textSo";
    println!("data len: {}", data.as_bytes().len());
    let encrypted_u8 = encrypt(params.public_key.as_str(), data.as_bytes());

    let decrypted_u8 = decrypt(params.private_key.as_str(), params.passphrase.as_str(), encrypted_u8.as_ref());

    let decrypted = String::from_utf8(decrypted_u8).unwrap();

    assert_eq!(data.to_string(), decrypted.replace('\0', ""));
}