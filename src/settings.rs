use std::path::PathBuf;

use clap::builder::PossibleValue;
use config::Config;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SettingsOutputFormat {
    JSON,
    TOML,
    YAML,
}

impl Default for &SettingsOutputFormat {
    fn default() -> Self {
        &SettingsOutputFormat::TOML
    }
}

impl clap::ValueEnum for SettingsOutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            SettingsOutputFormat::JSON,
            SettingsOutputFormat::TOML,
            SettingsOutputFormat::YAML,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            SettingsOutputFormat::JSON => PossibleValue::new("json").help("JSON"),
            SettingsOutputFormat::TOML => PossibleValue::new("toml").help("TOML"),
            SettingsOutputFormat::YAML => PossibleValue::new("yaml").help("YAML"),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub verbose: String,
    pub address: String,
    pub port: u16,
    pub template_dir: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            verbose: "info".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8080,
            template_dir: default_template_path(),
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Config> for Settings {
    fn from(value: Config) -> Self {
        let mut cfg = Settings::default();
        if let Ok(o) = value.get_string("verbose") {
            cfg.verbose = o;
        }
        if let Ok(o) = value.get_string("address") {
            cfg.address = o;
        }
        if let Ok(o) = value.get_int("port") {
            cfg.port = o as u16;
        }
        if let Ok(o) = value.get_string("template_dir") {
            cfg.template_dir = PathBuf::from(o);
        }
        // FUTURE add more parsing for new fields added to Settings struct
        cfg
    }
}

#[allow(dead_code)]
pub fn write_settings(out: &mut dyn Write, settings: &Settings, fmt: &SettingsOutputFormat) {
    match fmt {
        SettingsOutputFormat::JSON => writeln!(
            out,
            "{}",
            serde_json::to_string_pretty(&settings).expect("Failed to serialize settings to JSON")
        )
        .expect("Failed to write config to stdout"),
        SettingsOutputFormat::TOML => writeln!(
            out,
            "{}",
            toml::to_string_pretty(&settings).expect("Failed to serialize settings to TOML")
        )
        .expect("Failed to write config to stdout"),
        SettingsOutputFormat::YAML => writeln!(
            out,
            "{}",
            serde_yaml::to_string(&settings).expect("Failed to serialize settings to YAML")
        )
        .expect("Failed to write config to stdout"),
    }
}

/// Returns the default configuration file path for the FIXME.
///
/// The default configuration file path is determined by appending
/// `".config/FIXME/default.yaml"` to the user's home directory.
///
/// # Examples
///
/// ```
/// use crate::default_config_path;
///
/// let path = default_config_path();
/// println!("Default configuration file path: {:?}", path);
/// ```
///
/// # Errors
///
/// This function will panic if it fails to retrieve the user's home directory
/// using the `UserDirs` struct from the `directories` crate.
///
/// # Returns
///
/// The function returns a `PathBuf` representing the default configuration file path.
///
/// # Safety
///
/// This function assumes that the `UserDirs` struct from the `directories` crate
/// is capable of correctly retrieving the user's home directory.
///
/// # Dependencies
///
/// This function depends on the following crates:
///
/// - `std::path::PathBuf` - For manipulating file paths.
/// - `directories` - For retrieving the user's home directory.
///
/// # Panics
///
/// This function will panic if it fails to retrieve the user's home directory.
///
/// # Notes
///
/// It is recommended to handle the potential errors when using this function.
///
pub fn default_config_path() -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    let mut path = PathBuf::from(user_dirs.home_dir());
    path.push(".config/FIXME/default.yaml");
    path
}

pub fn default_template_path() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
}
