extern crate openssl;

use std::fs;

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;


lazy_static! {
    static ref PKCS8KEY: Vec<u8> = load_key_file("PKCS8KEY");
    static ref ASN1KEY: Vec<u8> = load_key_file("ASN1KEY");
}

fn load_key_file(file_name: &str) -> Vec<u8> {
    let key = fs::read_to_string(file_name).expect("Key file not found");
    base64::decode(key.replace("\n", "")).expect("Invalid key file")
}

pub fn sign_with_pkcs8_key(data: String) -> String {
    let pkey = PKey::private_key_from_der(&PKCS8KEY).unwrap();

    let mut signer = Signer::new(MessageDigest::sha1(), pkey.as_ref()).unwrap();
    signer.update(data.as_bytes()).unwrap();

    base64::encode(signer.sign_to_vec().unwrap())
}

pub fn sign_with_asn1_key(data: String) -> String {
    let rsa = Rsa::private_key_from_der(&ASN1KEY).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    let mut signer = Signer::new(MessageDigest::md5(), pkey.as_ref()).unwrap();
    signer.update(data.as_bytes()).unwrap();

    hex::encode(signer.sign_to_vec().unwrap())
}
