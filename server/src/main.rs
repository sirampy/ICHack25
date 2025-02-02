use actix_files as fs;
use actix_web::{
    cookie::{Cookie, SameSite},
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_cors::Cors;
use serde::Deserialize;
mod api;
use std::sync::RwLock;
use askama_actix::Template;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn index_get(req: HttpRequest) -> impl Responder {
    match req.cookie("username") {
        Some(username) => HttpResponse::Found().header("Location", "/home").finish(),
        None => HttpResponse::Found().header("Location", "/register").finish(),
    }
}


#[derive(Template)]
#[template(path = "network.html")]
struct NetworkTemplate {}

async fn network_get() -> impl Responder {
    NetworkTemplate {}
}


#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate {}

async fn profile_get() -> impl Responder {
    ProfileTemplate {}
}


#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {}

async fn register_get() -> impl Responder {
    RegisterTemplate {}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let data = web::Data::new(RwLock::new(api::ApiState::fakedata()));

    HttpServer::new(move || {
        App::new()
            // Wrap your app with CORS middleware
            .wrap(
                Cors::default()
                    // For testing, allow any origin. For production, you might restrict this.
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(
                web::scope("/api")
                .app_data(data.clone())
                .service(api::hello)
                // .service(api::upload_edges)
                .service(api::upload)
                )
            .service(echo)
            .route("/", web::get().to(index_get))
            .route("/register", web::get().to(register_get))
            .route("/network", web::get().to(network_get))
            .route("/profile", web::get().to(profile_get))
            .service(fs::Files::new("/static", "static/").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
