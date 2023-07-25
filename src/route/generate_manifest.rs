use actix_multipart::Multipart;
use actix_web::{Responder, HttpResponse, web::{self, Redirect}, HttpRequest, http::header::CONTENT_LENGTH};
// use futures::StreamExt;
use futures_util::StreamExt as _;
use log::debug;
use std::{io::Write, fs::create_dir_all};


pub async fn generate_manifest(
    mut payload: Multipart,
    req: HttpRequest,
// ) -> Responder {
) -> Result<HttpResponse, actix_web::Error> {
    let content_lenth: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };
    debug!("CONTENT_LENTH: {}", content_lenth);
    let dest_dir = "./uploads/";

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field
            .content_disposition();
            // .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        let filename = content_disposition
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        debug!("File uploaded: {}", filename);
        create_dir_all(dest_dir)?;
        let destination = format!("{}{}", dest_dir, filename);
        debug!("Saving to: {}", destination);

        // Create a new file with the submitted filename
        let mut file = std::fs::File::create(destination)?;

        // Write the field's bytes to the file
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            debug!("Writing file data");
            file.write_all(&data)?;
            debug!("Wrote {} bytes", data.len());
        }
    }

    debug!("File upload complete!");

    // Create a new Tera context and render the success page
    // let mut ctx = Context::new();
    // ctx.insert("title", "Manifest Generation Success");
    // ctx.insert("message", "The file has been successfully processed.");
    // let s = tmpl.render("manifest.html.tera", &ctx).unwrap();

    // Ok(Redirect::to("/manifest"))
    Ok(HttpResponse::Ok().into())
    // Ok(HttpResponse::Ok().body(""))
}

// pub async fn generate_manifest(tmpl: web::Data<tera::Tera>) -> impl Responder {
//     // HttpResponse::Ok().body("Help text")

//     let mut ctx = Context::new();
//     ctx.insert("title", "Generate Manifest");
//     let s = tmpl.render("generate-manifest.html.tera", &ctx).unwrap();
//     HttpResponse::Ok().body(s)
// }
