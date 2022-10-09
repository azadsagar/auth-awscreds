extern crate openssl;
extern crate base64;

use openssl::rsa::{Rsa,Padding};
use openssl::symm::Cipher;

pub fn genkeypairs() -> (String,String) {
    let rsa = Rsa::generate(1024).unwrap();

    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(),"Sagar Barai".as_bytes()).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    (base64::encode(private_key) , base64::encode(public_key))
}

#[allow(dead_code)]
pub fn decrypt_data(private_key_b64: &String, encrypted_data_b64: &String) -> Vec<u8> {
    
    let private_key_pem : Vec<u8> = base64::decode(private_key_b64).unwrap();
    let encrypted_data : Vec<u8> = base64::decode(encrypted_data_b64).unwrap();

    let rsa = Rsa::private_key_from_pem_passphrase(&private_key_pem, "Sagar Barai".as_bytes()).unwrap();

    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];

    let bytes = rsa.private_decrypt(&encrypted_data, &mut buf, Padding::PKCS1_OAEP).unwrap();

    //String::from_utf8(buf).unwrap()
    buf[0..bytes].to_vec()

}