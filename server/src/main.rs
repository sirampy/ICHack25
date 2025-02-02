use actix_files as fs;
use actix_web::{cookie::{Cookie, SameSite}, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
mod api;
use std::sync::RwLock;
use askama_actix::Template;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}



async fn index_get(req: HttpRequest) -> impl Responder {

    match req.cookie("username"){
        Some(username) => HttpResponse::Found().header("Location", "/home").finish(),
        None => HttpResponse::Found().header("Location", "/register").finish()
    } 
     
}

#[derive(Template)]
#[template(path = "collector.html")] 
struct Collector_template {}


async fn collector_get() -> impl Responder {
    Collector_template {}
}


#[derive(Template)]
#[template(path = "register.html")] 
struct RegisterTemplate {}

async fn register_get() -> impl Responder {
    RegisterTemplate {}
}

#[derive(Deserialize, Debug)]
struct RegistrationData {
    username: String,
    email: String,
    linkedin: String,
}

async fn register_post(form: web::Form<RegistrationData>) -> impl Responder {

    let data = form.into_inner();
    println!("Received registration data: {:?}", data);

    let mut cookie = Cookie::new("username", data.username);
    cookie.set_http_only(true); 
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");

    HttpResponse::Found()
        .header("Location", "/collector")
        .cookie(cookie)
        .body("Redirect!")

}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = web::Data::new(RwLock::new(api::ApiState::default())); 
        
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                .app_data(data.clone())
                .service(api::hello)
                .service(api::upload_edges)
                )
            .service(echo)
            .route("/", web::get().to(index_get))
            .route("/home", web::get().to(index_get))
            .route("/register", web::get().to(register_get))
            .route("/register", web::post().to(register_post))
            .route("/collector", web::get().to(collector_get))

            .service(fs::Files::new("/static", "static/").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
