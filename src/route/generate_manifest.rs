use actix_web::{Responder, HttpResponse, web};
use tera::Context;


pub async fn generate_manifest(tmpl: web::Data<tera::Tera>) -> impl Responder {
// pub async fn index() -> impl Responder {
    // HttpResponse::Ok().body("Help text")
    let mut ctx = Context::new();
    ctx.insert("title", "Generate Manifest");
    let s = tmpl.render("generate-manifest.html.tera", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
