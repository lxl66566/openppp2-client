use std::{path::PathBuf, sync::LazyLock as Lazy};

use home::home_dir;
use serde::{Deserialize, Serialize};

pub static DEFAULT_CLIENT_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    home_dir()
        .unwrap_or(PathBuf::from("."))
        .join(".config")
        .join(env!("CARGO_PKG_NAME").to_string() + ".toml")
});

/// The config for openppp2 client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub defaults: Vec<DefaultConfigItem>,
    pub config_dirs: Vec<PathBuf>,
    pub args: Vec<String>,
}

/// the name, ip and port of default openppp2 config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfigItem {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

/// An example for Defaults.
impl Default for DefaultConfigItem {
    fn default() -> Self {
        Self {
            name: "example".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 2777,
        }
    }
}

impl std::fmt::Display for DefaultConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}:{}", self.name, self.ip, self.port)
    }
}

impl From<&str> for DefaultConfigItem {
    fn from(s: &str) -> Self {
        let mut split = s.trim_matches('/').split(':');
        let ip = split.next().expect("invalid ip").to_string();
        let port = split
            .next()
            .expect("invalid port")
            .parse()
            .expect("port must be a number");
        Self {
            name: "undefined".to_string(),
            ip,
            port,
        }
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            defaults: vec![DefaultConfigItem::default()],
            config_dirs: vec![
                PathBuf::from("."),
                DEFAULT_CLIENT_CONFIG_PATH
                    .parent()
                    .expect("builtin config file must have a parent directory.")
                    .to_path_buf(),
            ],
            args: vec![
                "--mode=client",
                "--tun-ip=10.0.0.2",
                "--tun-gw=10.0.0.0",
                "--tun-mask=24",
                "--tun-host=yes",
                "--tun-vnet=yes",
                "--block-quic=yes",
                "--set-http-proxy=no",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }
}
