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
    #[serde(default)]
    pub defaults: Vec<DefaultConfigItem>,
    pub config_dirs: Vec<PathBuf>,
    pub args: Vec<String>,
    /// The default port for [`DefaultConfigItem`]s that get from ssh config
    /// file.
    #[serde(default)]
    pub default_port_for_ssh: u16,
    /// Whether to enable chnroutes by default.
    #[serde(default)]
    pub enable_chnroutes_by_default: bool,
}

/// the name, ip and port of default openppp2 config.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefaultConfigItem {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

impl DefaultConfigItem {
    /// Parse the string to DefaultConfigItem.
    ///
    /// # Examples
    ///
    /// ```
    /// use openppp2_client::client_config::DefaultConfigItem;
    ///
    /// let item = DefaultConfigItem::parse("127.0.0.1:2777").unwrap();
    /// assert_eq!(item.name, "undefined");
    /// assert_eq!(item.ip, "127.0.0.1");
    /// assert_eq!(item.port, 2777);
    /// assert!(DefaultConfigItem::parse("asdf").is_none());
    /// ```
    pub fn parse(s: impl AsRef<str>) -> Option<Self> {
        let mut split = s.as_ref().trim_matches('/').split(':');
        let ip = split.next()?.to_string();
        let port = split.next()?.parse().ok()?;
        Some(Self {
            name: "undefined".to_string(),
            ip,
            port,
        })
    }
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
                "--tun-flash=yes",
                "--tun-vnet=yes",
                "--block-quic=yes",
                "--set-http-proxy=yes",
                "--tun-mux=4",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
            default_port_for_ssh: 80,
            enable_chnroutes_by_default: false,
        }
    }
}
