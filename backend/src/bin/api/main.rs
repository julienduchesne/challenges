#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate cached;

use anyhow::Result;
use challenges::groups::group_manager::GroupManager;
use regex::Regex;
use rocket::{http::Method, Config, Data};
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use serde::Serialize;

#[macro_use]
extern crate rocket;

const DEFAULT_PORT: u16 = 8081;

#[derive(Clone, Serialize, Debug)]
pub struct ItemName {
    key: String,
    display_name: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct Group {
    name: String,
    url: String,
    challenges: Vec<ItemName>,
}

fn create_key(display_name: &str) -> String {
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    return re
        .replace_all(display_name, "-")
        .to_lowercase()
        .replace("--", "-");
}

cached! {
    GROUPS;
    fn get_groups() -> Vec<ItemName> = {
        let manager = GroupManager::new();
        let value: Vec<ItemName> = manager
        .get_group_names()
        .iter()
        .map(|g| ItemName {
            key: create_key(g),
            display_name: (*g).clone(),
        })
        .collect();
        return value;
    }
}

cached! {
    GROUP_NAME;
    fn get_group_name(key: String) -> Option<ItemName> = {
        let groups = get_groups();
        return match groups.iter().find(|g| g.key == key) {
            Some(s) => Some(s.clone()),
            None => None,
        };
    }
}

cached! {
    GROUP;
    fn get_group(group_key: String) -> Option<Group> = {
        // Get the group
        let group_name = match get_group_name(group_key) {
            Some(g) => g,
            None => return None,
        };
        let manager = GroupManager::new();
        let group = match manager.get_group(&group_name.display_name) {
            Some(g) => g,
            None => return None,
        };
        let challenges: Vec<ItemName> = group
            .challenge_names()
            .iter()
            .map(|c| ItemName {
                key: create_key(c),
                display_name: (*c).clone(),
            })
            .collect();
        return Some(Group {
            name: group.name().to_owned(),
            url: group.url().to_owned(),
            challenges: challenges,
        });
    }
}

cached! {
    CHALLENGE_NAME;
    fn get_challenge_name(group_key: String, key: String) -> Option<ItemName> = {
        let group = match get_group(group_key) {
            Some(g) => g,
            None => return None,
        };
        return match group.challenges.iter().find(|g| g.key == key) {
            Some(s) => Some(s.clone()),
            None => None,
        };
    }
}

#[get("/groups")]
fn groups() -> Json<Vec<ItemName>> {
    Json(get_groups())
}

#[get("/groups/<group_key>")]
fn group(group_key: String) -> Json<Option<Group>> {
    return Json(get_group(group_key));
}

#[derive(Serialize, Debug)]
pub struct Challenge {
    title: String,
    description: String,
}

#[get("/groups/<group_key>/<challenge_key>")]
fn challenge(group_key: String, challenge_key: String) -> Option<Json<Challenge>> {
    // Get the group
    let group_name = match get_group_name(group_key.clone()) {
        Some(g) => g,
        None => return None,
    };
    // Get the challenge
    let challenge_name = match get_challenge_name(group_key.clone(), challenge_key) {
        Some(g) => g,
        None => return None,
    };
    let manager = GroupManager::new();
    let group = match manager.get_group(&group_name.display_name) {
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
    let group_name = match get_group_name(group_key.clone()) {
        Some(g) => g,
        None => return None,
    };
    // Get the challenge
    let challenge_name = match get_challenge_name(group_key.clone(), challenge_key) {
        Some(g) => g,
        None => return None,
    };
    let manager = GroupManager::new();
    let group = match manager.get_group(&group_name.display_name) {
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
    let port = match std::env::var("CHALLENGES_API_PORT") {
        Ok(p) => p.parse::<u16>().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };
    let mut config = Config::active().unwrap();
    config.set_port(port);
    rocket::custom(config)
        .mount("/api/", routes![groups, group, challenge, solve])
        .attach(cors)
        .launch();
}
