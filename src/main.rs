mod settings;
mod command;
mod route;

use actix_web::{web, App, HttpServer};
use clap::{value_parser, Arg};
use config::{Config, File};
use log::{debug, error, info, trace, warn, LevelFilter};
use route::index::index;
use tera::Tera;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::command::Command;
use crate::command::run::RunCommand;
use crate::route::generate_manifest::generate_manifest;
use crate::route::image_upload::{image_upload, image_upload_get};
use crate::route::images::images;
use crate::route::manifest::manifest;
use crate::settings::{default_config_path, Settings};

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

    error!("log level enabled: error");
    warn!("log level enabled: warn");
    info!("log level enabled: info");
    debug!("log level enabled: debug");
    trace!("log level enabled: trace");
}

// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Help text")
// }

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

#[actix_web::main]
async fn main() {
    // setup_logging("info");
    const ABOUT: &str = "An example CLI program using the following crates:

  - clap
  - config
  - env_logger
  - directories
  - serde";
    let default_config_path_value = OsString::from(default_config_path().display().to_string());
    let app = clap::Command::new("FIXME")
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
                // .default_value(Settings::default().verbose)
                .help("Sets the verbosity log level")
                .long_help("Choices: [off, error, warn, info, debug, trace]"),
        )
        .infer_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::Command::new("run")
                .about("Run the web server")
                .arg(
                    Arg::new("address")
                        .long("address")
                        .short('a')
                        .env("FIXME_address")
                        // .action(ArgAction::Set)
                        // .default_value("127.0.0.1")
                        .value_name("ADDRESS")
                        .help("The port to run the HTTP server on")
                    )
                .arg(
                    Arg::new("port")
                        .long("port")
                        .short('p')
                        .env("FIXME_port")
                        // .action(ArgAction::Set)
                        .default_value("8080")
                        .value_parser(value_parser!(u16))
                        .value_name("PORT")
                        .help("The port to run the HTTP server on")
                    )
        )
        .subcommand(
            clap::Command::new("generate-manifest")
                .about("Generates a manifest file")
        );
    let matches = &app.get_matches();

    let config_path = matches.get_one::<PathBuf>("config").unwrap();
    println!("Loading config: {}", config_path.display());
    let settings = Config::builder()
        // Instead using clap for checking environment for variables.
        // .add_source(Environment::with_prefix("FIXME"))
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
        .build()
        .unwrap();

    // This will call From<Config> for Settings in settings.rs which will handle reading
    // the various config formats given the sources listed above via `add_source()`.
    let mut settings: Settings = settings.try_into().unwrap();

    // Override the verbose setting in the with command-line arg value if specified.
    if let Some(o) = matches.get_one::<String>("verbose") {
        println!("overriding verbose to {}", o);
        settings.verbose = o.to_owned();
    }

    // std::env::set_var("RUST_LOG", "actix_web=debug");
    // std::env::set_var("RUST_LOG", "trace");
    // std::env::set_var("RUST_BACKTRACE", "1");
    setup_logging(&settings.verbose);
    debug!("{}", settings);

    if let Some(sub_matches) = matches.subcommand_matches("run") {
        // Override the address setting in the with command-line arg value if specified.
        if let Some(o) = sub_matches.get_one::<String>("address") {
            settings.address = o.to_owned();
        }
        // Override the port setting in the with command-line arg value if specified.
        if let Some(o) = sub_matches.get_one::<u16>("port") {
            settings.port = o.to_owned();
        }
        // FUTURE add more parsing for new fields added to Settings struct
        debug!("{}", settings);
        run_http_server(&settings).await;
    } else {
        let subcommand = match matches.subcommand() {
            Some(("run", _sub_matches)) => {
                Box::new(RunCommand)
            }
            // Some(("config", sub_matches)) => {
            //     let mut out = match sub_matches.get_one::<PathBuf>("output") {
            //         Some(file) => {
            //             if Path::new(file).exists() {
            //                 if sub_matches.get_flag("force") {
            //                     fs::remove_file(file).unwrap();
            //                 } else {
            //                     error!("File already exists: {}", file.display());
            //                     std::process::exit(1);
            //                 }
            //             }

            //             let file = OpenOptions::new()
            //                 .create_new(true)
            //                 .write(true)
            //                 .append(true)
            //                 .open(file)
            //                 .unwrap();
            //             Box::new(file) as Box<dyn Write>
            //         }
            //         None => Box::new(std::io::stdout()) as Box<dyn Write>,
            //     };
            //     write_settings(
            //         &mut out,
            //         &settings,
            //         sub_matches
            //             .get_one::<SettingsOutputFormat>("format")
            //             .unwrap_or_default(),
            //     );
            // }
            _ => {
                // println!("{}", ABOUT);
                unreachable!()
            }
        };
        if let Err(e) = subcommand.execute() {
            eprintln!("Error executing command: {}", e);
        }
    }
}

async fn run_http_server(cfg: &Settings) {
    info!("Running HTTP Server at http://{}:{}", cfg.address, cfg.port);
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/images", web::get().to(images))
            .route("/image-upload", web::get().to(image_upload_get))
            .route("/image-upload", web::post().to(image_upload))
            .route("/generate-manifest", web::post().to(generate_manifest))
            .route("/manifest", web::get().to(manifest))
    })
    .bind((cfg.address.as_str(), cfg.port));

    match server {
        Ok(server) => {
            if let Err(e) = server.run().await {
                eprintln!("Server error: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to bind server: {}", e),
    }
}
