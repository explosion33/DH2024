#![feature(proc_macro_hygiene, decl_macro)]

use crate::ai::call_and_parse;
use crate::db;

use std::path::{Path, PathBuf};

use mongodb::Client;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response, State};
use rocket::{
    self, fs::NamedFile, Config
};

use rocket::serde::{Serialize, Deserialize, json::Json};


//mongodb+srv://admin:admin@dubhacks.rvpym.mongodb.net/
//mongodb+srv://admin:admin@dubhacks.rvpym.mongodb.net/?retryWrites=true&w=majority&appName=DubHacks

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct MatchRequest {
    pub uid: String,
    pub limit: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Match {
    pub uid: String,
    pub percent: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Matches {
    pub uid: String,
    pub matches: Vec<Match>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct User {
    pub uid: String,
    pub first: String,
    pub last: String,
    pub contact: Vec<String>,
    pub skills: Vec<String>,
    pub wants: Vec<String>,
}

#[rocket::get("/api/static/<file>")]
async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[rocket::get("/api/matches/<uid>/<limit>")]
async fn matches(state: &State<Client>, uid: String, limit: usize) -> Result<Json<Matches>, Status> {
    let mut matches = match db::get_matches(state.inner(), uid.clone()).await {
        Ok(n) => n,
        Err(_) => {return Err(Status::InternalServerError);}
    };

    println!("got matches for {}: {:?}", uid, matches);

    matches.matches.truncate(limit);
    Ok(Json(matches))
}

#[rocket::get("/api/users/<limit>")]
async fn users(state: &State<Client>, limit: usize) -> Json<Vec<User>> {
    Json(db::get_random_users(state.inner(), limit).await)
}

#[rocket::get("/api/info/<uid>")]
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

#[rocket::get("/api/ai")]
async fn get_api(state: &State<Client>) -> Status {
    let users = db::get_random_users(state.inner(), 10).await;

    let res = match call_and_parse(users).await {
        Ok(n) => n,
        Err(_) => {return Status::InternalServerError;}
    };

    match db::update_matches(state.inner(), res).await {
        true => Status::Ok,
        false => Status::InternalServerError,
    }

}

#[rocket::put("/api/info", format = "json", data = "<user>")]
async fn add_user(state: &State<Client>, user: Json<User>) -> Status {
    println!("user: {:?}", user);

    let res = db::add_user(state.inner(), user.into_inner()).await;
    
    if res {
        let users = db::get_random_users(state.inner(), 10).await;

        let res = match call_and_parse(users).await {
            Ok(n) => n,
            Err(_) => {return Status::InternalServerError;}
        };

        match db::update_matches(state.inner(), res).await {
            true => Status::Ok,
            false => Status::InternalServerError,
        }
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
            .mount("/", rocket::routes![get_file, matches, users, info, add_user, get_api])
            .manage(client)
            .launch()
            .await;
        });
}