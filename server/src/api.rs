use actix_multipart::form::{
    json::Json as MpJson, tempfile::TempFile, text::Text as MpText, MultipartForm,
};
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub firstname: String,
    pub lastname: String,
    pub pfp: String,
    pub linkedin: String,
    pub email: String,
}

#[derive(Default)]
pub struct ApiState {
    pub graph: HashMap<u64, HashMap<u64, DateTime<Local>>>,
    pub people: HashMap<u64, Profile>,
}

impl ApiState {
    pub fn fakedata() -> Self {
        let mut state = ApiState {
            graph: HashMap::new(),
            people: HashMap::new(),
        };

        let p0: Profile = Profile {
            firstname: "Person 0".to_owned(),
            lastname: "L0".to_owned(),
            pfp: "".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };
        let p1: Profile = Profile {
            firstname: "Person 1".to_owned(),
            lastname: "L1".to_owned(),
            pfp: "".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };
        let p2: Profile = Profile {
            firstname: "Person 2".to_owned(),
            lastname: "L2".to_owned(),
            pfp: "".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };
        let p3: Profile = Profile {
            firstname: "Person 3".to_owned(),
            lastname: "L3".to_owned(),
            pfp: "".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };
        let p4: Profile = Profile {
            firstname: "Person 4".to_owned(),
            lastname: "L4".to_owned(),
            pfp: "".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };

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
        for (id, ts) in &conns {

            let mut lwr = 0;
            let mut hgr = 0;

            if *id < *my {
                lwr = *id;
                hgr = *my;
            } else {
                lwr = *my;
                hgr = *id;

            }

            match self.graph.get_mut(&lwr) {
                Some(m) => match m.get_mut(&hgr) {
                    Some(old_ts) => {
                        if *old_ts > *ts {
                            *old_ts = *ts
                        };
                    }
                    None => {
                        m.insert(hgr, *ts);
                    }
                },
                None => {
                    let mut newmap = HashMap::new();
                    newmap.insert(hgr, *ts);
                    self.graph.insert(lwr, newmap);
                }
            }
        }
    }


    pub fn get_graphData(&self) -> GraphDataPacket {
        let mut profiles = Vec::new();
        for (k, v) in self.people.iter() {
            profiles.push((*k, v.clone()));
        }
        GraphDataPacket {
            edges: self.get_edges(),
            people: profiles,
        }
    }

    fn get_edges(&self) -> Vec<edge> {
        let mut out = Vec::new();
        for (from, tomap) in &self.graph {
            for (to, ts) in tomap {
                out.push(edge {
                    from: *from,
                    to: *to,
                    ts: *ts,
                });
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

#[derive(Debug, MultipartForm)]
struct Form {
    #[multipart(limit = "100MB")]
    profilePicture: TempFile,
    firstname: MpText<String>,
    lastname: MpText<String>,
    linkedin: MpText<String>,
    email: MpText<String>,
    scan: MpText<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScanData {
    myid: u8,
    scans: Vec<Vec<u64>>,
}

#[post("/upload")]
async fn upload(
    state: web::Data<RwLock<ApiState>>,
    mut payload: MultipartForm<Form>,
) -> impl Responder {
    // iterate over multipart stream
    let mut s = state.write().unwrap();

    let sd: ScanData = serde_json::from_str(&payload.scan).unwrap();
    let url_path = format!("/static/pfp/{}", sd.myid);
    let file_path = format!("./static/pfp/{}", sd.myid);

    let mut newProfile = Profile {
        firstname: payload.firstname.clone(),
        lastname: payload.lastname.clone(),
        linkedin: payload.linkedin.clone(),
        email: payload.email.clone(),
        pfp: url_path,
    };

    // println!("{:?}", payload.0);

    match s.people.get_mut(&(sd.myid as u64)) {
        Some(old) => *old = newProfile,
        None => drop(s.people.insert(sd.myid as u64, newProfile)),
    };

    fs::copy(payload.profilePicture.file.path(), file_path);

    // ADD RELATIONS

    let conns: Vec<(u64, DateTime<Local>)> = sd
        .scans
        .iter()
        .map(|f| (f[0], (Local::now() - Duration::seconds(f[1] as i64))))
        .collect();
    s.addEdge(&(sd.myid as u64), conns);

    println!("{:?}", s.people);

    println!("{:?}", s.graph);

    HttpResponse::Ok().body(format!("{{}}"))
}
