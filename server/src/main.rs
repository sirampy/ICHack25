use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod api;
use std::sync::RwLock;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = web::Data::new(RwLock::new(api::ApiState {
                    test: 0,
                    last: "hello".to_string(),
                })); 
        
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                .app_data(data.clone())
                .service(api::hello)
                .service(api::upload_edges)
                )
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(fs::Files::new("/static", "static/").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
