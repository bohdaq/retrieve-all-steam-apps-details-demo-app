use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use sha256::digest;
use openssl::rsa::{Padding, RsaPrivateKeyBuilder};
use openssl::rsa::Rsa;
use openssl::symm::Cipher;

fn setup_encryption() -> Result<(), String> {
    let passphrase_path = ".passphrase";
    let boxed_passphrase = get_or_create_passphrase(passphrase_path);
    if boxed_passphrase.is_err() {
        return Err(boxed_passphrase.err().unwrap());
    }
    let passphrase = boxed_passphrase.unwrap();

    let boxed_keys = get_or_create_private_public_keys(passphrase);
    if boxed_keys.is_err() {
        return Err(boxed_keys.err().unwrap());
    }

    Ok(())
}

fn get_or_create_passphrase(path: &str) -> Result<String, String> {

    let boxed_passphrase = generate_passphrase();
    if boxed_passphrase.is_err() {
        let message = boxed_passphrase.err().unwrap();
        return Err(message)
    }

    let passphrase = boxed_passphrase.unwrap();

    let boxed_passphrase = read_or_create_and_write(path, passphrase.as_str());
    if boxed_passphrase.is_err() {
        let message = boxed_passphrase.err().unwrap();
        return Err(message)
    }

    let passphrase = boxed_passphrase.unwrap();
    Ok(passphrase)
}

fn read_or_create_and_write(path: &str, content: &str) -> Result<String, String> {
    let does_passphrase_exist = does_file_exist(path);
    return if does_passphrase_exist {
        let boxed_read = read_file(path);
        if boxed_read.is_err() {
            return Err(boxed_read.err().unwrap());
        }
        let passphrase = boxed_read.unwrap();
        Ok(passphrase)
    } else {
        let boxed_create = create_file(path);
        if boxed_create.is_err() {
            let message = boxed_create.err().unwrap();
            return Err(message)
        }

        let boxed_write = write_file(path, content.as_bytes());
        if boxed_write.is_err() {
            let message = boxed_write.err().unwrap();
            return Err(message)
        }
        Ok(content.to_string())
    }
}

fn create_file(path: &str) -> Result<File, String>  {
    let boxed_file = OpenOptions::new()
        .read(false)
        .write(false)
        .create(true)
        .truncate(false)
        .open(path);

    if boxed_file.is_err() {
        let message = format!("unable to create file: {}", boxed_file.err().unwrap());
        return Err(message)
    }

    let file = boxed_file.unwrap();
    Ok(file)
}

fn does_file_exist(path: &str) -> bool {
    let file_exists = Path::new(path).is_file();
    file_exists
}

fn read_file(path: &str) -> Result<String, String> {
    let mut file_contents : String = "".to_string();
    let boxed_open = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .truncate(false)
        .open(path);
    if boxed_open.is_err() {
        let message = format!("unable to read from file: {}", boxed_open.err().unwrap());
        return Err(message)
    }

    let mut file = boxed_open.unwrap();

    let boxed_read = file.read_to_string(&mut file_contents);
    if boxed_read.is_err() {
        let message = format!("unable to read from file: {}", boxed_read.err().unwrap());
        return Err(message)
    }

    Ok(file_contents)
}

fn write_file(path: &str, file_content: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .truncate(false)
        .open(path)
        .unwrap();
    let boxed_write = file.write_all(file_content);
    if boxed_write.is_err() {
        let message = format!("unable to write to file: {}", boxed_write.err().unwrap());
        return Err(message)
    }
    Ok(())
}

fn overwrite_file(path: &str, file_content: &[u8]) -> Result<(), String>{
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .truncate(true)
        .open(path)
        .unwrap();
    let boxed_write = file.write_all(file_content);
    if boxed_write.is_err() {
        let message = format!("unable to overwrite to file: {}", boxed_write.err().unwrap());
        return Err(message)
    }
    Ok(())
}

fn generate_passphrase() -> Result<String, String> {
    let now = SystemTime::now();
    let boxed_time_in_nanos = now.duration_since(UNIX_EPOCH);
    if boxed_time_in_nanos.is_err() {
        let message = format!("unable to get system time: {}", boxed_time_in_nanos.err().unwrap());
        return Err(message)
    }
    let time_in_nanos = boxed_time_in_nanos.unwrap().as_nanos();
    let hex_time_in_millis = format!("{time_in_nanos:X}");
    let sha_timestamp = digest(hex_time_in_millis);
    Ok(sha_timestamp)
}

fn get_or_create_private_public_keys(passphrase: String) -> Result<(String, String), String> {
    let rsa_size = 4096;
    let rsa = Rsa::generate(rsa_size).unwrap();

    let boxed_private_key = rsa.private_key_to_pem_passphrase(Cipher::aes_256_gcm(), passphrase.as_bytes());
    let private_key  = String::from_utf8(boxed_private_key.unwrap()).unwrap();

    let boxed_public_key = rsa.public_key_to_pem();
    let public_key = String::from_utf8(boxed_public_key.unwrap()).unwrap();

    Ok((private_key.to_string(), public_key.to_string()))
}