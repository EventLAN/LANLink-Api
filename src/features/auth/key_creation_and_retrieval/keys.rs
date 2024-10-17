use jsonwebtoken::{DecodingKey, EncodingKey};

///////// TEMP KEYS WILL EVENTUALLY BE GOT FROM ENV FILE OR FROM SECRET MANAGER
use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 60);
    Keys::new(secret.as_bytes())
});
///////// TEMP KEYS WILL EVENTUALLY BE GOT FROM ENV FILE OR FROM SECRET MANAGER

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub(crate) fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}