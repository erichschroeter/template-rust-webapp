use std::path::PathBuf;

use clap::builder::PossibleValue;
use config::Config;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CfgOutputFormat {
    JSON,
    TOML,
    YAML,
}

impl Default for &CfgOutputFormat {
    fn default() -> Self {
        &CfgOutputFormat::TOML
    }
}

impl clap::ValueEnum for CfgOutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            CfgOutputFormat::JSON,
            CfgOutputFormat::TOML,
            CfgOutputFormat::YAML,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            CfgOutputFormat::JSON => PossibleValue::new("json").help("JSON"),
            CfgOutputFormat::TOML => PossibleValue::new("toml").help("TOML"),
            CfgOutputFormat::YAML => PossibleValue::new("yaml").help("YAML"),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cfg {
    pub verbose: String,
    pub address: String,
    pub port: u16,
    pub template_glob: String,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            verbose: "info".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8080,
            template_glob: default_template_glob(),
        }
    }
}

impl std::fmt::Display for Cfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Config> for Cfg {
    fn from(value: Config) -> Self {
        let mut cfg = Cfg::default();
        if let Ok(o) = value.get_string("verbose") {
            cfg.verbose = o;
        }
        if let Ok(o) = value.get_string("address") {
            cfg.address = o;
        }
        if let Ok(o) = value.get_int("port") {
            cfg.port = o as u16;
        }
        if let Ok(o) = value.get_string("template_glob") {
            cfg.template_glob = o;
        }
        // FUTURE add more parsing for new fields added to Cfg struct
        cfg
    }
}

#[allow(dead_code)]
pub fn write_cfg(out: &mut dyn Write, settings: &Cfg, fmt: &CfgOutputFormat) {
    match fmt {
        CfgOutputFormat::JSON => writeln!(
            out,
            "{}",
            serde_json::to_string_pretty(&settings).expect("Failed to serialize settings to JSON")
        )
        .expect("Failed to write config to stdout"),
        CfgOutputFormat::TOML => writeln!(
            out,
            "{}",
            toml::to_string_pretty(&settings).expect("Failed to serialize settings to TOML")
        )
        .expect("Failed to write config to stdout"),
        CfgOutputFormat::YAML => writeln!(
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
#[allow(dead_code)]
pub fn default_config_path() -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    let mut path = PathBuf::from(user_dirs.home_dir());
    path.push(".config/FIXME/default.yaml");
    path
}

#[allow(dead_code)]
pub fn default_template_glob() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*").to_string()
}

#[cfg(test)]
mod tests {
    use unindent::unindent;

    use super::*;

    #[test]
    fn writing_default_cfg_as_yaml() {
        let expected = format!(
            r#"
        verbose: info
        address: 127.0.0.1
        port: 8080
        template_glob: {}

        "#,
            default_template_glob()
        );
        let mut actual = Vec::new();
        let settings = Cfg::default();
        write_cfg(&mut actual, &settings, &CfgOutputFormat::YAML);
        assert_eq!(unindent(&expected), String::from_utf8_lossy(&actual));
    }
}
