#![feature(proc_macro_hygiene, decl_macro)]

use anyhow::Result;
use challenges::groups::{group_config::GroupConfig, group_manager::GroupManager};
use regex::Regex;
use rocket::{http::Method, Data};
use rocket_contrib::{json::Json, serve::StaticFiles};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Clone, Serialize, Debug)]
pub struct ItemName {
    key: String,
    display_name: String,
}

fn create_key(display_name: &str) -> String {
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    return re
        .replace_all(display_name, "-")
        .to_lowercase()
        .replace("--", "-");
}

fn get_groups() -> Vec<ItemName> {
    let manager = GroupManager::new();
    return manager
        .get_group_names()
        .iter()
        .map(|g| ItemName {
            key: create_key(g),
            display_name: (*g).clone(),
        })
        .collect();
}

fn get_group_name(key: &str) -> Option<ItemName> {
    let groups = get_groups();
    return match groups.iter().find(|g| g.key == key) {
        Some(s) => Some(s.clone()),
        None => None,
    };
}

fn get_challenges(group: &Box<dyn GroupConfig>) -> Vec<ItemName> {
    return group
        .challenge_names()
        .iter()
        .map(|c| ItemName {
            key: create_key(c),
            display_name: (*c).clone(),
        })
        .collect();
}

fn get_challenge_name(group: &Box<dyn GroupConfig>, key: &str) -> Option<ItemName> {
    let challenges = get_challenges(group);
    return match challenges.iter().find(|g| g.key == key) {
        Some(s) => Some(s.clone()),
        None => None,
    };
}

#[get("/groups")]
fn groups() -> Json<Vec<ItemName>> {
    Json(get_groups())
}

#[derive(Serialize, Debug)]
pub struct Group {
    name: String,
    url: String,
    challenges: Vec<ItemName>,
}

#[get("/groups/<group_key>")]
fn group(group_key: String) -> Option<Json<Group>> {
    // Get the group
    let group_name = match get_group_name(&group_key) {
        Some(g) => g,
        None => return None,
    };
    let manager = GroupManager::new();
    let group = match manager.get_group(&group_name.display_name) {
        Some(g) => g,
        None => return None,
    };

    Some(Json(Group {
        name: group.name().to_owned(),
        url: group.url().to_owned(),
        challenges: get_challenges(&group),
    }))
}

#[derive(Serialize, Debug)]
pub struct Challenge {
    title: String,
    description: String,
}

#[get("/groups/<group_key>/<challenge_key>")]
fn challenge(group_key: String, challenge_key: String) -> Option<Json<Challenge>> {
    // Get the group
    let group_name = match get_group_name(&group_key) {
        Some(g) => g,
        None => return None,
    };
    let manager = GroupManager::new();
    let group = match manager.get_group(&group_name.display_name) {
        Some(g) => g,
        None => return None,
    };

    // Get the challenge
    let challenge_name = match get_challenge_name(group, &challenge_key) {
        Some(g) => g,
        None => return None,
    };
    let challenge = match group.challenge(&challenge_name.display_name) {
        Some(c) => c,
        None => return None,
    };

    Some(Json(Challenge {
        title: challenge.title().to_owned(),
        description: challenge.description().to_owned(),
    }))
}

#[post(
    "/groups/<group_key>/<challenge_key>/solve",
    format = "text/plain",
    data = "<input>"
)]
fn solve(group_key: String, challenge_key: String, input: Data) -> Option<Result<String>> {
    // Get the group
    let group_name = match get_group_name(&group_key) {
        Some(g) => g,
        None => return None,
    };
    let manager = GroupManager::new();
    let group = match manager.get_group(&group_name.display_name) {
        Some(g) => g,
        None => return None,
    };

    // Get the challenge
    let challenge_name = match get_challenge_name(group, &challenge_key) {
        Some(g) => g,
        None => return None,
    };
    let challenge = match group.challenge(&challenge_name.display_name) {
        Some(c) => c,
        None => return None,
    };

    let mut data_bytes: Vec<u8> = vec![];
    match input.stream_to(&mut data_bytes) {
        Err(err) => return Some(Err(err.into())),
        _ => {}
    };
    return match String::from_utf8(data_bytes) {
        Ok(data) => Some(challenge.solve(&data)),
        Err(err) => Some(Err(err.into())),
    };
}

fn main() {
    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
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
