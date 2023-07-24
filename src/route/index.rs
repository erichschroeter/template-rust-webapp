use actix_web::{Responder, HttpResponse, web};
use tera::Context;


pub async fn index(tmpl: web::Data<tera::Tera>) -> impl Responder {
// pub async fn index() -> impl Responder {
    // HttpResponse::Ok().body("Help text")
    let mut ctx = Context::new();
    ctx.insert("title", "Index Page");
    let s = tmpl.render("index.html.tera", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
