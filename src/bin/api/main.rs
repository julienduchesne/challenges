#![feature(proc_macro_hygiene, decl_macro)]
use std::collections::HashMap;

use challenges::groups::group_manager::GroupManager;
use rocket::http::Method;
use rocket_contrib::{json::Json, serve::StaticFiles};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Debug)]
pub struct GroupName {
    key: String,
    display_name: String,
}

fn get_group_map() -> HashMap<String, String> {
    let mut group_map = HashMap::new();
    let manager = GroupManager::new();
    for group_name in manager.get_group_names() {
        group_map.insert(group_name.to_lowercase().replace(" ", "-"), group_name);
    }
    return group_map;
}

#[get("/groups")]
fn groups() -> Json<Vec<GroupName>> {
    Json(
        get_group_map()
            .iter()
            .map(|(k, v)| GroupName {
                key: k.clone(),
                display_name: v.clone(),
            })
            .collect(),
    )
}

#[derive(Serialize, Debug)]
pub struct Group {
    name: String,
    url: String,
    challenges: Vec<String>,
}

#[get("/groups/<key>")]
fn group(key: String) -> Json<Group> {
    let group_name = get_group_map().get(&key).unwrap().clone();
    let manager = GroupManager::new();
    let group = manager.get_group(&group_name).unwrap();
    Json(Group {
        name: group.name().to_owned(),
        url: group.url().to_owned(),
        challenges: group.challenge_names(),
    })
}

#[derive(Serialize, Debug)]
pub struct Challenge {
    title: String,
    description: String,
    variables: Vec<String>,
}

#[get("/groups/<key>/challenges/<challenge>")]
fn challenge(key: String, challenge: String) -> Json<Challenge> {
    let group_name = get_group_map().get(&key).unwrap().clone();
    let manager = GroupManager::new();
    let group = manager.get_group(&group_name).unwrap();
    let challenge = group.challenge(&challenge).unwrap();
    Json(Challenge {
        title: challenge.title().to_owned(),
        description: challenge.description().to_owned(),
        variables: challenge.variables(),
    })
}

#[post(
    "/groups/<key>/challenges/<challenge>/solve",
    format = "application/json",
    data = "<input>"
)]
fn solve(key: String, challenge: String, input: Json<HashMap<String, String>>) -> String {
    let group_name = get_group_map().get(&key).unwrap().clone();
    let manager = GroupManager::new();
    let group = manager.get_group(&group_name).unwrap();
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
    .to_cors()
    .unwrap();
    rocket::ignite()
        .mount("/", StaticFiles::from("frontend/build"))
        .mount("/api/", routes![groups, group, challenge, solve])
        .attach(cors)
        .launch();
}
