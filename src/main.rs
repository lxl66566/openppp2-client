#![feature(let_chains)]
#![feature(lazy_cell)]

mod client_config;

use std::{
    env,
    fs::{self, File},
    io::{Cursor, Write},
    path::{Path, PathBuf},
    process::Command,
};

use colored::Colorize;
use die_exit::DieWith;
use log::{debug, info};

fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = client_config::read()?;
    let (json_files, mut file_names) = read_openppp2_settings(&config.config_dirs)?;
    file_names.insert(0, "Default".to_string());
    let selected_index = select(&file_names);
    debug!("selected_index: {:?}", selected_index);

    // cannot use tempdir: openppp will read config after few seconds. At that time
    // the config json will be deleted, cause openppp2 panic.
    let temp_dir = temp_dir()?;

    // selected `Default`
    let config_path = match selected_index {
        Some(0) => {
            let defaults_string: Vec<String> =
                config.defaults.iter().map(|x| x.to_string()).collect();
            let selected_index = select(&defaults_string).unwrap_or_else(|| std::process::exit(0));
            let select_ip_and_port = config
                .defaults
                .get(selected_index)
                .expect("the selected index must be valid");
            info!(
                "default select index: {}, ip and port: {}",
                selected_index, select_ip_and_port
            );
            let defaults_file = temp_dir.join("Default.json");
            fs::write(
                &defaults_file,
                default_settings(select_ip_and_port.ip.as_str(), select_ip_and_port.port).dump(),
            )?;
            defaults_file
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
    run(&config_path, &config.args);
    Ok(())
}

/// Returns openppp2 config json vec and filename vec.
fn read_openppp2_settings(config_dirs: &[PathBuf]) -> std::io::Result<(Vec<PathBuf>, Vec<String>)> {
    let mut json_files: Vec<PathBuf> = Vec::new();
    let mut file_names: Vec<String> = Vec::new();
    for dir in config_dirs {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if let Some(ext) = entry.path().extension()
                && ext == "json"
            {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                if file_names.contains(&file_name)
                    && let Some(parent_dir) = entry.path().parent()
                {
                    let parent_dir_name = parent_dir
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned();
                    let new_file_name = format!("{}/{}", parent_dir_name, file_name);
                    file_names.push(new_file_name);
                } else {
                    file_names.push(file_name);
                }
                json_files.push(entry.path());
            }
        }
    }
    debug_assert_eq!(json_files.len(), file_names.len());
    Ok((json_files, file_names))
}

fn default_settings(ip: &str, port: u16) -> json::JsonValue {
    let mut json = json::parse(include_str!("../appsettings.json")).unwrap();
    let ip_and_port = format!("{ip}:{port}");
    json["client"]["server"] = format!("ppp://{ip_and_port}/").into();
    json["udp"]["static"]["servers"] = vec![ip_and_port].into();
    json
}

/// Returns the index of selected item.
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
    info!("selected: {}", temp.selected_item_name());

    // The returned index start with 1 !!
    if temp.selected_item_name() != "Exit" {
        Some(selected - 1)
    } else {
        None
    }
}

/// run openppp2 with given config file and other args.
fn run(config_path: &Path, args: &[String]) {
    debug_assert!(config_path.exists());
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
    if let Err(e) = status
        && e.kind() == std::io::ErrorKind::NotFound
    {
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
        let res = new_command.spawn();
        res.die_with(|err| format!("Failed to start command: {}", err));
    }
}

/// make a permanent tempdir.
fn temp_dir() -> std::io::Result<PathBuf> {
    let path = env::temp_dir().join("openppp2");
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn write_direct_list() -> std::io::Result<PathBuf> {
    let compressed_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/direct-list.zst"));
    let decoded = zstd::stream::decode_all(Cursor::new(compressed_bytes)).unwrap();
    let path = temp_dir()?.join("direct-list.txt");
    let mut file = File::create(&path)?;
    file.write_all(&decoded)?;
    Ok(path)
}
