use crate::crypto_ext::{decrypt, encrypt, setup_encryption, sign, verify};
use hex::{self, FromHex, ToHex};

#[test]
fn encryption() {
    // path needs to be accessible by user with write permission for initial setup
    let relative_path_to_working_directory_for_storing_encryption_parameters = "/test/encryption_parameters/";
    // it will read encryption params like public, private keys and passphrase or create them
    let params = setup_encryption(Some(relative_path_to_working_directory_for_storing_encryption_parameters)).unwrap();

    //maximum 501 bytes at once to be encrypted
    let data = "Some random textSome random textSome random textSome random textSome random textSome random textSome random textSomeeSome random textSome random textSome random textSome random textSome random textSome random textSome random textSomeeSome random textSome random textSome random textSome random textSome random textSome random textSome random textSomeeSome random textSome random textSome random textSome random textSome random textSome random textSome random textSomee123textSomee123textSomee123textSo";
    println!("data len: {}", data.as_bytes().len());
    let encrypted_u8 = encrypt(params.public_key.as_str(), data.as_bytes());

    let decrypted_u8 = decrypt(params.private_key.as_str(), params.passphrase.as_str(), encrypted_u8.as_ref());

    let decrypted = String::from_utf8(decrypted_u8).unwrap();

    assert_eq!(data.to_string(), decrypted.replace('\0', ""));
}

#[test]
fn signing() {
    // path needs to be accessible by user with write permission for initial setup
    let relative_path_to_working_directory_for_storing_encryption_parameters = "/test/encryption_parameters/";
    // it will read encryption params like public, private keys and passphrase or create them
    let params = setup_encryption(Some(relative_path_to_working_directory_for_storing_encryption_parameters)).unwrap();

    let data = "some text";
    let data_as_hex = hex::encode(data);
    let expected_signature = "9169fe249953094a4edf6f478fc17ce7d6316222d343b34aa8f823a618e29651217940c5d1bdec14e6e1b62b22bfd300dd8c768f6fd440812450bc035c8fe0540d491482f3e346f6a6069d6c59ece7b450fad6018226b99fe767dd436cc0a13e57ece8120fb48bc1d925f11392e21199fbec9b85dd93518bd81e2753ab72709f25e2dce8a686a5061c1c558bf813983d03f9c7fb58a912b6f11e13a9554d258258b62a7b9d3b13ee3b87025e24d598da75c140eae8188348d38691125ce316facead435d965cd04c7a4656ef64a07e98834160e0f6c3f5cd9e6293347b00a6b23ee22cd804c64ed365c287be687809088b510a66f47d5dd1cf8dadf82b26f88916010efb78a12e0cb2c0efee8488e64a720a643bc562baa2e397d597f8711414e57e458f3ed56eb3688ee628f8d9a190079034643ae173f6eefff906df82089e97584e01c9952c6e4cca89a0bf2000f085e2e76b4a9d0464cb1716476bd84994540ac919f0b984dbbde7464fb148c34e45e237a3b86ab9bb816d61f944318896138f4ad7979859a75feb14415b38005157af955ef98e1e1b71b2c38d4690ba9236514d47fae4d5a0fc7fa5057455ad99efc2a75531b853472a627540ab279a0e443b3ba90bd3a9bf40465af3f51b379b33379aef84e7c4564868c8c8162b260fde0b48472dc56b74729b1f6d2760deb9f26a16d5cf1e9af3b295cfbaebd892fe";
    let signature =
        sign(
            params.private_key.as_str(),
            params.passphrase.as_str(),
            data_as_hex.as_bytes());

    assert_eq!(expected_signature, signature);
}

#[test]
fn verification() {
    let relative_path_to_working_directory_for_storing_encryption_parameters = "/test/encryption_parameters/";
    // it will read encryption params like public, private keys and passphrase or create them
    let params = setup_encryption(Some(relative_path_to_working_directory_for_storing_encryption_parameters)).unwrap();

    let data = "some text";
    let data_as_hex = hex::encode(data);
    let signature = "9169fe249953094a4edf6f478fc17ce7d6316222d343b34aa8f823a618e29651217940c5d1bdec14e6e1b62b22bfd300dd8c768f6fd440812450bc035c8fe0540d491482f3e346f6a6069d6c59ece7b450fad6018226b99fe767dd436cc0a13e57ece8120fb48bc1d925f11392e21199fbec9b85dd93518bd81e2753ab72709f25e2dce8a686a5061c1c558bf813983d03f9c7fb58a912b6f11e13a9554d258258b62a7b9d3b13ee3b87025e24d598da75c140eae8188348d38691125ce316facead435d965cd04c7a4656ef64a07e98834160e0f6c3f5cd9e6293347b00a6b23ee22cd804c64ed365c287be687809088b510a66f47d5dd1cf8dadf82b26f88916010efb78a12e0cb2c0efee8488e64a720a643bc562baa2e397d597f8711414e57e458f3ed56eb3688ee628f8d9a190079034643ae173f6eefff906df82089e97584e01c9952c6e4cca89a0bf2000f085e2e76b4a9d0464cb1716476bd84994540ac919f0b984dbbde7464fb148c34e45e237a3b86ab9bb816d61f944318896138f4ad7979859a75feb14415b38005157af955ef98e1e1b71b2c38d4690ba9236514d47fae4d5a0fc7fa5057455ad99efc2a75531b853472a627540ab279a0e443b3ba90bd3a9bf40465af3f51b379b33379aef84e7c4564868c8c8162b260fde0b48472dc56b74729b1f6d2760deb9f26a16d5cf1e9af3b295cfbaebd892fe";

    let result = verify(
        params.private_key.as_str(),
        params.passphrase.as_str(),
        data_as_hex.as_bytes(),
        signature
    );

    assert!(result);
}