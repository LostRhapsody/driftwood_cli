use base64::{engine::general_purpose, Engine};
use rsa::{pkcs8::LineEnding, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs8::EncodePublicKey};
use rand::rngs::OsRng;

pub fn generate_key_pair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

pub fn get_public_key_pem(public_key: &RsaPublicKey) -> String {
    public_key.to_public_key_pem(LineEnding::LF).expect("failed to encode public key")
}

pub fn decrypt_token(encrypted_token: &str, private_key: &RsaPrivateKey) -> String {
    let enc_data = general_purpose::STANDARD.decode(encrypted_token)
        .expect("failed to decode base64");
    let dec_data = private_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    String::from_utf8(dec_data).expect("failed to parse decrypted data")
}