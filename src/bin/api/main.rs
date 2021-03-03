#![feature(proc_macro_hygiene, decl_macro)]
use std::collections::HashMap;

use anyhow::Result;
use challenges::groups::{group_config::GroupConfig, group_manager::GroupManager};
use itertools::Itertools;
use rocket::http::Method;
use rocket_contrib::{json::Json, serve::StaticFiles};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Debug)]
pub struct ItemName {
    key: String,
    display_name: String,
}

fn get_groups() -> Vec<ItemName> {
    let manager = GroupManager::new();
    return manager
        .get_group_names()
        .iter()
        .map(|g| ItemName {
            key: g.to_lowercase().replace(" ", "-").clone(),
            display_name: (*g).clone(),
        })
        .collect();
}

fn get_challenges(group: &Box<dyn GroupConfig>) -> Vec<ItemName> {
    return group
        .challenge_names()
        .iter()
        .map(|c| ItemName {
            key: c.to_lowercase().replace(" ", "-").clone(),
            display_name: (*c).clone(),
        })
        .collect();
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
    // Find the group name
    let groups = get_groups();
    let group_name = match groups.iter().find(|&g| g.key == group_key) {
        Some(s) => s,
        None => return None,
    }
    .clone();

    // Get the group
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
    variables: Vec<String>,
}

#[get("/groups/<group_key>/<challenge_key>")]
fn challenge(group_key: String, challenge_key: String) -> Option<Json<Challenge>> {
    // Find the group name
    let groups = get_groups();
    let group_name = match groups.iter().find(|&g| g.key == group_key) {
        Some(s) => s,
        None => return None,
    }
    .clone();

    // Get the group
    let manager = GroupManager::new();
    let group = match manager.get_group(&group_name.display_name) {
        Some(g) => g,
        None => return None,
    };

    // Get the challenge
    let challenge = match group.challenge(&challenge_key) {
        Some(c) => c,
        None => return None,
    };

    Some(Json(Challenge {
        title: challenge.title().to_owned(),
        description: challenge.description().to_owned(),
        variables: challenge.variables(),
    }))
}

#[post(
    "/groups/<group_key>/challenges/<challenge_key>/solve",
    format = "application/json",
    data = "<input>"
)]
fn solve(
    group_key: String,
    challenge_key: String,
    input: Json<HashMap<String, String>>,
) -> Option<Result<String>> {
    // Find the group name
    let groups = get_groups();
    let group_name = match groups.iter().find(|&g| g.key == group_key) {
        Some(s) => s,
        None => return None,
    }
    .clone();

    // Get the group
    let manager = GroupManager::new();
    let group = match manager.get_group(&group_name.display_name) {
        Some(g) => g,
        None => return None,
    };

    // Get the challenge
    let challenge = match group.challenge(&challenge_key) {
        Some(c) => c,
        None => return None,
    };

    Some(challenge.solve_string(input.into_inner()))
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
