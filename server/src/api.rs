use actix_web::{get,  post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::RwLock;
use std::collections::HashMap;
use serde::Deserialize;

pub struct Profile {
    pub fname: String,
    pub sname: String,
    pub pfp: String,
    pub linkedin: String,
    pub email: String,
}

#[derive(Default)]
pub struct ApiState {
    pub last: String,
    pub graph: HashMap<u64, HashMap<u64, u64>>,
    pub people: HashMap<u64, Profile>,
}

impl ApiState {
    fn addEdge(&mut self, my: &u64, conns: Vec<(u64, u64)>) {
        for (id, ts) in &conns{
            let lwr = if id < my {id} else {my};
            let hgr = if id < id {my} else {my};

            match self.graph.get_mut(lwr) {
                | Some(m) => {
                    match m.get_mut(hgr){
                    | Some(old_ts) => {
                        if *old_ts > *ts {*old_ts = *ts};
                    }
                    | None => {
                        m.insert(*hgr, *ts);
                    }
                    }
                }
                | None => {
                    let mut newmap = HashMap::new();
                    newmap.insert(*hgr, *ts);
                    self.graph.insert(*hgr, newmap);
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct UploadPacket {
    pub fname: String,
    pub sname: String,
    pub pfp: String,
    pub linkedin: String,
    pub email: String,
}

#[get("/idk")]
async fn hello(state: web::Data<RwLock<ApiState>>) -> impl Responder {
    let s = state.read().unwrap();
    HttpResponse::Ok().body(s.last.to_string())
}

#[post("/upload")]
async fn upload_edges(body: String, state: web::Data<RwLock<ApiState>>) -> impl Responder {
    let mut s = state.write().unwrap();
    println!("skill issue {} {:p}", s.last, &s);
    s.last = body;
    println!("skill issue {}", s.last);
    HttpResponse::Ok().body("Hello world!")
}

