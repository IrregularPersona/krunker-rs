use crate::error::{Error, Result};
use crate::types::*;
use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use serde_path_to_error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    base_url: String,
    http: HttpClient,
    api_key: String,
    debug: AtomicBool,
    rate_limit: RwLock<Option<RateLimitInfo>>,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let http = HttpClient::builder().build()?;
        Ok(Self {
            inner: Arc::new(ClientInner {
                base_url: "https://gapi.svc.krunker.io/api".to_string(),
                http,
                api_key: api_key.into(),
                debug: AtomicBool::new(false),
                rate_limit: RwLock::new(None),
            }),
        })
    }

    pub fn set_debug(&self, debug: bool) {
        self.inner.debug.store(debug, Ordering::Relaxed);
    }

    pub async fn last_rate_limit(&self) -> Option<RateLimitInfo> {
        self.inner.rate_limit.read().await.clone()
    }

    /// Internal helper to make requests
    async fn request<T: DeserializeOwned>(&self, path: &str, params: &[(&str, String)]) -> Result<T> {
        let url = format!("{}{}", self.inner.base_url, path);
        let mut request = self
            .inner
            .http
            .get(&url)
            .header("X-Developer-API-Key", &self.inner.api_key);

        if !params.is_empty() {
            request = request.query(params);
        }

        let response = request.send().await?;

        // check rate limit headers
        let headers = response.headers();
        let limit = headers
            .get("X-RateLimit-Limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());
        let remaining = headers
            .get("X-RateLimit-Remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());
        let reset = headers
            .get("X-RateLimit-Reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        if let (Some(limit), Some(remaining), Some(reset)) = (limit, remaining, reset) {
            let mut rl = self.inner.rate_limit.write().await;
            *rl = Some(RateLimitInfo {
                limit,
                remaining,
                reset,
            });
        }

        let status = response.status();

        if status.is_success() {
            let body = response.text().await?;
            if self.inner.debug.load(Ordering::Relaxed) {
                println!("DEBUG: Raw response body:\n{}", body);
            }
            let jd = &mut serde_json::Deserializer::from_str(&body);
            serde_path_to_error::deserialize(jd).map_err(|e| {
                let path = e.path().to_string();
                Error::Decode {
                    message: e.into_inner().to_string(),
                    body,
                    field: Some(path),
                }
            })
        } else {
            let body = response.text().await.unwrap_or_default();
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if let Ok(rl) = serde_json::from_str::<RateLimitResponse>(&body) {
                    return Err(Error::RateLimit {
                        retry_after: rl.retry_after,
                    });
                }
            }

            let message = serde_json::from_str::<GenericErrorResponse>(&body)
                .map(|e| e.error)
                .unwrap_or_else(|_| body);

            Err(Error::Api { status, message })
        }
    }

    pub async fn get_player(&self, name: &str) -> Result<Player> {
        self.request(&format!("/player/{}", name), &[]).await
    }

    pub async fn get_player_inventory(&self, name: &str) -> Result<Vec<InventoryItem>> {
        self.request(&format!("/player/{}/inventory", name), &[]).await
    }

    pub async fn get_player_matches(
        &self,
        name: &str,
        page: Option<i32>,
        season: Option<i32>,
    ) -> Result<PlayerMatchesResponse> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(("page", p.to_string()));
        }
        if let Some(s) = season {
            params.push(("season", s.to_string()));
        }
        self.request(&format!("/player/{}/matches", name), &params).await
    }

    pub async fn get_player_posts(&self, name: &str, page: Option<i32>) -> Result<PostsResponse> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(("page", p.to_string()));
        }
        self.request(&format!("/player/{}/posts", name), &params).await
    }

    pub async fn get_match(&self, match_id: i64) -> Result<Match> {
        self.request(&format!("/match/{}", match_id), &[]).await
    }

    pub async fn get_clan(&self, name: &str) -> Result<Clan> {
        self.request(&format!("/clan/{}", name), &[]).await
    }

    pub async fn get_clan_members(
        &self,
        name: &str,
        page: Option<i32>,
    ) -> Result<ClanMembersResponse> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(("page", p.to_string()));
        }
        self.request(&format!("/clan/{}/members", name), &params).await
    }

    pub async fn get_leaderboard(
        &self,
        region: i32,
        page: Option<i32>,
    ) -> Result<LeaderboardResponse> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(("page", p.to_string()));
        }
        self.request(&format!("/leaderboard/{}", region), &params).await
    }

    /// Retrieve information about a specific map.
    /// Note: The map name is case-sensitive.
    pub async fn get_map(&self, name: &str) -> Result<GameMap> {
        self.request(&format!("/map/{}", name), &[]).await
    }

    /// Retrieve the leaderboard for a specific map.
    /// Note: The map name is case-sensitive.
    pub async fn get_map_leaderboard(
        &self,
        name: &str,
        page: Option<i32>,
    ) -> Result<MapLeaderboardResponse> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(("page", p.to_string()));
        }
        self.request(&format!("/map/{}/leaderboard", name), &params).await
    }

    pub async fn get_mods(&self, page: Option<i32>) -> Result<ModsResponse> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(("page", p.to_string()));
        }
        self.request("/mods", &params).await
    }

    pub async fn get_mod(&self, name: &str) -> Result<Mod> {
        self.request(&format!("/mods/{}", name), &[]).await
    }

    pub async fn get_market_skin(
        &self,
        skin_index: i32,
        page: Option<i32>,
    ) -> Result<MarketResponse> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(("page", p.to_string()));
        }
        self.request(&format!("/market/skin/{}", skin_index), &params).await
    }
}
