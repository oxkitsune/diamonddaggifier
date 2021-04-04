use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

// Error code taken from https://github.com/caelunshun/mojang-api-rs/blob/master/src/lib.rs
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for this crate.
#[derive(Debug)]
pub enum Error {
    /// Indicates that an HTTP error occurred.
    Http(reqwest::Error),
    /// Indicates that the response included malformed JSON.
    /// This could also indicate that, for example, authentication
    /// failed, because the response would have unexpected fields.
    Json(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), fmt::Error> {
        match self {
            Error::Http(e) => write!(f, "{}", e)?,
            Error::Json(e) => write!(f, "{}", e)?,
        }
        Ok(())
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::Http(e1), Error::Http(e2)) => e1.to_string() == e2.to_string(),
            (Error::Json(e1), Error::Json(e2)) => e1.to_string() == e2.to_string(),
            _ => false,
        }
    }
}

impl std::error::Error for Error {}

#[derive(Serialize, Deserialize)]
pub struct PlayerProfile {
    #[serde(rename = "uuid")]
    pub uuid: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "username_history")]
    pub username_history: Vec<UsernameHistory>,

    #[serde(rename = "textures")]
    pub textures: Textures,

    #[serde(rename = "created_at")]
    pub created_at: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Textures {
    #[serde(rename = "custom")]
    pub custom: bool,

    #[serde(rename = "slim")]
    pub slim: bool,

    #[serde(rename = "skin")]
    pub skin: Skin,

    #[serde(rename = "raw")]
    pub raw: Raw,
}

#[derive(Serialize, Deserialize)]
pub struct Raw {
    #[serde(rename = "value")]
    pub value: String,

    #[serde(rename = "signature")]
    pub signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct Skin {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct UsernameHistory {
    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "changed_at")]
    pub changed_at: Option<String>,
}

pub async fn get_profile(username: &str) -> Result<PlayerProfile> {
    let url = format!("https://api.ashcon.app/mojang/v2/user/{}", username);
    let string = Client::new()
        .get(&url)
        .send()
        .await
        .map_err(Error::Http)?
        .text()
        .await
        .map_err(Error::Http)?;

    let profile = serde_json::from_str(&string).map_err(Error::Json)?;

    Ok(profile)
}

pub async fn get_skin_bytes(url: &str) -> Result<Vec<u8>> {
    let bytes = Client::new()
        .get(url)
        .send()
        .await
        .map_err(Error::Http)?
        .bytes()
        .await
        .map_err(Error::Http)?;

    Ok(bytes.to_vec())
}
