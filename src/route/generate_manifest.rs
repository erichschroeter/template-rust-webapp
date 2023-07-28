use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, web};
// use futures_util::{StreamExt, TryStreamExt};
use futures_util::StreamExt as _;
use log::{debug, warn};
// use log::debug;
// use std::{io::Write, fs::create_dir_all, path::Path};


async fn manifest_tool(
    _image_path: &str,
    _payload_uri: &str,
) -> Result<(), std::io::Error> {
    // while let Some(item) = payload.next().await {
    //     let mut field = item?;
    //     let content_disposition = field.content_disposition();

    //     if let Some(name) = content_disposition.get_name() {
    //         debug!("Form name: {}", name);
    //     }
    // }
    warn!("Running manifest_tool");
    Ok(())
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
    Ok(HttpResponse::Ok().finish())
    // Ok(HttpResponse::SeeOther().append_header(("Location", "/")).finish())
}
