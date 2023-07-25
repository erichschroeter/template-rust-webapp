use actix_multipart::Multipart;
use actix_web::{Responder, HttpResponse, web::{self, Redirect}};
// use futures::StreamExt;
use futures_util::StreamExt as _;
use log::debug;
use std::io::Write;


pub async fn generate_manifest(
    mut payload: Multipart
// ) -> Responder {
) -> Result<HttpResponse, actix_web::Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field
            .content_disposition();
            // .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        let filename = content_disposition
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        debug!("File uploaded: {}", filename);

        // Create a new file with the submitted filename
        let mut file = std::fs::File::create(filename)?;

        // Write the field's bytes to the file
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file.write_all(&data)?;
        }
    }

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
