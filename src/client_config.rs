use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    sync::LazyLock as Lazy,
};

use die_exit::Die;
use home::home_dir;
use serde::{Deserialize, Serialize};

pub static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    home_dir()
        .unwrap_or(PathBuf::from("."))
        .join(".config")
        .join(env!("CARGO_PKG_NAME").to_string() + ".toml")
});

/// The config for openppp2 client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub defaults: Vec<Defaults>,
    pub config_dirs: Vec<PathBuf>,
    pub args: Vec<String>,
}

/// the name, ip and port of default openppp2.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

/// An example for Defaults.
impl Default for Defaults {
    fn default() -> Self {
        Self {
            name: "example".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 2777,
        }
    }
}

impl std::fmt::Display for Defaults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}:{}", self.name, self.ip, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: vec![Defaults::default()],
            config_dirs: vec![
                PathBuf::from("."),
                CONFIG_FILE
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

/// Read config from CONFIG_FILE, if config file not found, create a default
/// config.
pub fn read() -> std::io::Result<Config> {
    let file = File::open(CONFIG_FILE.as_path());
    let mut file = match file {
        Ok(f) => f,
        Err(e) => {
            // If the file doesn't exist, write it.
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(e);
            }
            let temp = Config::default();
            fs::create_dir_all(CONFIG_FILE.parent().unwrap())?;
            let mut f = File::create(CONFIG_FILE.as_path())?;
            f.write_all(
                toml::to_string(&temp)
                    .expect("default config must to_string successfully.")
                    .as_bytes(),
            )?;
            return Ok(temp);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents).die("The config toml is not a valid toml.");
    Ok(config)
}
