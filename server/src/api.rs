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
            firstname: "Petr".to_owned(),
            lastname: "Olsan".to_owned(),
            pfp: "/static/petr.jpeg".to_owned(),
            linkedin: "https://www.linkedin.com/in/petr-olsan/".to_owned(),
            email: "".to_owned(),
        };
        let p1: Profile = Profile {
            firstname: "Mateusz".to_owned(),
            lastname: "Pietrzkiewicz".to_owned(),
            pfp: "/static/matthew.jpeg".to_owned(),
            linkedin: "https://www.linkedin.com/in/mateusz-pietrzkiewicz-557a04254/".to_owned(),
            email: "".to_owned(),
        };
        let p2: Profile = Profile {
            firstname: "Alex".to_owned(),
            lastname: "Charlton".to_owned(),
            pfp: "/static/alex.jpeg".to_owned(),
            linkedin: "https://www.linkedin.com/in/alexander-charlton-404186259/".to_owned(),
            email: "".to_owned(),
        };
        let p3: Profile = Profile {
            firstname: "Quentin".to_owned(),
            lastname: "Duff".to_owned(),
            pfp: "/static/quentin.jpeg".to_owned(),
            linkedin: "https://www.linkedin.com/in/quentin-duff/".to_owned(),
            email: "".to_owned(),
        };
        let p4: Profile = Profile {
            firstname: "Rahul".to_owned(),
            lastname: "Ganish".to_owned(),
            pfp: "/static/rahul.jpeg".to_owned(),
            linkedin: "https://www.linkedin.com/in/rahul-ganesh-b9404b23b/".to_owned(),
            email: "".to_owned(),
        };
        // p5: Emily Johnson (female)
        let p5: Profile = Profile {
            firstname: "Emily".to_owned(),
            lastname: "Johnson".to_owned(),
            pfp: "https://randomuser.me/api/portraits/women/10.jpg".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };

        // p6: Michael Smith (male)
        let p6: Profile = Profile {
            firstname: "Michael".to_owned(),
            lastname: "Smith".to_owned(),
            pfp: "https://randomuser.me/api/portraits/men/10.jpg".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };

        // p7: Sarah Williams (female)
        let p7: Profile = Profile {
            firstname: "Sarah".to_owned(),
            lastname: "Williams".to_owned(),
            pfp: "https://randomuser.me/api/portraits/women/11.jpg".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };

        // p8: David Brown (male)
        let p8: Profile = Profile {
            firstname: "David".to_owned(),
            lastname: "Brown".to_owned(),
            pfp: "https://randomuser.me/api/portraits/men/11.jpg".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };

        // p9: Olivia Jones (female)
        let p9: Profile = Profile {
            firstname: "Olivia".to_owned(),
            lastname: "Jones".to_owned(),
            pfp: "https://randomuser.me/api/portraits/women/12.jpg".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };

        // p10: Daniel Miller (male)
        let p10: Profile = Profile {
            firstname: "Daniel".to_owned(),
            lastname: "Miller".to_owned(),
            pfp: "https://randomuser.me/api/portraits/men/12.jpg".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };

        // p11: Sophia Davis (female)
        let p11: Profile = Profile {
            firstname: "Sophia".to_owned(),
            lastname: "Davis".to_owned(),
            pfp: "https://randomuser.me/api/portraits/women/13.jpg".to_owned(),
            linkedin: "".to_owned(),
            email: "".to_owned(),
        };

        
        state.people.insert(0, p0);
        state.people.insert(1, p1);
        state.people.insert(2, p2);
        state.people.insert(3, p3);
        state.people.insert(4, p4);
        state.people.insert(5, p5);
        state.people.insert(6, p6);
        state.people.insert(7, p7);
        state.people.insert(8, p8);
        state.people.insert(9, p9);
        state.people.insert(10, p10);
        state.people.insert(11, p11);
        

        // p0scans
        let mut p0scans = HashMap::new();
        p0scans.insert(4 as u64, Local::now() + Duration::minutes(10));
        state.graph.insert(0 as u64, p0scans);

        // p1scans
        let mut p1scans = HashMap::new();
        p1scans.insert(4 as u64, Local::now() + Duration::minutes(25));
        p1scans.insert(3 as u64, Local::now() + Duration::minutes(10));
        p1scans.insert(2 as u64, Local::now() + Duration::minutes(20));
        state.graph.insert(1 as u64, p1scans);

        // p2scans
        let mut p2scans = HashMap::new();
        state.graph.insert(2 as u64, p2scans);

        // p3scans
        let mut p3scans = HashMap::new();
        p3scans.insert(4 as u64, Local::now() + Duration::minutes(5));
        state.graph.insert(3 as u64, p3scans);

        // p4scans
        let mut p4scans = HashMap::new();
        state.graph.insert(4 as u64, p4scans);

        // Person 5 connects to persons 8 and 9.
        let mut p5scans = HashMap::new();
        p5scans.insert(6, Local::now() + Duration::minutes(5));
        p5scans.insert(7, Local::now() + Duration::minutes(15));
        state.graph.insert(5, p5scans);

        // Person 6 connects to person 10.
        let mut p6scans = HashMap::new();
        p6scans.insert(8, Local::now() + Duration::minutes(10));
        state.graph.insert(6, p6scans);

        // Person 7 connects to person 11.
        let mut p7scans = HashMap::new();
        p7scans.insert(8, Local::now() + Duration::minutes(10));
        state.graph.insert(7, p7scans);

        // Person 8 connects to person 0.
        let mut p8scans = HashMap::new();
        p8scans.insert(3, Local::now() + Duration::minutes(30));
        state.graph.insert(8, p8scans);

        // Person 9 connects to person 2.
        let mut p9scans = HashMap::new();
        p9scans.insert(2, Local::now() + Duration::minutes(40));
        state.graph.insert(9, p9scans);

        // Person 10 connects to person 4.
        let mut p10scans = HashMap::new();
        p10scans.insert(4, Local::now() + Duration::minutes(20));
        state.graph.insert(10, p10scans);

        // Person 11 connects to person 3.
        let mut p11scans = HashMap::new();
        p11scans.insert(3, Local::now()+ Duration::minutes(30));
        state.graph.insert(11, p11scans);

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
