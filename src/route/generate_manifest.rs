use actix_multipart::Multipart;
use actix_web::{HttpResponse, web::{self}, HttpRequest, http::header::CONTENT_LENGTH};
// use futures_util::{StreamExt, TryStreamExt};
use futures_util::StreamExt as _;
use log::debug;
use std::{io::Write, fs::create_dir_all, path::Path};


// async fn save_file(
//     mut payload: Multipart,
//     file_path: &str,
// ) -> Result<(), actix_web::Error> {
//     Ok(())
// }

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
        let content_disposition = field.content_disposition();

        if let Some(filename) = content_disposition.get_filename() {
                // .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
            debug!("File uploaded: {}", filename);
            create_dir_all(dest_dir)?;
            let destination = format!("{}{}", dest_dir, filename);
            debug!("Saving to: {}", destination);
            // If the file already exists, return 409 with Location of the conflicted file path.
            if Path::new(&destination).exists() {
                return Ok(HttpResponse::Conflict().append_header(("Location", destination)).finish());
            }

            // // Create a new file with the submitted filename
            // // let mut file = std::fs::File::create(destination)?;

            // // Write the field's bytes to the file
            // // while let Some(chunk) = field.next().await {
            // //     let data = chunk?;
            // //     debug!("Writing file data");
            // //     file.write_all(&data)?;
            // //     debug!("Wrote {} bytes", data.len());
            // // }

            // File::create is blocking operation, use threadpool
            let mut file = web::block(|| std::fs::File::create(destination))
            .await
            .unwrap().unwrap();

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                // let data = chunk.unwrap();
                let data = chunk.expect("HERE 0");
                // filesystem operations are blocking, we have to use threadpool
                file = web::block(move || {
                        debug!("Writing file data");
                        let res = file.write_all(&data).map(|_| file);
                        debug!("Wrote {} bytes", data.len());
                        res
                    })
                    .await
                    .unwrap().unwrap();
            }
        }
    }

    debug!("File upload complete!");

    // Create a new Tera context and render the success page
    // let mut ctx = Context::new();
    // ctx.insert("title", "Manifest Generation Success");
    // ctx.insert("message", "The file has been successfully processed.");
    // let s = tmpl.render("manifest.html.tera", &ctx).unwrap();

    // Ok(Redirect::to("/manifest"))
    // Ok(HttpResponse::Ok().into())
    Ok(HttpResponse::SeeOther().append_header(("Location", "/manifest")).finish())
    // Ok(HttpResponse::Ok().body(""))
}

// pub async fn generate_manifest(tmpl: web::Data<tera::Tera>) -> impl Responder {
//     // HttpResponse::Ok().body("Help text")

//     let mut ctx = Context::new();
//     ctx.insert("title", "Generate Manifest");
//     let s = tmpl.render("generate-manifest.html.tera", &ctx).unwrap();
//     HttpResponse::Ok().body(s)
// }
