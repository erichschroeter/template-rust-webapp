use actix_web::{Responder, HttpResponse, web};
use tera::Context;


pub async fn manifest(tmpl: web::Data<tera::Tera>) -> impl Responder {
    // HttpResponse::Ok().body("Help text")

    let mut ctx = Context::new();
    ctx.insert("title", "Manifest");
    ctx.insert("message", "Successful");
    let s = tmpl.render("manifest.html.tera", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
