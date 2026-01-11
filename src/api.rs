use crate::client::Client;
pub use crate::types::*;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    Http(reqwest::Error),
    Api {
        status: StatusCode,
        body: String,
    },
    Decode {
        message: String,
        body: String,
    },
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Http(e) => write!(f, "HTTP error: {}", e),
            ApiError::Api { status, body } => write!(f, "API error ({}): {}", status, body),
            ApiError::Decode { message, body } => write!(f, "Decode error: {} | Body: {}", message, body),
        }
    }
}

impl std::error::Error for ApiError {}

fn make_request(client: &Client, path: &str, params: &[(&str, String)]) -> Result<String, ApiError> {
    let url = format!("{}{}", client.client_base_url, path);
    let mut request = client.client.get(&url)
        .header("X-Developer-API-Key", &client.api_key);

    if !params.is_empty() {
        request = request.query(params);
    }

    let response = request.send().map_err(ApiError::Http)?;
    let status = response.status();

    if status.is_success() {
        response.text().map_err(ApiError::Http)
    } else {
        let body = response.text().unwrap_or_default();
        Err(ApiError::Api { status, body })
    }
}

fn request_and_decode<T: DeserializeOwned>(client: &Client, path: &str, params: &[(&str, String)]) -> Result<T, ApiError> {
    let body = make_request(client, path, params)?;
    serde_json::from_str(&body).map_err(|e| ApiError::Decode {
        message: e.to_string(),
        body,
    })
}

pub fn get_player(client: &Client, name: &str) -> Result<Player, ApiError> {
    request_and_decode(client, &format!("/player/{}", name), &[])
}

pub fn get_player_inventory(client: &Client, name: &str) -> Result<Vec<InventoryItem>, ApiError> {
    request_and_decode(client, &format!("/player/{}/inventory", name), &[])
}

pub fn get_player_matches(client: &Client, name: &str, page: Option<i32>, season: Option<i32>) -> Result<PlayerMatchesResponse, ApiError> {
    let mut params = Vec::new();
    if let Some(p) = page { params.push(("page", p.to_string())); }
    if let Some(s) = season { params.push(("season", s.to_string())); }
    request_and_decode(client, &format!("/player/{}/matches", name), &params)
}

pub fn get_player_posts(client: &Client, name: &str, page: Option<i32>) -> Result<PostsResponse, ApiError> {
    let mut params = Vec::new();
    if let Some(p) = page { params.push(("page", p.to_string())); }
    request_and_decode(client, &format!("/player/{}/posts", name), &params)
}

pub fn get_match(client: &Client, match_id: i32) -> Result<Match, ApiError> {
    request_and_decode(client, &format!("/match/{}", match_id), &[])
}

pub fn get_clan(client: &Client, name: &str) -> Result<Clan, ApiError> {
    request_and_decode(client, &format!("/clan/{}", name), &[])
}

pub fn get_clan_members(client: &Client, name: &str, page: Option<i32>) -> Result<ClanMembersResponse, ApiError> {
    let mut params = Vec::new();
    if let Some(p) = page { params.push(("page", p.to_string())); }
    request_and_decode(client, &format!("/clan/{}/members", name), &params)
}

pub fn get_leaderboard(client: &Client, region: i32, page: Option<i32>) -> Result<LeaderboardResponse, ApiError> {
    let mut params = Vec::new();
    if let Some(p) = page { params.push(("page", p.to_string())); }
    request_and_decode(client, &format!("/leaderboard/{}", region), &params)
}

pub fn get_map(client: &Client, name: &str) -> Result<GameMap, ApiError> {
    request_and_decode(client, &format!("/map/{}", name), &[])
}

pub fn get_map_leaderboard(client: &Client, name: &str, page: Option<i32>) -> Result<MapLeaderboardResponse, ApiError> {
    let mut params = Vec::new();
    if let Some(p) = page { params.push(("page", p.to_string())); }
    request_and_decode(client, &format!("/map/{}/leaderboard", name), &params)
}

pub fn get_mods(client: &Client, page: Option<i32>) -> Result<ModsResponse, ApiError> {
    let mut params = Vec::new();
    if let Some(p) = page { params.push(("page", p.to_string())); }
    request_and_decode(client, "/mods", &params)
}

pub fn get_mod(client: &Client, name: &str) -> Result<Mod, ApiError> {
    request_and_decode(client, &format!("/mods/{}", name), &[])
}

pub fn get_market_skin(client: &Client, skin_index: i32, page: Option<i32>) -> Result<MarketResponse, ApiError> {
    let mut params = Vec::new();
    if let Some(p) = page { params.push(("page", p.to_string())); }
    request_and_decode(client, &format!("/market/skin/{}", skin_index), &params)
}
