use actix_web::{rt, web, HttpServer};
use clap::ArgMatches;
use log::{debug, info};
use tera::Tera;

use crate::{
    cli::{ArgHandler, CfgFileHandler, DefaultHandler, EnvHandler, Handler},
    cfg::{default_config_path, default_template_glob, Cfg},
};

const APP_PREFIX: &str = "FIXME_";

fn run_http_server(cfg: &Cfg) -> std::io::Result<()> {
    info!("Running HTTP Server at http://{}:{}", cfg.address, cfg.port);
    // let template_dir = cfg
    //     .template_dir
    //     .clone()
    //     .into_os_string()
    //     .into_string()
    //     .unwrap();
    // // let tera = Tera::new(&Path::new(&template_dir).join("/**/*").display().to_string()).unwrap();
    // // let template_dir = Arc::new(template_dir);
    // let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let tera = Tera::new(&cfg.template_glob).unwrap();
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(crate::route::index::index))
            .route("/images", web::get().to(crate::route::images::images))
            .route(
                "/image-upload",
                web::get().to(crate::route::image_upload::image_upload_get),
            )
            .route(
                "/image-upload",
                web::post().to(crate::route::image_upload::image_upload),
            )
            .route(
                "/generate-manifest",
                web::post().to(crate::route::generate_manifest::generate_manifest),
            )
            .route("/manifest", web::get().to(crate::route::manifest::manifest))
    })
    .bind((cfg.address.as_str(), cfg.port));

    if let Ok(server) = server {
        rt::System::new().block_on(server.run())
    } else {
        unimplemented!()
    }
}

pub fn run(matches: &ArgMatches) {
    // let default_config_path_value = OsString::from(default_config_path().display().to_string());
    // let default_template_dir = OsString::from(default_template_path());
    // let default_template_dir = OsString::from(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
    // let config_path = matches.get_one::<PathBuf>("config").unwrap();
    let config_path = ArgHandler::new(matches)
        .next(Box::new(EnvHandler::new().prefix(APP_PREFIX).next(
            Box::new(DefaultHandler::new(
                &default_config_path().display().to_string(),
            )),
        )))
        .handle_request("config");
    // let config_path = config_path.handle_request("config");
    let config_path = config_path.expect("msg");
    info!("Loading config: {}", config_path);
    // let settings = Config::builder()
    //     // Instead using clap for checking environment for variables.
    //     // .add_source(Environment::with_prefix("FIXME"))
    //     .add_source(
    //         config::File::with_name(
    //             &matches
    //                 .get_one::<PathBuf>("config")
    //                 .unwrap()
    //                 .display()
    //                 .to_string(),
    //         )
    //         .required(false),
    //     )
    //     .build()
    //     .unwrap();

    // // This will call From<Config> for Cfg in settings.rs which will handle reading
    // // the various config formats given the sources listed above via `add_source()`.
    // let mut cfg: Cfg = settings.try_into().unwrap();
    let mut cfg = Cfg::default();
    debug!("{}", cfg);

    let cfg_handler = CfgFileHandler::new(config_path);
    let template_glob = ArgHandler::new(matches)
        .next(Box::new(EnvHandler::new().prefix(APP_PREFIX).next(
            Box::new(cfg_handler.next(Box::new(DefaultHandler::new(&default_template_glob())))),
        )))
        .handle_request("template_glob");
    if let Some(template_glob) = template_glob {
        cfg.template_glob = template_glob;
    }

    // Override the address setting in the with command-line arg value if specified.
    if let Some(o) = matches.get_one::<String>("address") {
        cfg.address = o.to_owned();
    }
    // Override the port setting in the with command-line arg value if specified.
    if let Some(o) = matches.get_one::<u16>("port") {
        cfg.port = o.to_owned();
    }
    // FUTURE add more parsing for new fields added to Cfg struct
    debug!("{}", cfg);
    match run_http_server(&cfg) {
        Ok(_) => {}
        Err(_) => {}
    }
}
