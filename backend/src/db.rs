use core::f32;
use std::u8;

use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Serialize, Deserialize};
use crate::routes::User;

pub async fn init() -> Client {
    let options = ClientOptions::parse("mongodb+srv://admin:admin@dubhacks.rvpym.mongodb.net/").await.expect("DB Options Error");
    let client = Client::with_options(options).expect("Could not connect to DB");

    return client;
}

pub async fn get_user(client: &Client, uid: String) -> Result<Option<User>, ()> {
    let db = client.database("E2");
    let res =  db.collection::<User>("users").find_one(doc!{"uid": uid.clone()}).await;

    match res {
        Ok(n) => Ok(n),
        Err(_) => Err(())
    }
}

pub async fn add_user(client: &Client, user: User) -> bool {
    let db = client.database("E2");
    let res = db.collection::<User>("users").insert_one(user).await;

    match res {
        Ok(_) => true,
        Err(_) => false,
    }
    
}