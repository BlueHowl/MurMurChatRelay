use aes_gcm::{Aes256Gcm};
use aes_gcm::aead::{NewAead, generic_array::GenericArray, Aead};
use base64::{Engine as _, engine::general_purpose};
use rand::{RngCore, thread_rng};

pub struct AesGcmEncryptor {
    cipher: Aes256Gcm
}

impl AesGcmEncryptor {
    pub fn new(aes_key: String) -> AesGcmEncryptor {

        AesGcmEncryptor {
            cipher: Aes256Gcm::new(GenericArray::from_slice(&*general_purpose::STANDARD.decode(aes_key).unwrap()))
        }
    }

    pub fn encrypt_string(&self, plaintext: String) -> Result<String, ()> {
        let mut nonce = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce);

        let ciphertext = self.cipher.encrypt(&nonce.into(), plaintext.as_bytes()).unwrap();
        let mut result = nonce.to_vec();
        result.extend_from_slice(&*ciphertext);

        Ok(general_purpose::STANDARD.encode(&result))
    }

    pub fn decrypt_string(&self, encrypted: &String) -> Result<String, ()> {
        let decoded = general_purpose::STANDARD.decode(encrypted.trim()).unwrap();
        let nonce = &decoded[..12];
        let ciphertext = &decoded[12..decoded.len()];

        let uncryptedtext = self.cipher.decrypt(nonce.into(), ciphertext).unwrap();

        Ok(String::from_utf8(uncryptedtext).unwrap())
    }

}
