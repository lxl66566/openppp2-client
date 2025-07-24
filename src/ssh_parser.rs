use std::{fs, path::PathBuf, sync::LazyLock as Lazy};

use anyhow::Result;
use log::debug;

use crate::client_config::DefaultConfigItem;

static DEFAULT_SSH_CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    home::home_dir()
        .expect("home dir not found")
        .join(".ssh")
        .join("config")
});

pub fn get_config_items_from_ssh_config(
    config_file_path: Option<PathBuf>,
    default_port: u16,
) -> Result<Vec<DefaultConfigItem>> {
    let mut path = None;
    if let Some(config_file_path) = config_file_path
        && config_file_path.exists() {
            path = Some(config_file_path);
        }
    let content = if let Some(path) = path {
        fs::read_to_string(path)?
    } else {
        fs::read_to_string(DEFAULT_SSH_CONFIG_FILE.as_path())?
    };
    let mut items = vec![];
    let mut current_host = None;
    for line in content.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 2 {
            continue;
        }
        if parts[0] == "Host" {
            current_host = Some(parts[1].to_string());
            continue;
        }
        if parts[0] == "HostName" {
            let ip = parts[1].to_string();
            items.push(DefaultConfigItem {
                name: current_host
                    .clone()
                    .unwrap_or_else(|| "undefined".to_string()),
                ip,
                port: default_port,
            });
        }
    }
    debug!("ssh config items: {items:?}");
    Ok(items)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_get_config_items_from_ssh_config() -> Result<()> {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_file_path = temp_dir.path().join("config");
        fs::write(
            &config_file_path,
            r#"Host ali
  User root
  HostName 47.76.185.33
Host axw
  User itdep
  HostName 172.18.58.54
Host github
  Port 443
  User git
  HostName ssh.github.com
"#,
        )?;
        let items = get_config_items_from_ssh_config(Some(config_file_path), 80)?;
        assert_eq!(items.len(), 3);
        assert_eq!(
            items[0],
            DefaultConfigItem {
                name: "ali".to_string(),
                ip: "47.76.185.33".to_string(),
                port: 80
            }
        );
        assert_eq!(
            items[1],
            DefaultConfigItem {
                name: "axw".to_string(),
                ip: "172.18.58.54".to_string(),
                port: 80
            }
        );
        assert_eq!(
            items[2],
            DefaultConfigItem {
                name: "github".to_string(),
                ip: "ssh.github.com".to_string(),
                port: 80
            }
        );
        Ok(())
    }
}
