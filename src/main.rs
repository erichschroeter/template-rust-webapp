mod settings;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::builder::OsStr;
use clap::{value_parser, Arg, ArgAction};
use config::{Config, Environment, File};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::settings::{default_config_path, write_settings, Settings, SettingsOutputFormat};

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

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Help text")
}

// async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
//     while let Ok(Some(mut field)) = payload.try_next().await {
//         let content_disposition = field.content_disposition().unwrap();
//         let filename = content_disposition.get_filename().unwrap();
//         let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
//         let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();
//         while let Some(chunk) = field.next().await {
//             let data = chunk.unwrap();
//             f = web::block(move || f.write_all(&data).map(|_| f)).await?;
//         }
//     }
//     Ok(HttpResponse::Ok().into())
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     const HOST_URI: &str = "127.0.0.1:8080";
//     setup_logging("info");
//     info!("Access via {}", HOST_URI);
//     HttpServer::new(|| {
//         App::new()
//             // .service(web::resource("/upload").route(web::post().to(upload)))
//             .route("/", web::get().to(index))
//             // .route("/generate-manifest", web::get().to(generate_manifest))
//     })
//     .bind(HOST_URI)?
//     .run()
//     .await
// }

fn main() {
    const ABOUT: &str = "An example CLI program using the following crates:

  - clap
  - config
  - env_logger
  - directories
  - serde";
    let default_config_path_value = OsString::from(default_config_path().display().to_string());
    // const DEFAULT_CONFIG_PATH: &str = "";
    let matches = clap::Command::new("FIXME")
        .version("v1.0.0")
        .author("Erich Schroeter <erich.schroeter@gmail.com>")
        .about(ABOUT)
        .long_about(format!(
            "{}

Argument values are processed in the following order, using the last processed value:

  1. config file (e.g. $HOME/.config/FIXME/default.json)
  2. environment variable (e.g. FIXME_config=<path>)
  3. explicit argument (e.g. --config <path>)",
            ABOUT
        ))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .default_value(&default_config_path_value)
                .help("Sets a custom config file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .value_name("VERBOSE")
                .default_value(Settings::default().verbose)
                .help("Sets the verbosity log level")
                .long_help("Choices: [off, error, warn, info, debug, trace]"),
        )
        .get_matches();

    let settings = Config::builder()
        .add_source(
            File::with_name(
                &matches
                    .get_one::<PathBuf>("config")
                    .unwrap()
                    .display()
                    .to_string(),
            )
            .required(false),
        )
        .add_source(Environment::with_prefix("FIXME"))
        .build()
        .unwrap();

    let mut settings: Settings = settings.try_into().unwrap();

    // Override the verbose setting in the config file with a command line arg value if specified.
    if let Some(o) = matches.get_one::<String>("verbose") {
        settings.verbose = o.to_owned();
    }

    setup_logging(&settings.verbose);

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
