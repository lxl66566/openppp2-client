pub mod cli;
pub mod client_config;
pub mod utils;

use std::{
    env, fs,
    fs::File,
    io::{Cursor, Write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use assert2::assert;
use cli::CLI;
use client_config::{ClientConfig, DefaultConfigItem, DEFAULT_CLIENT_CONFIG_PATH};
use colored::Colorize;
use config_file2::{LoadConfigFile, StoreConfigFile};
use log::{debug, info, warn};
use once_fn::once;
use path_absolutize::Absolutize;
use utils::Unzip;

fn main() -> anyhow::Result<()> {
    utils::log_init();
    let config = ClientConfig::load(
        CLI.config
            .as_deref()
            .unwrap_or(DEFAULT_CLIENT_CONFIG_PATH.as_path()),
    )?;
    // Get the config, otherwise store one to the default path.
    let client_config = config.unwrap_or_else(|| {
        warn!(
            "config file not found, use default config and write to {:?}.",
            DEFAULT_CLIENT_CONFIG_PATH
        );
        ClientConfig::default()
            .store(DEFAULT_CLIENT_CONFIG_PATH.as_path())
            .expect("store default config failed");
        ClientConfig::default()
    });

    // If provide the use param
    if let Some(subcommand) = &CLI.subcommand {
        match subcommand {
            cli::SubCommand::Use { config } => {
                if let Some(config_item) = DefaultConfigItem::parse(config) {
                    let config_path =
                        dumped_default_settings(config_item.ip.as_str(), config_item.port);
                    run(&config_path, &client_config.args)?;
                } else {
                    run(Path::new(config), &client_config.args)?;
                }
            }
        }
        return Ok(());
    }

    let (json_files, mut file_names) = read_openppp2_settings(&client_config.config_dirs)?.unzip();
    file_names.insert(0, "Default".to_string());
    let selected_index = select(&file_names);
    debug!("selected_index: {:?}", selected_index);

    let config_path = match selected_index {
        // selected `Default`
        Some(0) => {
            let defaults_string: Vec<String> = client_config
                .defaults
                .iter()
                .map(|x| x.to_string())
                .collect();
            let selected_index = select(&defaults_string).unwrap_or_else(|| std::process::exit(0));
            let select_ip_and_port = client_config
                .defaults
                .get(selected_index)
                .expect("the selected index must be valid");
            info!(
                "default select index: {}, ip and port: {}",
                selected_index, select_ip_and_port
            );
            dumped_default_settings(select_ip_and_port.ip.as_str(), select_ip_and_port.port)
        }
        None => {
            std::process::exit(0);
        }
        Some(index) => {
            let selected_file_path = json_files.get(index - 1).expect("The index must be valid.");
            let file_name = file_names.get(index - 1).expect("The index must be valid.");
            info!(
                "selected file: {}, path: {}",
                file_name,
                selected_file_path.display()
            );
            selected_file_path.to_owned()
        }
    };
    debug!("use config file: {}", config_path.display());
    run(&config_path, &client_config.args)
}

/// Returns all config files and their names in the given config dirs.
fn read_openppp2_settings(config_dirs: &[PathBuf]) -> anyhow::Result<Vec<(PathBuf, String)>> {
    let mut output = vec![];
    for config_dir in config_dirs {
        let dir_full_path = config_dir.absolutize()?;
        for entry in glob::glob(&format!("{}/*.json", config_dir.to_string_lossy()))? {
            let entry = entry?;
            let entry_full_path = entry.absolutize()?;
            let name = entry_full_path
                .strip_prefix(&dir_full_path)?
                .to_string_lossy()
                .into_owned();
            output.push((entry_full_path.into_owned(), name));
        }
    }
    Ok(output)
}

/// Returns the default settings value for openppp2.
#[inline]
fn default_settings(ip: &str, port: u16) -> json::JsonValue {
    let mut json = json::parse(include_str!("../appsettings.json")).unwrap();
    let ip_and_port = format!("{ip}:{port}");
    json["client"]["server"] = format!("ppp://{ip_and_port}/").into();
    json["udp"]["static"]["servers"] = vec![ip_and_port].into();
    json
}

/// Dump the default settings value for openppp2 to a temp file.
///
/// # Returns
///
/// The path of the dumped file.
fn dumped_default_settings(ip: &str, port: u16) -> PathBuf {
    let defaults_file = temp_dir().join("Default.json");
    fs::write(&defaults_file, default_settings(ip, port).dump())
        .expect("write default settings failed");
    defaults_file
}

/// Returns the index of selected item. If select `Exit`, return None.
fn select(items: &[String]) -> Option<usize> {
    use terminal_menu::{back_button, button, label, menu, mut_menu, run};
    let mut menu_items = vec![label(
        "Please select the config you want to use:"
            .bold()
            .to_string(),
    )];
    menu_items.reserve(items.len() + 2);
    items.iter().map(button).for_each(|x| menu_items.push(x));
    menu_items.push(back_button("Exit"));
    let select_menu = menu(menu_items);
    run(&select_menu);
    let temp = mut_menu(&select_menu);
    let selected = temp.selected_item_index();
    debug!("selected: {}", temp.selected_item_name());

    // The returned index start with 1 !!
    if temp.selected_item_name() != "Exit" {
        Some(selected - 1)
    } else {
        None
    }
}

/// run openppp2 with given config file and running args.
fn run(config_path: &Path, args: &[String]) -> anyhow::Result<()> {
    assert!(
        fs::exists(config_path).context("cannot access config file")?,
        "config file `{:?}` not found",
        config_path
    );
    let content = fs::read_to_string(config_path).expect("read config file failed");
    debug!("config file content: {}", content);

    let mut command = Command::new("ppp");
    let args: Vec<&String> = args.iter().collect();
    command.args(&args);
    command.arg(format!("--config={}", config_path.to_string_lossy()));
    if let Ok(direct_list) = write_direct_list() {
        command.arg(format!("--dns-rules={}", direct_list.to_string_lossy()));
    }

    info!("Running: `{:?}`", command);
    let status = command.spawn();

    // if NotFound, try other extension.
    if status.is_err() && status.unwrap_err().kind() == std::io::ErrorKind::NotFound {
        let mut new_command = if cfg!(windows) {
            info!("exe not found, try cmd");
            Command::new("ppp.cmd")
        } else {
            info!("ppp not found, try sh");
            Command::new("ppp.sh")
        };

        command.get_args().for_each(|arg| {
            new_command.arg(arg);
        });
        let status = new_command.spawn()?.wait()?;
        info!("running status: {:?}", status);
    }
    Ok(())
}

/// make a permanent tempdir and return its path.
#[once]
fn temp_dir() -> PathBuf {
    let path = env::temp_dir().join("openppp2");
    fs::create_dir_all(&path).expect("create temp dir failed");
    path
}

/// Write the direct-list.txt to a temp file.
fn write_direct_list() -> std::io::Result<PathBuf> {
    let compressed_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/direct-list.zst"));
    let decoded = zstd::stream::decode_all(Cursor::new(compressed_bytes)).unwrap();
    let path = temp_dir().join("direct-list.txt");
    let mut file = File::create(&path)?;
    file.write_all(&decoded)?;
    Ok(path)
}
