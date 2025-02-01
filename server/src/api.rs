use actix_web::{get,  post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::RwLock;

pub struct ApiState {
    pub test: i32,
    pub last: String,
}

#[get("/idk")]
async fn hello(state: web::Data<RwLock<ApiState>>) -> impl Responder {
    let s = state.read().unwrap();
    HttpResponse::Ok().body(s.last.to_string())
}

#[post("/upload_edges")]
async fn upload_edges(body: String, state: web::Data<RwLock<ApiState>>) -> impl Responder {
    let mut s = state.write().unwrap();
    println!("skill issue {} {:p}", s.last, &s);
    s.last = body;
    println!("skill issue {}", s.last);
    HttpResponse::Ok().body("Hello world!")
}

