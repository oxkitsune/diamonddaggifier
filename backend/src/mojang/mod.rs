use rocket::http::Status;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fmt,
    fs::File,
    io::{copy, Cursor},
};

use crate::api::APIError;

#[derive(Debug)]
struct MojangError(String);

impl fmt::Display for MojangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mojang API Error: {}", self.0)
    }
}

impl Error for MojangError {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfile {
    pub id: String,
    pub name: String,
}

pub async fn get_uuid(name: &str) -> Result<PlayerProfile, APIError> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let response = reqwest::get(&url).await.ok();

    if response.is_none() {
        return Err(APIError(
            "Failed to get uuid from Mojang API".to_string(),
            rocket::http::Status::NotFound,
        ));
    }

    let unwrapped_response = response.unwrap();

    if unwrapped_response.status() != 200 {
        return Err(APIError(
            "Failed to get uuid from Mojang API (Failed to unwrap response)".to_string(),
            rocket::http::Status::NotFound,
        ));
    }

    let body = unwrapped_response.json::<HashMap<String, String>>().await;

    match body {
        Ok(map) => {
            if map.contains_key(&"error".to_string()) {
                let error_msg = map.get(&"errorMessage".to_string()).unwrap().clone();
                return Err(APIError(error_msg.to_string(), Status::InternalServerError));
            }

            let name = map.get(&"name".to_string()).unwrap().clone();
            let uuid = map.get(&"id".to_string()).unwrap().clone();

            Ok(PlayerProfile { id: uuid, name: name })
        }
        Err(_) => {
            return Err(APIError(
                "Failed to read uuid response from Mojang (is it malformed?)".to_string(),
                Status::InternalServerError,
            ));
        }
    }
}

pub async fn download_skin(uuid: &str) -> Result<(), APIError> {
    let url = format!("https://crafatar.com/skins/{}", uuid);
    let response = reqwest::get(&url).await;

    if response.is_err() {
        return Err(APIError("Failed to download user skin!".to_string(), Status::InternalServerError));
    }

    let unwrapped_response = response.unwrap();

    println!("unwrapped_response status: {}", unwrapped_response.status());

    if unwrapped_response.status() != 200 {
        return Err(APIError(
            "Failed to unwrap Mojang API response!".to_string(),
            Status::InternalServerError,
        ));
    }

    let bytes = unwrapped_response.bytes().await;

    if bytes.is_err() {
        return Err(APIError("Failed to get byte stream!".to_string(), Status::InternalServerError));
    }

    let mut content = Cursor::new(bytes.unwrap());
    let output_name = format!("./skins/cache/{}.png", uuid);
    match File::create(output_name) {
        Ok(mut output) => {
            let result = copy(&mut content, &mut output);

            match result {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(APIError("Failed to copy skin datastream!".to_string(), Status::InternalServerError));
                }
            }
        }
        Err(err) => {
            return Err(APIError("Failed to create file stream!".to_string(), Status::InternalServerError));
        }
    }
}
