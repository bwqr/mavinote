use aes_gcm_siv::{Aes256GcmSiv, KeyInit, Nonce, aead::Aead};
use base64ct::{Base64, Encoding};
use serde::Serialize;
use sqlx::{Sqlite, pool::PoolConnection};
use x25519_dalek::{StaticSecret, PublicKey};

use crate::{models::StoreKey, storage::db, Error};

#[derive(Clone, Debug, Serialize)]
pub enum CryptoError {
    Base64Decode,
    InvalidLength,
    Decrypt,
    Encrypt,
}

pub struct DeviceCipher {
    cipher: Aes256GcmSiv,
}

impl DeviceCipher {
    pub fn try_from_key(privkey: &StaticSecret, pubkey: &str) -> Result<Self, CryptoError> {
        let pubkey = PublicKey::from(parse_key(pubkey)?);
        let shared_secret = privkey.diffie_hellman(&pubkey);
        let cipher = Aes256GcmSiv::new_from_slice(&shared_secret.to_bytes()).unwrap();

        Ok(DeviceCipher { cipher })
    }

    pub fn encrypt(&self, message: &str) -> Result<String, CryptoError> {
        let nonce = Nonce::from_slice(b"unique nonce");

        let ciphertext = self.cipher.encrypt(nonce, message.as_bytes())
            .map_err(|_| CryptoError::Encrypt)?;

        Ok(Base64::encode_string(ciphertext.as_slice()))
    }

    pub fn decrypt(&self, encoded: &str) -> Result<String, CryptoError> {
        let nonce = Nonce::from_slice(b"unique nonce");

        let bytes = Base64::decode_vec(encoded)
            .map_err(|_| CryptoError::Base64Decode)?;
        let bytes = self.cipher.decrypt(nonce, bytes.as_slice())
            .map_err(|_| CryptoError::Decrypt)?;

        String::from_utf8(bytes)
            .map_err(|_| CryptoError::Decrypt)
    }
}

pub async fn load_privkey(conn: &mut PoolConnection<Sqlite>) -> Result<StaticSecret, Error> {
    let store = db::fetch_value(conn, StoreKey::IdentityPrivKey).await?.unwrap();

    parse_key(&store.value)
        .map(|bytes| StaticSecret::from(bytes))
        .map_err(|e| e.into())
}

fn parse_key(key: &str) -> Result<[u8; 32], CryptoError> {
    let bytes = Base64::decode_vec(key)
        .map_err(|_| CryptoError::Base64Decode)?;

    <Vec<u8> as TryInto::<[u8; 32]>>::try_into(bytes)
        .map_err(|_| CryptoError::InvalidLength)
}
