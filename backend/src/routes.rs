#![feature(proc_macro_hygiene, decl_macro)]


use std::path::{Path, PathBuf};

use rocket::{
    self, fs::NamedFile, Config
};

use rocket::serde::{Serialize, Deserialize, json::Json};

//mongodb+srv://admin:admin@dubhacks.rvpym.mongodb.net/
//mongodb+srv://admin:admin@dubhacks.rvpym.mongodb.net/?retryWrites=true&w=majority&appName=DubHacks

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct MatchRequest {
    uid: String,
    limit: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Match {
    uid: String,
    percent: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct User {
    first: String,
    last: String,
    contact: Vec<String>,
    skills: Vec<String>,
    wants: Vec<String>,
}

#[rocket::get("/static/<file>")]
async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[rocket::get("/matches", format = "json", data = "<req>")]
async fn matches(req: Json<MatchRequest>) -> Json<Vec<Match>> {
    println!("matches: {:?}", req);

    let mut vec: Vec<Match> = Vec::new();

    vec.push(Match {
        uid: "user1".to_string(),
        percent: 100.0,
    });

    vec.push(Match {
        uid: "user2".to_string(),
        percent: 84.3,
    });

    Json(vec)

}

#[rocket::get("/skills/<limit>")]
async fn skills(limit: i32) -> Json<Vec<String>> {
    println!("limit: {:?}", limit);

    let mut vec: Vec<String> = Vec::new();

    
    vec.push("uid1".to_string());
    vec.push("uid2".to_string());

    Json(vec)
}

#[rocket::get("/info/<uid>")]
async fn info(uid: String) -> Json<User> {
    println!("uid: {:?}", uid);

    Json(User {
        first: "Ethan".to_string(),
        last: "Armstrong".to_string(),
        contact: vec!["949-424-4530".to_string(), "warmst@uw.edu".to_string()],
        skills: vec!["programming".to_string(), "being cool".to_string()],
        wants: vec!["ui design".to_string()]
    })
}

pub fn start_api() {
    rocket::tokio::runtime::Builder::new_multi_thread()
        .worker_threads(Config::from(Config::figment()).workers)
        // NOTE: graceful shutdown depends on the "rocket-worker" prefix.
        .thread_name("rocket-worker-thread")
        .enable_all()
        .build()
        .expect("create tokio runtime")
        .block_on(async move {
            let _ = rocket::build()
            .mount("/", rocket::routes![get_file, matches, skills, info])
            //.manage()
            .launch()
            .await;
        });
}