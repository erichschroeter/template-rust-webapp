use std::fs;

use actix_web::{HttpResponse, web};
use tera::Context;

use super::VERSION;


pub async fn manifest(tmpl: web::Data<tera::Tera>) -> actix_web::Result<HttpResponse> {
    // Collect .cgi and .sgi files
    let mut images: Vec<String> = Vec::new();

    // Read the uploads directory
    if let Ok(entries) = fs::read_dir("./uploads") {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Check if the entry is a file and if it has the correct extension
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "cgi" || extension == "sgi" {
                        if let Some(file_name) = path.file_name() {
                            images.push(file_name.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    }

    let mut ctx = Context::new();
    ctx.insert("version", &VERSION);
    ctx.insert("title", "Manifest");
    ctx.insert("images", &images);
    let rendered = tmpl.render("manifest.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}
