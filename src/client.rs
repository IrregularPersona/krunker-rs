use crate::error::{Error, Result};
use crate::types::*;
use reqwest::header::{HeaderMap, HeaderValue, RETRY_AFTER};
use reqwest::{Client as HttpClient, StatusCode, Url};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

const DEFAULT_BASE_URL: &str = "https://gapi.svc.krunker.io/api";
const API_KEY_HEADER: &str = "X-Developer-API-Key";

/// An asynchronous client for the Krunker.io Game API.
///
/// Clone a client to share its connection pool, debug setting, and rate limit
/// snapshot between tasks. Each call to [`Client::new`] creates independent state.
/// Requests require a Tokio runtime and have a 30-second timeout by default.
#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    base_url: Url,
    http: HttpClient,
    api_key: HeaderValue,
    timeout: Duration,
    rate_limit_retries: u32,
    debug: AtomicBool,
    rate_limit: RwLock<Option<RateLimitInfo>>,
}

/// Configures a [`Client`] before building it.
///
/// ```no_run
/// use krunker_rs::Client;
/// use std::time::Duration;
///
/// # fn main() -> krunker_rs::Result<()> {
/// let client = Client::builder("your-api-key")
///     .timeout(Duration::from_secs(20))
///     .connect_timeout(Duration::from_secs(5))
///     .retry_rate_limits(2)
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub struct ClientBuilder {
    api_key: String,
    base_url: String,
    http: Option<HttpClient>,
    timeout: Duration,
    connect_timeout: Duration,
    rate_limit_retries: u32,
    debug: bool,
}

impl ClientBuilder {
    /// Sets the timeout for each attempt, including reading the response body.
    ///
    /// Defaults to 30 seconds. It also applies to an injected HTTP client.
    /// Time spent waiting between rate limit retries is outside this timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the connection timeout for the default HTTP client (10 seconds).
    ///
    /// An injected HTTP client keeps its own connection timeout.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Sets the API root, including its path prefix, for example `/api`.
    ///
    /// Useful for mock servers or a proxy. HTTP and HTTPS URLs are accepted;
    /// credentials, query strings, and fragments are rejected. A trailing slash
    /// is optional. Defaults to `https://gapi.svc.krunker.io/api`.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Uses an existing HTTP client and its connection, TLS, proxy, and redirect settings.
    ///
    /// The request timeout set with [`Self::timeout`] still applies. The default
    /// client does not follow redirects, so the API key stays on the chosen host.
    pub fn http_client(mut self, http: HttpClient) -> Self {
        self.http = Some(http);
        self
    }

    /// Retries HTTP 429 responses up to `max_retries` times after the server's delay.
    ///
    /// Defaults to zero: rate limits are returned immediately. Retry delay is
    /// taken from JSON `retry_after`, then `Retry-After` (seconds or HTTP date),
    /// then `X-RateLimit-Reset`. If none is usable, the delay is one second.
    /// Other HTTP and transport errors are not retried.
    pub fn retry_rate_limits(mut self, max_retries: u32) -> Self {
        self.rate_limit_retries = max_retries;
        self
    }

    /// Enables request metadata on stderr without printing keys or response bodies.
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Validates the configuration and creates a client without making a request.
    pub fn build(self) -> Result<Client> {
        if self.api_key.trim().is_empty() {
            return Err(Error::invalid_input("api_key", "must not be empty"));
        }
        let mut api_key = HeaderValue::from_str(&self.api_key)
            .map_err(|_| Error::invalid_input("api_key", "must be a valid HTTP header value"))?;
        api_key.set_sensitive(true);

        let base_url = Url::parse(&self.base_url).map_err(|_| {
            Error::invalid_input("base_url", "must be an absolute HTTP or HTTPS URL")
        })?;
        if !matches!(base_url.scheme(), "http" | "https")
            || base_url.host_str().is_none()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(Error::invalid_input(
                "base_url",
                "must be an HTTP or HTTPS URL without credentials, a query, or a fragment",
            ));
        }
        if self.timeout.is_zero() {
            return Err(Error::invalid_input("timeout", "must be greater than zero"));
        }
        if self.connect_timeout.is_zero() {
            return Err(Error::invalid_input(
                "connect_timeout",
                "must be greater than zero",
            ));
        }

        let http = match self.http {
            Some(http) => http,
            None => HttpClient::builder()
                .connect_timeout(self.connect_timeout)
                .redirect(reqwest::redirect::Policy::none())
                .user_agent(concat!("krunker-rs/", env!("CARGO_PKG_VERSION")))
                .build()?,
        };

        Ok(Client {
            inner: Arc::new(ClientInner {
                base_url,
                http,
                api_key,
                timeout: self.timeout,
                rate_limit_retries: self.rate_limit_retries,
                debug: AtomicBool::new(self.debug),
                rate_limit: RwLock::new(None),
            }),
        })
    }
}

impl Client {
    /// Creates a client with a 30-second request timeout and no automatic retries.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key).build()
    }

    /// Starts configuring a client. No network request is made until an endpoint is called.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_owned(),
            http: None,
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            rate_limit_retries: 0,
            debug: false,
        }
    }

    /// Toggles metadata logging on stderr for this client and all of its clones.
    ///
    /// Logs method, URL, status, body size, and retry delay, without response
    /// bodies or the API key. Use [`Error::Decode`] to inspect a failed body explicitly.
    pub fn set_debug(&self, debug: bool) {
        self.inner.debug.store(debug, Ordering::Relaxed);
    }

    /// Returns the rate limit headers from the most recently processed response.
    ///
    /// Returns `None` before any response, or when that response does not contain
    /// all three valid headers. Concurrent requests may finish out of order;
    /// this snapshot is advisory and does not reserve or enforce a quota.
    pub async fn last_rate_limit(&self) -> Option<RateLimitInfo> {
        self.inner.rate_limit.read().await.clone()
    }

    async fn request<T: DeserializeOwned>(
        &self,
        path: &[&str],
        params: &[(&str, String)],
    ) -> Result<T> {
        let mut url = self.inner.base_url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::invalid_input("base_url", "cannot contain path segments"))?
            .pop_if_empty()
            .extend(path.iter().copied());

        let mut retries = 0;
        loop {
            let response = self
                .inner
                .http
                .get(url.clone())
                .header(API_KEY_HEADER, self.inner.api_key.clone())
                .query(params)
                .timeout(self.inner.timeout)
                .send()
                .await?;

            let headers = response.headers();
            let rate_limit = rate_limit_info(headers);
            *self.inner.rate_limit.write().await = rate_limit;
            let retry_after_header = headers.get(RETRY_AFTER).cloned();
            let reset: Option<u64> = header_number(headers, "X-RateLimit-Reset");
            let status = response.status();
            // Preserve transport failures even when the HTTP status is an error.
            let body = response.text().await?;

            if self.inner.debug.load(Ordering::Relaxed) {
                eprintln!("krunker-rs: GET {url} -> {status} ({} bytes)", body.len());
            }

            if status.is_success() {
                return decode(body);
            }
            if status == StatusCode::TOO_MANY_REQUESTS {
                let retry_after = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|value| value.get("retry_after").and_then(serde_json::Value::as_u64))
                    .or_else(|| retry_after_header.as_ref().and_then(retry_after_seconds))
                    .or_else(|| reset.and_then(seconds_until_reset))
                    .unwrap_or(1);

                if retries < self.inner.rate_limit_retries {
                    let deadline =
                        tokio::time::Instant::now().checked_add(Duration::from_secs(retry_after));
                    if let Some(deadline) = deadline {
                        if self.inner.debug.load(Ordering::Relaxed) {
                            eprintln!("krunker-rs: rate limited; retrying in {retry_after}s");
                        }
                        tokio::time::sleep_until(deadline).await;
                        retries += 1;
                        continue;
                    }
                }
                return Err(Error::RateLimit { retry_after });
            }

            let message = serde_json::from_str::<GenericErrorResponse>(&body)
                .map(|response| response.error)
                .unwrap_or_else(|_| {
                    if body.is_empty() {
                        status
                            .canonical_reason()
                            .unwrap_or("Empty error response")
                            .to_owned()
                    } else {
                        body
                    }
                });
            return Err(Error::Api { status, message });
        }
    }

    /// Gets a player's profile and lifetime statistics. Pass the unencoded name.
    pub async fn get_player(&self, name: &str) -> Result<Player> {
        validate_name(name)?;
        self.request(&["player", name], &[]).await
    }

    /// Gets skin counts in a player's inventory, excluding items listed on the market.
    pub async fn get_player_inventory(&self, name: &str) -> Result<Vec<InventoryItem>> {
        validate_name(name)?;
        self.request(&["player", name, "inventory"], &[]).await
    }

    /// Gets ranked match history, newest first, usually 10 matches per page.
    ///
    /// Pages start at 1; `None` uses the API default. `season: None` includes all
    /// seasons. A supplied season must be nonnegative.
    pub async fn get_player_matches(
        &self,
        name: &str,
        page: Option<i32>,
        season: Option<i32>,
    ) -> Result<PlayerMatchesResponse> {
        validate_name(name)?;
        let mut params = page_params(page)?;
        if let Some(season) = season {
            validate_nonnegative("season", i64::from(season))?;
            params.push(("season", season.to_string()));
        }
        self.request(&["player", name, "matches"], &params).await
    }

    /// Gets a page of a player's social posts. Pages start at 1; `None` uses page 1.
    pub async fn get_player_posts(&self, name: &str, page: Option<i32>) -> Result<PostsResponse> {
        validate_name(name)?;
        self.request(&["player", name, "posts"], &page_params(page)?)
            .await
    }

    /// Gets the participants and statistics of a ranked match. IDs must be positive.
    pub async fn get_match(&self, match_id: i64) -> Result<Match> {
        if match_id <= 0 {
            return Err(Error::invalid_input(
                "match_id",
                "must be greater than zero",
            ));
        }
        self.request(&["match", &match_id.to_string()], &[]).await
    }

    /// Gets a clan's public profile. Pass the unencoded clan name.
    pub async fn get_clan(&self, name: &str) -> Result<Clan> {
        validate_name(name)?;
        self.request(&["clan", name], &[]).await
    }

    /// Gets clan members ordered by role, then score, usually 10 per page.
    pub async fn get_clan_members(
        &self,
        name: &str,
        page: Option<i32>,
    ) -> Result<ClanMembersResponse> {
        validate_name(name)?;
        self.request(&["clan", name, "members"], &page_params(page)?)
            .await
    }

    /// Gets a ranked leaderboard, highest MMR first, usually 10 entries per page.
    ///
    /// Known region IDs: 2 = Asia, 3 = Europe, 4 = North America. Other nonnegative
    /// IDs are sent to the API so newly supported regions do not require an SDK update.
    pub async fn get_leaderboard(
        &self,
        region: i32,
        page: Option<i32>,
    ) -> Result<LeaderboardResponse> {
        validate_nonnegative("region", i64::from(region))?;
        self.request(&["leaderboard", &region.to_string()], &page_params(page)?)
            .await
    }

    /// Gets a map's details. The name is case-sensitive and must be unencoded.
    pub async fn get_map(&self, name: &str) -> Result<GameMap> {
        validate_name(name)?;
        self.request(&["map", name], &[]).await
    }

    /// Gets a map leaderboard, usually 25 entries per page.
    ///
    /// Map names are case-sensitive. Values follow the returned leaderboard type
    /// and order. An unconfigured leaderboard returns an empty entries array.
    pub async fn get_map_leaderboard(
        &self,
        name: &str,
        page: Option<i32>,
    ) -> Result<MapLeaderboardResponse> {
        validate_name(name)?;
        self.request(&["map", name, "leaderboard"], &page_params(page)?)
            .await
    }

    /// Lists active mods by votes, usually 10 per page. Pages start at 1.
    pub async fn get_mods(&self, page: Option<i32>) -> Result<ModsResponse> {
        self.request(&["mods"], &page_params(page)?).await
    }

    /// Gets a mod's details. Pass the unencoded mod name.
    pub async fn get_mod(&self, name: &str) -> Result<Mod> {
        validate_name(name)?;
        self.request(&["mods", name], &[]).await
    }

    /// Gets market listings, owners, and price history for a nonnegative skin index.
    ///
    /// `page` applies to listings only, usually 10 per page. Owners and price
    /// history are included independently of listing pagination.
    pub async fn get_market_skin(
        &self,
        skin_index: i32,
        page: Option<i32>,
    ) -> Result<MarketResponse> {
        validate_nonnegative("skin_index", i64::from(skin_index))?;
        self.request(
            &["market", "skin", &skin_index.to_string()],
            &page_params(page)?,
        )
        .await
    }
}

fn validate_name(name: &str) -> Result<()> {
    if name.trim().is_empty() || matches!(name, "." | "..") || name.chars().any(char::is_control) {
        return Err(Error::invalid_input(
            "name",
            "must be nonempty, must not be '.' or '..', and must not contain control characters",
        ));
    }
    Ok(())
}

fn validate_nonnegative(parameter: &'static str, value: i64) -> Result<()> {
    if value < 0 {
        return Err(Error::invalid_input(parameter, "must not be negative"));
    }
    Ok(())
}

fn page_params(page: Option<i32>) -> Result<Vec<(&'static str, String)>> {
    match page {
        Some(page) if page < 1 => Err(Error::invalid_input("page", "must be at least 1")),
        Some(page) => Ok(vec![("page", page.to_string())]),
        None => Ok(Vec::new()),
    }
}

fn decode<T: DeserializeOwned>(body: String) -> Result<T> {
    let mut deserializer = serde_json::Deserializer::from_str(&body);
    let value = match serde_path_to_error::deserialize(&mut deserializer) {
        Ok(value) => value,
        Err(error) => {
            let field = Some(error.path().to_string());
            return Err(Error::Decode {
                message: error.into_inner().to_string(),
                body,
                field,
            });
        }
    };
    deserializer.end().map_err(|error| Error::Decode {
        message: error.to_string(),
        body,
        field: None,
    })?;
    Ok(value)
}

fn header_number<T: std::str::FromStr>(headers: &HeaderMap, name: &str) -> Option<T> {
    headers.get(name)?.to_str().ok()?.parse().ok()
}

fn rate_limit_info(headers: &HeaderMap) -> Option<RateLimitInfo> {
    Some(RateLimitInfo {
        limit: header_number(headers, "X-RateLimit-Limit")?,
        remaining: header_number(headers, "X-RateLimit-Remaining")?,
        reset: header_number(headers, "X-RateLimit-Reset")?,
    })
}

fn retry_after_seconds(header: &HeaderValue) -> Option<u64> {
    let value = header.to_str().ok()?.trim();
    value
        .parse()
        .ok()
        .or_else(|| httpdate::parse_http_date(value).ok().map(seconds_until))
}

fn seconds_until_reset(reset: u64) -> Option<u64> {
    UNIX_EPOCH
        .checked_add(Duration::from_secs(reset))
        .map(seconds_until)
}

fn seconds_until(deadline: SystemTime) -> u64 {
    let remaining = deadline
        .duration_since(SystemTime::now())
        .unwrap_or_default();
    // Round up so the next attempt does not arrive before the server's deadline.
    remaining
        .as_secs()
        .saturating_add(u64::from(remaining.subsec_nanos() != 0))
}
