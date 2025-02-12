use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use cipher::{BlockEncryptMut, KeyIvInit};
use md5::Digest;
use p256::elliptic_curve::rand_core::OsRng;
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
        let ecdh_secret = p256::ecdh::EphemeralSecret::random(&mut OsRng);

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
            .encrypt_padded_b2b_mut::<cipher::block_padding::Pkcs7>(password, &mut ciphertext)
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
        let mut request = ureq::post(&format!("https://ankerpower-api-eu.anker.com{}", endpoint))
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
