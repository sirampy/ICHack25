use actix_web::{cookie::time::Date, get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Local, Utc};
use std::sync::RwLock;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::iter;

#[derive(Serialize, Clone)]
pub struct Profile {
    pub name: String,
    pub pfp: String,
    pub linkedin: String,
    pub email: String,
}

#[derive(Default)]
pub struct ApiState {
    pub graph: HashMap<u64, HashMap<u64, DateTime<Local>>>,
    pub people: HashMap<u64, Profile>,
}

impl ApiState{
    pub fn fakedata() -> Self{
        let mut state = ApiState{
            graph: HashMap::new(),
            people: HashMap::new(),
        };

        let p0: Profile = Profile {name: "Person 0".to_owned(), pfp: "".to_owned(), linkedin:"".to_owned(), email: "".to_owned()};
        let p1: Profile = Profile {name: "Person 1".to_owned(), pfp: "".to_owned(), linkedin:"".to_owned(), email: "".to_owned()};
        let p2: Profile = Profile {name: "Person 2".to_owned(), pfp: "".to_owned(), linkedin:"".to_owned(), email: "".to_owned()};
        let p3: Profile = Profile {name: "Person 3".to_owned(), pfp: "".to_owned(), linkedin:"".to_owned(), email: "".to_owned()};
        let p4: Profile = Profile {name: "Person 4".to_owned(), pfp: "".to_owned(), linkedin:"".to_owned(), email: "".to_owned()};

        state.people.insert(0, p0);
        state.people.insert(1, p1);
        state.people.insert(2, p2);
        state.people.insert(3, p3);
        state.people.insert(4, p4);

        // p0scans
        let mut p0scans = HashMap::new();
        
        state.graph.insert(0 as u64, p0scans);
        
        // p0scans
        let mut p1scans = HashMap::new();
        p1scans.insert(3 as u64, Local::now());
        p1scans.insert(2 as u64, Local::now());
        state.graph.insert(1 as u64, p1scans);

        // p0scans
        let mut p2scans = HashMap::new();
        state.graph.insert(2 as u64, p2scans);

        // p0scans
        let mut p3scans = HashMap::new();
        state.graph.insert(3 as u64, p3scans);

        // p0scans
        let mut p4scans = HashMap::new();
        state.graph.insert(4 as u64, p4scans);
        
        state
    }
}



#[derive(Serialize)]
struct edge {
    from: u64,
    to: u64,
    ts: DateTime<Local>,
}

#[derive(Serialize)]
struct GraphDataPacket {
    edges: Vec<edge>,
    people: Vec<(u64, Profile)>,
}

impl ApiState {
    pub fn addEdge(&mut self, my: &u64, conns: Vec<(u64, DateTime<Local>)>) {
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



#[derive(Deserialize, Debug)]
struct RegistrationData {
    name: String,
    username: String,
    email: String,
    linkedin: String,
    scan: String,
}


#[post("/upload")]
async fn upload_edges(form: web::Form<RegistrationData>, state: web::Data<RwLock<ApiState>>) -> impl Responder {

    let data = form.into_inner();
    println!("Received registration data: {:?}", data);


    let mut s = state.write().unwrap();
    // println!("skill issue {} {:p}", s.last, &s);


    // s.last = body;
    // println!("skill issue {}", s.last);
    HttpResponse::Ok().body("Hello world!")
}

