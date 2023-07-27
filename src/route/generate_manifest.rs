use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse};
// use futures_util::{StreamExt, TryStreamExt};
// use futures_util::StreamExt as _;
// use log::debug;
// use std::{io::Write, fs::create_dir_all, path::Path};


pub async fn generate_manifest(
    mut _payload: Multipart,
    _req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::SeeOther().append_header(("Location", "/")).finish())
}
