use std::io::{self, ErrorKind};

use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse};
// use futures_util::{StreamExt, TryStreamExt};
// use futures_util::StreamExt as _;
use log::warn;
// use log::debug;
// use std::{io::Write, fs::create_dir_all, path::Path};

async fn manifest_tool(_image_path: &str, _payload_uri: &str) -> Result<String, std::io::Error> {
    // while let Some(item) = payload.next().await {
    //     let mut field = item?;
    //     let content_disposition = field.content_disposition();

    //     if let Some(name) = content_disposition.get_name() {
    //         debug!("Form name: {}", name);
    //     }
    // }
    warn!("Executing manifest_tool");
    let output = std::process::Command::new("manifest-tool")
        .arg("--version")
        // .args(&args)
        .output()?;

    if !output.status.success() {
        let kind = ErrorKind::Other;
        let err = io::Error::new(kind, "manifest-tool execution failed");
        return Err(err);
    }

    match String::from_utf8(output.stdout) {
        Ok(v) => Ok(v),
        Err(e) => Err(io::Error::new(ErrorKind::InvalidData, e.to_string())),
    }

    // Ok(())
}

pub async fn generate_manifest(
    mut _payload: Multipart,
    _req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    // let mut image_filename = None;
    // let mut payload_uri = None;
    // while let Some(item) = payload.next().await {
    //     let mut field = item?;
    //     let content_disposition = field.content_disposition();

    //     if let Some(field_name) = content_disposition.get_name() {
    //         match field_name {
    //             "image" => {
    //                 image_filename = Some(content_disposition);
    //             }
    //             "uri" => {
    //                 payload_uri = Some(field_name);
    //             }
    //             _ => {}
    //         }
    //     }
    //     // if let Some(name) = content_disposition.get_name() {
    //     //     debug!("Form name: {}", name);
    //     //     // If the file already exists, return 409 with Location of the conflicted file path.
    //     //     // if Path::new(&destination).exists() {
    //     //     //     return Ok(HttpResponse::Conflict().append_header(("Location", destination)).finish());
    //     //     // }

    //     //     // File::create is blocking operation, use threadpool
    //     //     let _file = web::block(|| manifest_tool(payload))
    //     //     .await
    //     //     .unwrap();
    //     // }
    // }
    // manifest_tool(image_filename.unwrap(), payload_uri.unwrap());
    if let Ok(output) = manifest_tool("sbh.sgi", "http://google.com").await {
        warn!("OUTPUT: {}", output);
    }
    Ok(HttpResponse::Ok().finish())
    // Ok(HttpResponse::SeeOther().append_header(("Location", "/")).finish())
}
