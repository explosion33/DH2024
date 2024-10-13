#![feature(proc_macro_hygiene, decl_macro)]

use crate::db;

use std::path::{Path, PathBuf};

use mongodb::Client;
use rocket::http::Status;
use rocket::{State};
use rocket::{
    self, fs::NamedFile, Config
};

use rocket::serde::{Serialize, Deserialize, json::Json};

use futures::executor;


//mongodb+srv://admin:admin@dubhacks.rvpym.mongodb.net/
//mongodb+srv://admin:admin@dubhacks.rvpym.mongodb.net/?retryWrites=true&w=majority&appName=DubHacks

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct MatchRequest {
    uid: String,
    limit: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Match {
    uid: String,
    percent: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct User {
    uid: String,
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
async fn info(state: &State<Client>, uid: String) -> Result<Json<User>, Json<()>> {
    println!("uid: {:?}", uid);

    match db::get_user(state.inner(), uid).await {
        Ok(user) => {
            match user {
                Some(n) => Ok(Json(n)),
                None => Err(Json(())),
            }
        }
        Err(_) => {
            Err(Json(()))
        },
    }
}

#[rocket::put("/info", format = "json", data = "<user>")]
async fn add_user(state: &State<Client>, user: Json<User>) -> Status {
    println!("user: {:?}", user);

    let res = db::add_user(state.inner(), user.into_inner()).await;
    
    if res {
        Status::Ok
    }
    else {
        Status::InternalServerError
    }
    
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
            let client = db::init().await;
            println!("DB Connected");
            let _ = rocket::build()
            .mount("/", rocket::routes![get_file, matches, skills, info, add_user])
            .manage(client)
            .launch()
            .await;
        });
}