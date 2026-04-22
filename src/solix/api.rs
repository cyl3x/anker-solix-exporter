use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use aes::cipher::{BlockEncryptMut, KeyIvInit};
use md5::Digest;
use p256::elliptic_curve::rand_core::{CryptoRng, OsRng, RngCore};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::credentials::Credentials;
use super::data;

const SERVER_PUBLIC_KEY: &str = "04c5c00c4f8d1197cc7c3167c52bf7acb054d722f0ef08dcd7e0883236e0d72a3868d9750cb47fa4619248f3d83f0f662671dadc6e2d31c2f41db0161651c7c076";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("API error {0}: {1}")]
    Api(u32, String),
    #[error("Request error: {0}")]
    Request(Box<ureq::Error>),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    #[allow(dead_code)]
    Data { code: u32, data: T, msg: String },
    NoData { code: u32, msg: String },
}

pub struct SolixApi {
    country: String,
    timezone: String,
    shared_secret: p256::ecdh::SharedSecret,
    public_key: String,
}

impl SolixApi {
    pub fn new(country: impl Into<String>, timezone: impl Into<String>) -> Self {
        Self::new_with_rng(country, timezone, &mut OsRng)
    }

    fn new_with_rng<R: CryptoRng + RngCore>(
        country: impl Into<String>,
        timezone: impl Into<String>,
        rng: &mut R,
    ) -> Self {
        let ecdh_secret = p256::ecdh::EphemeralSecret::random(rng);

        let server_pub_key_bytes =
            hex::decode(SERVER_PUBLIC_KEY).expect("Failed to decode public key");
        let server_pub_key = p256::PublicKey::from_sec1_bytes(&server_pub_key_bytes)
            .expect("Failed to create public key");

        let shared_secret = ecdh_secret.diffie_hellman(&server_pub_key);
        let public_key = hex::encode(ecdh_secret.public_key().to_sec1_bytes());

        SolixApi {
            country: country.into(),
            timezone: timezone.into(),
            shared_secret,
            public_key,
        }
    }

    fn encrypt_password(&self, password: &[u8]) -> String {
        let iv = &self.shared_secret.raw_secret_bytes()[0..16];

        let mut ciphertext = vec![0u8; password.len() + 16 - password.len() % 16];
        let cipher =
            cbc::Encryptor::<aes::Aes256>::new(self.shared_secret.raw_secret_bytes(), iv.into());
        cipher
            .encrypt_padded_b2b_mut::<aes::cipher::block_padding::Pkcs7>(password, &mut ciphertext)
            .expect("Encryption failed");

        base64::engine::general_purpose::STANDARD.encode(&ciphertext)
    }

    pub fn fetch<T>(
        &self,
        endpoint: &str,
        data: Option<impl Serialize>,
        credentials: Option<&Credentials>,
    ) -> Result<Response<T>, Error>
    where
        T: DeserializeOwned,
    {
        let mut request = ureq::post(&format!("https://ankerpower-api-eu.anker.com{endpoint}"))
            .header("Country", &self.country)
            .header("Timezone", &self.timezone)
            .header("Model-Type", "DESKTOP")
            .header("App-Name", "anker_power")
            .header("Os-Type", "android");

        if let Some(user) = credentials {
            if user.expires_in().unwrap() <= 0 {
                return Err(Error::InvalidCredentials);
            }

            request = request
                .header("X-Auth-Token", &user.auth_token)
                .header("gtoken", &hex::encode(md5::Md5::digest(&user.user_id)))
        }

        let response = match data {
            Some(data) => request.send_json(data),
            None => request.send_empty(),
        };

        match response {
            Ok(response) => match response.into_body().read_json::<Response<T>>() {
                Ok(data) => Ok(data),
                Err(err) => Err(Error::Request(Box::new(err))),
            },
            Err(ureq::Error::StatusCode(401)) => Err(Error::InvalidCredentials),
            Err(err) => Err(Error::Request(Box::new(err))),
        }
    }

    pub fn login(&self, username: &str, password: &str) -> Result<data::Login, Error> {
        let data = serde_json::json!({
            "ab": self.country,
            "client_secret_info": { "public_key": self.public_key },
            "enc": 0,
            "email": username,
            "password": self.encrypt_password(password.as_bytes()),
            "transaction": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
        });

        match self.fetch::<data::Login>("/passport/login", Some(&data), None) {
            Ok(Response::Data { data, .. }) => Ok(data),
            Ok(Response::NoData { msg, code, .. }) => Err(Error::Api(code, msg)),
            Err(err) => Err(err),
        }
    }

    pub fn get_scen_info(
        &self,
        creds: &Credentials,
        site_id: &str,
    ) -> Result<data::ScenInfo, Error> {
        let data = serde_json::json!({ "site_id": site_id });

        match self.fetch::<data::ScenInfo>(
            "/power_service/v1/site/get_scen_info",
            Some(&data),
            Some(creds),
        ) {
            Ok(Response::Data { data, .. }) => Ok(data),
            Ok(Response::NoData { msg, code, .. }) => Err(Error::Api(code, msg)),
            Err(err) => Err(err),
        }
    }

    pub fn get_site_homepage(&self, creds: &Credentials) -> Result<data::SiteHomepage, Error> {
        match self.fetch::<data::SiteHomepage>(
            "/power_service/v1/site/get_site_homepage",
            None::<()>,
            Some(creds),
        ) {
            Ok(Response::Data { data, .. }) => Ok(data),
            Ok(Response::NoData { msg, code, .. }) => Err(Error::Api(code, msg)),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedRng(u8);

    impl RngCore for FixedRng {
        fn next_u32(&mut self) -> u32 {
            u32::from_ne_bytes([self.0; 4])
        }
        fn next_u64(&mut self) -> u64 {
            u64::from_ne_bytes([self.0; 8])
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(self.0);
        }
        fn try_fill_bytes(
            &mut self,
            dest: &mut [u8],
        ) -> Result<(), p256::elliptic_curve::rand_core::Error> {
            dest.fill(self.0);
            Ok(())
        }
    }

    impl CryptoRng for FixedRng {}

    fn test_api() -> SolixApi {
        SolixApi::new_with_rng("DE", "Europe/Berlin", &mut FixedRng(0x01))
    }

    #[test]
    fn public_key_is_stable() {
        let api = test_api();
        assert_eq!(
            api.public_key,
            "046ff03b949241ce1dadd43519e6960e0a85b41a69a05c328103aa2bce1594ca163c4f753a55bf01dc53f6c0b0c7eee78b40c6ff7d25a96e2282b989cef71c144a"
        );
    }

    #[test]
    fn shared_secret_is_stable() {
        let api = test_api();
        let bytes = api.shared_secret.raw_secret_bytes();
        assert_eq!(
            hex::encode(bytes),
            "5174d46b10fe3195fbce476fc72adbe0e787a738fa61c6a1c3ddd04b1b05e4e4"
        );
    }

    #[test]
    fn encrypt_password_is_stable() {
        let api = test_api();
        let encrypted = api.encrypt_password(b"hunter2-correct-horse-battery-staple");
        assert_eq!(encrypted, "MUaEsGE15zyqvVSh9h9J99zH03hzJreSOUA74ivouUc+di96P8ZgerdqOCeR748e");
    }
}
