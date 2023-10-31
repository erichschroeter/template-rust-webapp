use actix_multipart::Multipart;
use actix_web::{
    http::header::CONTENT_LENGTH,
    web::{self},
    HttpRequest, HttpResponse,
};
// use futures_util::{StreamExt, TryStreamExt};
use futures_util::StreamExt as _;
use log::debug;
use std::{fs::create_dir_all, io::Write, path::Path};
use tera::Context;

use super::VERSION;

pub async fn image_upload_get(tmpl: web::Data<tera::Tera>) -> actix_web::Result<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("version", &VERSION);
    ctx.insert("title", "Upload Firmware Image");
    let rendered = tmpl.render("image_upload.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn image_upload(
    mut payload: Multipart,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let content_lenth: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };
    let dest_dir = "./uploads/";

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();

        if let Some(filename) = content_disposition.get_filename() {
            create_dir_all(dest_dir)?;
            let destination = format!("{}{}", dest_dir, filename);
            debug!(
                "Writing to {} file '{}' ({} bytes)",
                dest_dir, filename, content_lenth
            );
            // If the file already exists, return 409 with Location of the conflicted file path.
            if Path::new(&destination).exists() {
                return Ok(HttpResponse::Conflict()
                    .append_header(("Location", destination))
                    .finish());
            }

            // File::create is blocking operation, use threadpool
            let mut file = web::block(|| std::fs::File::create(destination))
                .await
                .unwrap()
                .unwrap();

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                file = web::block(move || {
                    let res = file.write_all(&data).map(|_| file);
                    res
                })
                .await
                .unwrap()
                .unwrap();
            }
        }
    }

    debug!("File upload complete!");

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/manifest"))
        .finish())
}
