use aes_gcm::aead::{Aead, KeyInit, generic_array::GenericArray};
use aes_gcm::Aes256Gcm;
use rand::Rng;
use rand::rngs::OsRng;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

const NONCE_SIZE: usize = 12;
const PBKDF2_ITERATIONS: u32 = 100_000;

pub struct Key;

impl Key {
    // extract 256-bit key from a password using PBKDF2-SHA256
    pub fn derive_key(password: &[u8]) -> [u8; 32] {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password, &[], PBKDF2_ITERATIONS, &mut key);

        key
    }

    // encrypt data using AES-256-GCM + randomly generated nonce
    pub fn encrypt(data: &[u8], password: &[u8]) -> Vec<u8> {
        let key = Self::derive_key(password);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
        let mut rng = OsRng;
        let mut nonce = [0u8; NONCE_SIZE];
        rng.fill(&mut nonce);

        let nonce = GenericArray::from_slice(&nonce);
        let ciphertext = cipher.encrypt(nonce, data).expect("Encryption failed");
        let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        
        result
    }

    // decrypt data using AES-256-GCM, extracting nonce from encrypted data
    pub fn decrypt(encrypted_data: &[u8], password: &[u8]) -> Vec<u8> {
        let key = Self::derive_key(password);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
        let nonce = GenericArray::from_slice(&encrypted_data[..NONCE_SIZE]);
        let ciphertext = &encrypted_data[NONCE_SIZE..];

        cipher.decrypt(nonce, ciphertext)
            .expect("Decryption failed: The provided key may be incorrect. Please check your key or create a new file using the \"create-file\" command.")
    }
}
