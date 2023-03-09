use aes_gcm::{AeadInPlace, Aes256Gcm, Tag};
use aes_gcm::aead::{NewAead, generic_array::GenericArray, Aead};
use base64::{Engine as _, engine::general_purpose};
use rand::{RngCore, thread_rng};
use regex::Regex;

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

        let mut ciphertext = plaintext.as_bytes().to_vec();
        let tag = self.cipher.encrypt_in_place_detached(&nonce.into(), &[], &mut ciphertext);

        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        result.extend_from_slice(&tag.unwrap());

        Ok(general_purpose::STANDARD.encode(&result))
    }

    /*
    pub fn encrypt(&self, plaintext: String) -> String {
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = GenericArray::from(nonce_bytes);

        let ciphertext = self.cipher.encrypt(&nonce, plaintext.as_bytes())
            .expect("encryption failure!");

        let nonce_string = String::from_utf8(nonce.to_vec()).expect("ciphertext is not valid utf-8");
        let text_string = String::from_utf8(ciphertext.to_vec()).expect("ciphertext is not valid utf-8");

        format!("{} {}", nonce_string, text_string)
    }*/
    /*
    pub fn decrypt(&self, ciphertext: String, iv: String) -> String {
        let plaintext = self.cipher.decrypt(GenericArray::from_slice(iv.as_bytes()), ciphertext.as_bytes())
            .expect("decryption failure!");

        String::from_utf8(plaintext.to_vec()).expect("ciphertext is not valid utf-8")
    }*/


    pub fn decrypt_string(&self, encrypted: &String) -> Result<String, ()> {
        let decoded = general_purpose::STANDARD.decode(encrypted.trim()).unwrap();
        let nonce = &decoded[..12];
        let ciphertext = &decoded[12..decoded.len() - 16];

        let uncryptedtext = self.cipher.decrypt(nonce.into(), ciphertext).unwrap();

        Ok(String::from_utf8(uncryptedtext).unwrap())
    }

}