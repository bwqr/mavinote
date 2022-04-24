use jsonwebtoken::{EncodingKey, DecodingKey, errors::Error as JWTError, Header, Validation};
use ring::hmac;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Clone)]
pub struct Crypto {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    header: Header,
    validation: Validation,
    hmac512_key: hmac::Key,
}

impl Crypto {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            header: Header::default(),
            validation: Validation::default(),
            hmac512_key: hmac::Key::new(hmac::HMAC_SHA512, secret.as_bytes()),
        }
    }

    pub fn encode<T: Serialize>(&self, claims: &T) -> Result<String, JWTError> {
        jsonwebtoken::encode(&self.header, claims, &self.encoding_key)
    }

    pub fn decode<T: DeserializeOwned>(&self, token: &str) -> Result<T, JWTError> {
        jsonwebtoken::decode::<T>(token, &self.decoding_key, &self.validation)
            .map(|t| t.claims)
    }

    pub fn sign512(&self, message: &str) -> String {
        base64::encode(hmac::sign(&self.hmac512_key, message.as_bytes()))
    }
}
