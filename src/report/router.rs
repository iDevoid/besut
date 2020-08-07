use actix_web::{ Responder, post, web, HttpResponse };
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    sentence: String
}

#[post("/report/new")]
async fn new_report(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("[POST] sentence: {}\n", form.sentence))
}

pub fn routes(conf: &mut web::ServiceConfig) {
    conf.service(new_report);
}