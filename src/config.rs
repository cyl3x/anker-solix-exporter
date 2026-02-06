use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::{Path, PathBuf};

use figment::providers::Format;
use figment::{
    providers::{Env, Json},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_address")]
    address: SocketAddr,
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    #[serde(default = "default_country")]
    country: String,
    #[serde(default = "default_timezone")]
    timezone: String,
    #[serde(default = "default_cache_file")]
    cache_file: PathBuf,
}

fn default_address() -> SocketAddr {
    SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080).into()
}

fn default_cache_file() -> PathBuf {
    PathBuf::from("token_cache.json")
}

fn default_country() -> String {
    "DE".to_string()
}

fn default_timezone() -> String {
    "Europe/Berlin".to_string()
}

impl Config {
    pub fn new() -> Result<Self, Box<figment::Error>> {
        let args: Vec<String> = env::args().collect();

        let json = args.get(2).map_or("{}", |v| v);

        Figment::new()
            .merge(Env::prefixed("ANKER_SOLIX_"))
            .join(Json::string(json))
            .extract()
            .map_err(Box::new)
    }

    pub fn country(&self) -> &str {
        self.country.as_str()
    }

    pub fn timezone(&self) -> &str {
        self.timezone.as_str()
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn password(&self) -> &str {
        self.password.as_str()
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn cache_file(&self) -> &Path {
        &self.cache_file
    }
}
