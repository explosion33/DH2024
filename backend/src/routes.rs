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

#[rocket::get("/static/<file>")]
async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[rocket::get("/matches", format = "json", data = "<req>")]
async fn matches(state: &State<Client>, req: Json<MatchRequest>) -> Result<Json<Matches>, Status> {
    let mut matches = match db::get_matches(state.inner(), req.uid.clone()).await {
        Ok(n) => n,
        Err(_) => {return Err(Status::InternalServerError);}
    };

    println!("got matches for {}: {:?}", req.uid, matches);

    matches.matches.truncate(req.limit);
    Ok(Json(matches))
}

#[rocket::get("/users/<limit>")]
async fn users(state: &State<Client>, limit: usize) -> Json<Vec<User>> {
    Json(db::get_random_users(state.inner(), limit).await)
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

#[rocket::get("/ai")]
async fn get_api(state: &State<Client>) -> Result<Json<Vec<Matches>>, Json<()>>{
    let users = db::get_random_users(state.inner(), 10).await;

    let res = match call_and_parse(users).await {
        Ok(n) => n,
        Err(_) => {return Err(Json(()));}
    };

    Ok(Json(res))

}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:3000",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::put("/info", format = "json", data = "<user>")]
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
            .attach(CORS)
            .launch()
            .await;
        });
}