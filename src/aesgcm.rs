use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use aes_gcm::aead::consts::U12;

pub struct AesGcmEncryptor {
    cipher: Aes256Gcm,
    nonce: GenericArray<u8, U12>
}

impl AesGcmEncryptor {
    pub fn new(aes_key: String) -> AesGcmEncryptor {

        AesGcmEncryptor {
            cipher: Aes256Gcm::new(GenericArray::from_slice((&aes_key).as_ref())),
            nonce: GenericArray::from([0u8; 12]) //todo gèrer Nonce? sensé être diff à chaque fois
        }
    }

    pub fn encrypt(&self, plaintext: String) -> String {
        let ciphertext = self.cipher.encrypt(GenericArray::from_slice(self.nonce.as_slice()), plaintext.as_bytes())
            .expect("encryption failure!");

        String::from_utf8(ciphertext.to_vec()).expect("ciphertext is not valid utf-8")
    }

    pub fn decrypt(&self, ciphertext: String) -> String {
        let plaintext = self.cipher.decrypt(GenericArray::from_slice(self.nonce.as_slice()), ciphertext.as_bytes())
            .expect("decryption failure!");

        String::from_utf8(plaintext.to_vec()).expect("ciphertext is not valid utf-8")
    }
}