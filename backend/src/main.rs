#![feature(proc_macro_hygiene, decl_macro)]

mod api;
mod mojang;
mod skin_creation;

extern crate image;
extern crate reqwest;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
extern crate rocket_download_response;
extern crate serde;
extern crate tokio;

use rocket::http::Method;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::Path,
    sync::{Arc, Mutex, RwLock},
};

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if !Path::new("./skins/cache/").exists() {
        fs::create_dir_all("./skins/cache/");
    }

    if !Path::new("./skins/modified/").exists() {
        fs::create_dir_all("./skins/modified/");
    }

    let allowed_origins = AllowedOrigins::All;

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .attach(cors)
        .mount("/", routes![api::name, api::skin, api::skin_minecraft, api::skin_download])
        .manage(api::APIState {
            requests: Arc::new(RwLock::new(HashMap::new())),
        })
        .launch()
        .await?;

    Ok(())
}
