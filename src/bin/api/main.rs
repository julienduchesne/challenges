#![feature(proc_macro_hygiene, decl_macro)]
use std::collections::HashMap;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use challenges::groups::{challenge_config::ChallengeConfig, group_manager::GroupManager};
use rocket_contrib::{json::Json, serve::StaticFiles};
use serde::Serialize;
use rocket::http::Method;
#[macro_use]
extern crate rocket;

#[derive(Serialize, Debug)]
pub struct Group {
    name: String,
    url: String,
    challenges: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Challenge {
    title: String,
    description: String,
    variables: Vec<String>,
}

#[get("/groups")]
fn groups() -> Json<Vec<String>> {
    let manager = GroupManager::new();
    Json(manager.get_group_names())
}

#[get("/groups/<name>")]
fn group(name: String) -> Json<Group> {
    let manager = GroupManager::new();
    let group = manager.get_group(&name).unwrap();
    Json(Group {
        name: group.name().to_owned(),
        url: group.url().to_owned(),
        challenges: group.challenge_names(),
    })
}

#[get("/groups/<name>/challenges/<challenge>")]
fn challenge(name: String, challenge: String) -> Json<Challenge> {
    let manager = GroupManager::new();
    let group = manager.get_group(&name).unwrap();
    let challenge = group.challenge(&challenge).unwrap();
    Json(Challenge {
        title: challenge.title().to_owned(),
        description: challenge.description().to_owned(),
        variables: challenge.variables(),
    })
}

#[post(
    "/groups/<name>/challenges/<challenge>/solve",
    format = "application/json",
    data = "<input>"
)]
fn solve(name: String, challenge: String, input: Json<HashMap<String, String>>) -> String {
    let manager = GroupManager::new();
    let group = manager.get_group(&name).unwrap();
    let challenge = group.challenge(&challenge).unwrap();
    challenge.solve_string(input.into_inner()).unwrap()
}

fn main() {
    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();
    rocket::ignite()
        .mount("/", StaticFiles::from("frontend/build"))
        .mount("/api/", routes![groups, group, challenge, solve])
        .attach(cors)
        .launch();
}
