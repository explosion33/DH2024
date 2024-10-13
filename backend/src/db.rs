use core::f32;
use std::u8;

use mongodb::{bson::doc, options::{ClientOptions, FindOptions}, Client};
use rocket::futures::{StreamExt, TryStreamExt};
use serde::{Serialize, Deserialize};
use crate::routes::{Matches, User};

pub async fn init() -> Client {
    let options = ClientOptions::parse("mongodb+srv://admin:admin@dubhacks.rvpym.mongodb.net/?retryWrites=true&w=majority&appName=DubHacks").await.expect("DB Options Error");
    let client = Client::with_options(options).expect("Could not connect to DB");

    return client;
}

pub async fn get_user(client: &Client, uid: String) -> Result<Option<User>, ()> {
    let db = client.database("E2");
    let res =  db.collection::<User>("users").find_one(doc!{"uid": uid.clone()}).await;

    match res {
        Ok(n) => Ok(n),
        Err(e) => {
            println!("Error | {:?}", e);
            Err(())
        }
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


pub async fn update_matches(client: &Client, matches: Vec<Matches>) -> bool {
    let db = client.database("E2");
    match db.collection::<Matches>("matches").delete_many(doc! {}).await {
        Ok(_) => {},
        Err(_) => {return false;},
    }

    let res = db.collection::<Matches>("matches").insert_many(matches).await;

    match res {
        Ok(_) => true,
        Err(_) => false,
    }    
}

pub async fn get_matches(client: &Client, uid: String) -> Result<Matches, ()> {
    let db = client.database("E2");
    match db.collection::<Matches>("matches").find_one(doc!{"uid": uid.clone()}).await {
        Ok(n) => {
            match n {
                Some(matches) => Ok(matches),
                None => Err(()),
            }
        },
        
        Err(_) => Err(()),
    }
}

pub async fn get_random_users(client: &Client, limit: usize) -> Vec<User> {
    let mut out: Vec<User> = Vec::new();

    let db = client.database("E2");

    // Execute the aggregation query
    match db.collection::<User>("users").find(doc! {}).await {
        Ok(mut n) => {
            let mut err = 0;
            while let Some(doc) = n.next().await {
                match doc {
                    Ok(user) => {
                        out.push(user);
                        
                    },
                    Err(_) => {
                        err += 1;
                    },
                }

                if out.len() >= limit || err >= 5 {
                    break;
                }
            };
            out
        },
        Err(_) => vec![],
    }
}

