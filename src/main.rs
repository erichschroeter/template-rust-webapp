mod settings;

use clap::{value_parser, Arg, ArgAction};
use config::{Config, Environment, File};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::settings::{write_settings, Settings, SettingsOutputFormat};

/// Sets up logging based on the specified verbosity level.
///
/// This function initializes the logging framework using `env_logger` crate.
/// The verbosity level determines the amount of log output that will be displayed.
///
/// # Examples
///
/// ```
/// use crate::setup_logging;
///
/// setup_logging("debug");
/// ```
///
/// # Arguments
///
/// * `verbose` - A string slice representing the desired verbosity level.
///   Valid values are "off", "error", "warn", "info", "debug", and "trace".
///   If an invalid value is provided, the default level will be set to "info".
///
/// # Dependencies
///
/// This function depends on the following crates:
///
/// - `env_logger` - For setting up logging.
/// - `log` - For defining log levels.
///
/// # Panics
///
/// This function will panic if the `verbose` string cannot be parsed into a `LevelFilter`.
///
/// # Notes
///
/// It is recommended to call this function early in the program to set up logging
/// before any log messages are generated.
///
fn setup_logging(verbose: &str) {
    env_logger::builder()
        .filter(None, verbose.parse().unwrap_or(LevelFilter::Info))
        .init();
}

fn main() {
    const ABOUT: &str = "An example CLI program using the following crates:

  - clap
  - config
  - env_logger
  - directories
  - serde";
    let matches = clap::Command::new("FIXME")
        .version("v1.0.0")
        .author("Erich Schroeter <erich.schroeter@gmail.com>")
        .about(ABOUT)
        .long_about(format!(
            "{}

Argument values are processed in the following order, using the last processed value:

  1. config file (e.g. $HOME/.config/FIXME/default.json)
  2. environment variable (e.g. EXAMPLE_CLI_config=<path>)
  3. explicit argument (e.g. --config <path>)",
            ABOUT
        ))
        .subcommand(
            clap::Command::new("config")
                .about("View the present config or generate a default config.")
                .arg(
                    Arg::new("default")
                        .short('d')
                        .long("default")
                        .action(ArgAction::SetTrue)
                        .help("Generate a default config, rather than present environment values"),
                )
                .arg(
                    Arg::new("force")
                        .long("force")
                        .action(ArgAction::SetTrue)
                        .help("Overwrite any existing file"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Write the config output to a file")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .help("Specify output format for the config")
                        .value_parser(value_parser!(SettingsOutputFormat)),
                ),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help(format!(
                    "Sets a custom config file [default: {}]",
                    Settings::default().config_path.display().to_string()
                ))
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .value_name("VERBOSE")
                .help(format!(
                    "Sets the verbosity log level [default: {}]",
                    Settings::default().verbose
                ))
                .long_help("Choices: [error, warn, info, debug, trace]"),
        )
        .get_matches();

    let settings = Config::builder()
        .add_source(
            File::with_name(&Settings::default().config_path.display().to_string()).required(false),
        )
        .add_source(Environment::with_prefix("EXAMPLE_CLI"))
        .build()
        .unwrap();

    let mut settings: Settings = settings.try_into().unwrap();

    if let Some(o) = matches.get_one::<String>("verbose") {
        settings.verbose = o.to_owned();
    }

    if let Some(o) = matches.get_one::<PathBuf>("config") {
        settings.config_path = o.to_owned();
    }

    setup_logging(&settings.verbose);

    // Only check for config file to exist if not the default config.
    if settings.config_path != Settings::default().config_path && !settings.config_path.exists() {
        error!("Config not found: {}", settings.config_path.display());
        std::process::exit(1);
    } else {
    }

    error!("testing");
    warn!("testing");
    info!("{}", settings);
    debug!("testing");
    trace!("testing");

    match matches.subcommand() {
        Some(("config", sub_matches)) => {
            let mut out = match sub_matches.get_one::<PathBuf>("output") {
                Some(file) => {
                    if Path::new(file).exists() {
                        if sub_matches.get_flag("force") {
                            fs::remove_file(file).unwrap();
                        } else {
                            error!("File already exists: {}", file.display());
                            std::process::exit(1);
                        }
                    }

                    let file = OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .append(true)
                        .open(file)
                        .unwrap();
                    Box::new(file) as Box<dyn Write>
                }
                None => Box::new(std::io::stdout()) as Box<dyn Write>,
            };
            write_settings(
                &mut out,
                &settings,
                sub_matches
                    .get_one::<SettingsOutputFormat>("format")
                    .unwrap_or_default(),
            );
        }
        _ => {
            println!("{}", ABOUT);
        }
    }
}
