use aes_gcm_siv::{Aes256GcmSiv, KeyInit, aead::Aead};
use base64ct::{Base64, Encoding};
use serde::Serialize;
use sqlx::{Sqlite, pool::PoolConnection};
use x25519_dalek::{StaticSecret, PublicKey};

use crate::{models::StoreKey, storage::db, Error as NoteError};

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    Base64Decode,
    InvalidLength,
    Decrypt,
    Encrypt,
}

pub struct DeviceCipher {
    pub device_id: i32,
    cipher: Aes256GcmSiv,
}

impl PartialEq for DeviceCipher {
    fn eq(&self, other: &Self) -> bool {
        self.device_id == other.device_id
    }
}

impl std::hash::Hash for DeviceCipher {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        <i32 as std::hash::Hash>::hash(&self.device_id, state)
    }
}

impl Eq for DeviceCipher { }

impl DeviceCipher {
    pub fn try_from_key(device_id: i32, privkey: &StaticSecret, pubkey: &str) -> Result<Self, Error> {
        let pubkey = PublicKey::from(parse_key(pubkey)?);
        let shared_secret = privkey.diffie_hellman(&pubkey);
        let cipher = Aes256GcmSiv::new_from_slice(&shared_secret.to_bytes()).unwrap();

        Ok(DeviceCipher { device_id, cipher })
    }

    pub fn encrypt(&self, message: &str, nonce: [u8; 12]) -> Result<String, Error> {
        // TODO reduce the allocations and encrypt plain text into the vec instead of returning vec.
        let ciphertext = self.cipher.encrypt(&nonce.into(), message.as_bytes())
            .map_err(|_| Error::Encrypt)?;

        let mut bytes = nonce.to_vec();
        bytes.extend_from_slice(ciphertext.as_slice());

        Ok(Base64::encode_string(&bytes))
    }

    pub fn decrypt(&self, encoded: &str) -> Result<String, Error> {
        let bytes = Base64::decode_vec(encoded)
            .map_err(|_| Error::Base64Decode)?;

        // Encoded string must also contain the nonce
        if bytes.len() < 12 {
            return Err(Error::Decrypt);
        }

        let (nonce, bytes) = bytes.split_at(12);
        let bytes = self.cipher.decrypt(nonce.into(), bytes)
            .map_err(|_| Error::Decrypt)?;

        String::from_utf8(bytes)
            .map_err(|_| Error::Decrypt)
    }
}

pub async fn load_privkey(conn: &mut PoolConnection<Sqlite>) -> Result<StaticSecret, NoteError> {
    let store = db::fetch_value(conn, StoreKey::IdentityPrivKey).await?.unwrap();

    parse_key(&store.value)
        .map(|bytes| StaticSecret::from(bytes))
        .map_err(|e| e.into())
}

fn parse_key(key: &str) -> Result<[u8; 32], Error> {
    let bytes = Base64::decode_vec(key)
        .map_err(|_| Error::Base64Decode)?;

    <Vec<u8> as TryInto::<[u8; 32]>>::try_into(bytes)
        .map_err(|_| Error::InvalidLength)
}
