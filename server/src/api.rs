use actix_web::{get,  post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::RwLock;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::iter;

#[derive(Serialize, Clone)]
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

#[derive(Serialize)]
struct edge {
    from: u64,
    to: u64,
    ts: u64,
}

#[derive(Serialize)]
struct GraphDataPacket {
    edges: Vec<edge>,
    people: Vec<(u64, Profile)>,
}

impl ApiState {
    pub fn addEdge(&mut self, my: &u64, conns: Vec<(u64, u64)>) {
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

    pub fn get_graphData(&self) -> GraphDataPacket {
        let mut profiles = Vec::new();
        for (k, v) in self.people.iter(){
            profiles.push((*k,v.clone()));
        }
        GraphDataPacket {
            edges: self.get_edges(), 
            people: profiles,
        }
    }

    fn get_edges(&self) -> Vec<edge> {
        let mut out = Vec::new();
        for (from, tomap) in &self.graph {
            for(to, ts) in tomap {
                out.push(edge{from: *from, to: *to, ts: *ts});
            }
        }
        out.sort_by(|a, b| a.ts.cmp(&b.ts));
        out
    }
}


#[get("/graphData")]
async fn hello(state: web::Data<RwLock<ApiState>>) -> impl Responder {
    let s = state.read().unwrap();
    web::Json(s.get_graphData())
}

#[post("/upload")]
async fn upload_edges(body: String, state: web::Data<RwLock<ApiState>>) -> impl Responder {
    let mut s = state.write().unwrap();
    println!("skill issue {} {:p}", s.last, &s);
    s.last = body;
    println!("skill issue {}", s.last);
    HttpResponse::Ok().body("Hello world!")
}

