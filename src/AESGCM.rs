use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::rngs::{OsRng};

pub struct AesGcmEncryptor {
    key: String,
}

impl AesGcmEncryptor {
    pub fn new(aesKey: String) -> AesGcmEncryptor {
        AesGcmEncryptor {
            key: aesKey,
        }
    }

    fn encrypt(key: String, nonce: &Nonce, plaintext: &[u8], aad: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(key);
        cipher.encrypt(nonce, plaintext, aad).expect("encryption failure")
    }

    fn decrypt(key: String, nonce: &Nonce, ciphertext: &[u8], aad: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(key);
        cipher.decrypt(nonce, ciphertext, aad).expect("decryption failure")
    }
}