use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, NamedFile, Responder, Response},
    State,
};
use rocket_contrib::json::{Json, JsonValue};
use rocket_download_response::DownloadResponse;

use std::{
    collections::HashMap,
    io::Cursor,
    net::{IpAddr, SocketAddr},
    path::Path,
    sync::{atomic::AtomicUsize, Arc, Mutex, RwLock},
    time::{Duration, Instant},
};

use crate::mojang;
use crate::skin_creation;

const RATE_LIMIT_RESET: u64 = 90;
const RATE_LIMIT: u32 = 60;

#[derive(Debug)]
pub struct APIError(pub String, pub Status);

impl APIError {
    fn to_json(&self) -> JsonValue {
        json!({"error": &self.0})
    }
}

impl std::error::Error for APIError {}

unsafe impl std::marker::Send for APIError {}

impl std::fmt::Display for APIError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        println!("APIError: {}", self.0);
        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for APIError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let json = self.to_json().to_string();
        Response::build()
            .header(ContentType::JSON)
            .status(self.1)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}

pub struct UserRequestProfile {
    ip: IpAddr,
    count: u32,
    last_request: Instant,
}

pub struct APIState {
    pub requests: Arc<RwLock<HashMap<IpAddr, Mutex<UserRequestProfile>>>>,
}

fn is_ratelimited(ip: IpAddr, state: State<'_, APIState>) -> bool {
    match state.requests.write() {
        Ok(mut map) => {
            if !map.contains_key(&ip) {
                map.insert(
                    ip,
                    Mutex::from(UserRequestProfile {
                        ip: ip,
                        count: 0,
                        last_request: Instant::now(),
                    }),
                );
            }

            match map.get(&ip) {
                Some(mutex) => match mutex.lock() {
                    Ok(mut user_profile) => {
                        if Instant::now().duration_since(user_profile.last_request) >= Duration::from_secs(RATE_LIMIT_RESET) {
                            user_profile.count = 0;
                        }

                        user_profile.last_request = Instant::now();
                        user_profile.count += 1;

                        if user_profile.count >= RATE_LIMIT {
                            return true;
                        }

                        return false;
                    }
                    Err(_) => {
                        return true;
                    }
                },
                None => {
                    return false;
                }
            };
        }
        Err(_) => {
            return true;
        }
    };
}

async fn generate_skin(uuid: String) -> Result<String, APIError> {
    let file_path = format!("./skins/modified/{}.png", uuid);
    if !Path::new(&file_path).exists() {
        let download = mojang::download_skin(&uuid).await;
        match download {
            Ok(_) => {
                let skin_generation = skin_creation::diamondagify(&uuid).await;
                match skin_generation {
                    Ok(_) => {
                        return Ok(file_path.clone());
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(file_path.clone())
}

#[get("/api/uuid/<name>")]
pub async fn name(remote_addr: SocketAddr, state: State<'_, APIState>, name: String) -> Result<JsonValue, APIError> {
    if is_ratelimited(remote_addr.ip(), state) {
        return Err(APIError("You are currently rate-limited!".to_string(), Status::Forbidden));
    }

    match mojang::get_uuid(&name).await {
        Ok(profile) => Ok(json!({"name": &profile.name, "uuid": &profile.id})),
        Err(err) => Err(APIError("Failed to get UUID from Mojang API".to_string(), Status::NotFound)),
    }
}

#[get("/api/skin/<uuid>")]
pub async fn skin(remote_addr: SocketAddr, state: State<'_, APIState>, uuid: String) -> Result<NamedFile, APIError> {
    if is_ratelimited(remote_addr.ip(), state) {
        return Err(APIError("You are currently rate-limited!".to_string(), Status::Forbidden));
    }

    let file_path = generate_skin(uuid).await?;
    match NamedFile::open(&file_path).await {
        Ok(file) => Ok(file),
        Err(err) => Err(APIError(format!("Skin generation failed: {}!", err), Status::InternalServerError)),
    }
}

#[get("/api/skin/<uuid>/minecraft.png")]
pub async fn skin_minecraft(remote_addr: SocketAddr, state: State<'_, APIState>, uuid: String) -> Result<NamedFile, APIError> {
    if is_ratelimited(remote_addr.ip(), state) {
        return Err(APIError("You are currently rate-limited!".to_string(), Status::Forbidden));
    }

    let file_path = format!("./skins/modified/{}.png", uuid);
    if !Path::new(&file_path).exists() {
        mojang::download_skin(&uuid).await;
        skin_creation::diamondagify(&uuid).await;
    }

    match NamedFile::open(&file_path).await {
        Ok(file) => Ok(file),
        Err(err) => Err(APIError(format!("Skin generation failed: {}!", err), Status::InternalServerError)),
    }
}

#[get("/api/skin/<uuid>/download")]
pub async fn skin_download(remote_addr: SocketAddr, state: State<'_, APIState>, uuid: String) -> Result<DownloadResponse, APIError> {
    if is_ratelimited(remote_addr.ip(), state) {
        return Err(APIError("You are currently rate-limited!".to_string(), Status::Forbidden));
    }

    let file_path = format!("./skins/modified/{}.png", uuid);
    let path = Path::new(&file_path);
    if !path.exists() {
        mojang::download_skin(&uuid).await;
        skin_creation::diamondagify(&uuid).await;
    }

    Ok(DownloadResponse::from_file(path, None::<String>, None).await)
}
