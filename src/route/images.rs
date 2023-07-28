use std::fs;

use actix_web::{ HttpResponse, web};
use tera::Context;


pub async fn images(tmpl: web::Data<tera::Tera>) -> actix_web::Result<HttpResponse> {
    // Read the uploads directory
    let entries = fs::read_dir("./uploads")?;

    // Collect .cgi and .sgi files
    let mut images: Vec<String> = Vec::new();
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

    let mut ctx = Context::new();
    ctx.insert("title", "Firmware Images");
    ctx.insert("images", &images);
    let rendered = tmpl.render("images.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}
