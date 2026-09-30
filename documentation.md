# Usage guide and API reference

This guide describes the Rust client and its mapping to the Krunker Game API.
Start with the [README][readme] for installation. Generated documentation for
all public response fields is available with `cargo doc --no-deps --open`.

## Contents

- [Authentication and runtime](#authentication-and-runtime)
- [Configuration](#configuration)
- [Sharing a client](#sharing-a-client)
- [Endpoint reference](#endpoint-reference)
- [Pagination](#pagination)
- [Rate limits and retries](#rate-limits-and-retries)
- [Errors and diagnostics](#errors-and-diagnostics)
- [Response fields and units](#response-fields-and-units)
- [Examples and troubleshooting](#examples-and-troubleshooting)
- [Testing and compatibility](#testing-and-compatibility)

## Authentication and runtime

All endpoint methods make authenticated, read-only GET requests. The default API
root is `https://gapi.svc.krunker.io/api`; the client supplies the
`X-Developer-API-Key` header automatically.

`Client::new` and `ClientBuilder::build` validate configuration without making a
network request. A syntactically valid key is checked by the server on the first
endpoint call; HTTP 403 means the server rejected access.

```no_run
use krunker_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("KRUNKER_API_KEY")?)?;
    let player = client.get_player("Sidney").await?;
    println!("{} has {} wins", player.player_name, player.player_wins);
    Ok(())
}
```

Use a Tokio runtime. The methods return futures: calling one without `.await`
does not execute a request. There is no synchronous client API.

## Configuration

| Builder method | Default | Behavior |
| --- | --- | --- |
| `timeout(Duration)` | 30 seconds | Timeout for each attempt, including reading its body |
| `connect_timeout(Duration)` | 10 seconds | Connection timeout for the default HTTP client |
| `retry_rate_limits(u32)` | 0 | Maximum additional attempts after HTTP 429 |
| `base_url(...)` | Production API root | Replaces the root and preserves its path prefix |
| `http_client(reqwest::Client)` | Created internally | Uses an existing HTTP client's connection, TLS, proxy, and redirect settings |
| `debug(bool)` | `false` | Enables request metadata on stderr |

```no_run
use krunker_rs::Client;
use std::time::Duration;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::builder(std::env::var("KRUNKER_API_KEY")?)
    .timeout(Duration::from_secs(20))
    .connect_timeout(Duration::from_secs(5))
    .retry_rate_limits(2)
    .build()?;
# Ok(())
# }
```

Timeouts must be greater than zero. The request timeout still applies when you
inject an HTTP client; the injected client controls its own connection timeout.
Add `reqwest = "0.12"` to your application's dependencies to configure one:

```no_run
use krunker_rs::Client;
use std::time::Duration;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let http = reqwest::Client::builder()
    .connect_timeout(Duration::from_secs(5))
    .build()?;
let client = Client::builder(std::env::var("KRUNKER_API_KEY")?)
    .http_client(http)
    .timeout(Duration::from_secs(20))
    .build()?;
# Ok(())
# }
```

The default HTTP client does not follow redirects. A 3xx response becomes
`Error::Api`. An injected HTTP client keeps its configured redirect policy.

For a mock server or a proxy, set the full root, including `/api` if needed:

```no_run
use krunker_rs::Client;

# fn main() -> krunker_rs::Result<()> {
let client = Client::builder("test-key")
    .base_url("http://127.0.0.1:8080/api/")
    .build()?;
# Ok(())
# }
```

HTTP and HTTPS roots are accepted with or without a trailing slash. Roots cannot
contain credentials, a query string, or a fragment.

## Sharing a client

Cloning a client is inexpensive. Its clones share one HTTP connection pool,
debug flag, and rate limit snapshot. A new independently constructed client gets
its own pool and state. There is no process-wide singleton.

```no_run
use krunker_rs::Client;

# async fn example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
let worker = client.clone();
let task = tokio::spawn(async move { worker.get_player("Sidney").await });
let posts = client.get_player_posts("IshaqAyubi", Some(1)).await?;
let player = task.await??;
println!("{}; {} posts", player.player_name, posts.posts_posts.unwrap_or_default().len());
# Ok(())
# }
```

Concurrent calls can consume the same API key's quota. The cached rate limit is
the last response processed, not a reservation or a guaranteed ordering of
server-side requests.

## Endpoint reference

Every method returns `krunker_rs::Result<T>` and must be awaited. Paths below are
relative to the configured root, which already includes `/api` by default.

| Rust method | GET path | Response type | Parameters and behavior |
| --- | --- | --- | --- |
| `get_player(name: &str)` | `/player/{name}` | `Player` | Profile, lifetime statistics, and current ranked profiles |
| `get_player_inventory(name: &str)` | `/player/{name}/inventory` | `Vec<InventoryItem>` | Owned skin quantities; excludes market listings |
| `get_player_matches(name: &str, page: Option<i32>, season: Option<i32>)` | `/player/{name}/matches` | `PlayerMatchesResponse` | Ranked matches, newest first; no season filter when `season` is `None` |
| `get_player_posts(name: &str, page: Option<i32>)` | `/player/{name}/posts` | `PostsResponse` | A page of social posts |
| `get_match(match_id: i64)` | `/match/{match_id}` | `Match` | Ranked match details; participants ordered by team, then score |
| `get_clan(name: &str)` | `/clan/{name}` | `Clan` | Public clan profile |
| `get_clan_members(name: &str, page: Option<i32>)` | `/clan/{name}/members` | `ClanMembersResponse` | Members ordered by role, then score |
| `get_leaderboard(region: i32, page: Option<i32>)` | `/leaderboard/{region}` | `LeaderboardResponse` | Current ranked season; entries ordered by MMR, highest first |
| `get_map(name: &str)` | `/map/{name}` | `GameMap` | Case-sensitive map name; restricted or deleted maps can return 404 |
| `get_map_leaderboard(name: &str, page: Option<i32>)` | `/map/{name}/leaderboard` | `MapLeaderboardResponse` | Case-sensitive; empty entries when no leaderboard is configured |
| `get_mods(page: Option<i32>)` | `/mods` | `ModsResponse` | Active mods sorted by votes; deleted mods and banned creators excluded |
| `get_mod(name: &str)` | `/mods/{name}` | `Mod` | One mod's details |
| `get_market_skin(skin_index: i32, page: Option<i32>)` | `/market/skin/{skin_index}` | `MarketResponse` | Listing pagination, top owners, and daily price history |

### Names and input validation

Pass names exactly as they appear in the game, without URL encoding. Spaces,
Unicode, `#`, `?`, `%`, and `/` are encoded as part of a single path segment.
For example, `"Arena #1"` is sent as `Arena%20%231`; passing `"Arena%20%231"`
instead looks up a name containing those literal percent sequences.

The client rejects these inputs before sending a request:

- Empty or whitespace-only names, literal `.` or `..`, and control characters.
- Page numbers below 1.
- Negative season, region, or skin index values.
- Match IDs of zero or less.
- Empty API keys or keys that cannot be represented as an HTTP header.

Other names and IDs are validated by the API and can produce `Error::Api`.

### Ranked region IDs

| Region | Request ID |
| --- | --- |
| Asia | 2 |
| Europe | 3 |
| North America | 4 |

Other nonnegative IDs are forwarded to the API for compatibility with new
regions. Treat region fields in returned player and match data as API values;
the client does not translate or normalize them.

## Pagination

`None` omits the `page` query parameter; the API currently defaults to page 1.
Pass `Some(2)` for page 2. Page sizes are controlled by the server, not the client.
Use a returned `per_page` field when available. Typical page sizes are 10 for
ranked history, clan members, ranked leaderboards, mods, and market listings;
map leaderboards typically use 25.

Collections use `Option<Vec<T>>`. An absent or JSON-null collection becomes
`None`; an empty array becomes `Some(vec![])`. Use `unwrap_or_default()` when
both should behave like an empty list.

This example fetches at most ten pages of ranked history and stops at an empty
page:

```no_run
use krunker_rs::{Client, PlayerMatch};

# async fn example(client: &Client) -> krunker_rs::Result<Vec<PlayerMatch>> {
let mut games = Vec::new();
for page in 1..=10 {
    let response = client.get_player_matches("IshaqAyubi", Some(page), None).await?;
    let batch = response.pmr_matches.unwrap_or_default();
    if batch.is_empty() {
        break;
    }
    games.extend(batch);
}
# Ok(games)
# }
```

Market `page` applies only to `mr_listings`. The API also returns up to 100 owners
and price history independently of the listing page. There is no automatic
pagination or response cache in the client.

## Rate limits and retries

API limits apply per key and can vary. Use the actual response headers instead
of hardcoding a request count or window length:

| Header | Rust field | Meaning |
| --- | --- | --- |
| `X-RateLimit-Limit` | `RateLimitInfo::limit` | Maximum requests in the current window |
| `X-RateLimit-Remaining` | `RateLimitInfo::remaining` | Requests remaining when the response was generated |
| `X-RateLimit-Reset` | `RateLimitInfo::reset` | Window reset time as Unix seconds |

```no_run
use krunker_rs::Client;

# async fn example(client: &Client) {
if let Some(limit) = client.last_rate_limit().await {
    println!("Remaining: {}/{}; reset: {}", limit.remaining, limit.limit, limit.reset);
}
# }
```

The snapshot is `None` before a response or if the most recently processed
response lacks a complete, valid set of headers. It is shared by clones.

Every HTTP 429 is returned as `Error::RateLimit`, including non-JSON responses
from a proxy. The recommended delay is selected in this order:

1. Nonnegative JSON `retry_after`, in seconds.
2. The `Retry-After` header, as seconds or an HTTP date.
3. Time until `X-RateLimit-Reset`, rounded up to a whole second.
4. One second when no usable delay is supplied.

Example server response:

```json
{"error":"Rate limit exceeded","retry_after":45}
```

`retry_rate_limits(2)` allows at most three attempts: the original request and
two retries. Retries wait for the recommended delay and apply only to 429s.
Other HTTP statuses, transport errors, and decode failures are returned immediately.
The client does not proactively throttle concurrent requests.

The per-attempt timeout excludes the delay between retries. Apply an outer
timeout when your application needs a total budget:

```no_run
use krunker_rs::Client;
use std::time::Duration;

# async fn example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
let player = tokio::time::timeout(
    Duration::from_secs(60),
    client.get_player("Sidney"),
).await??;
println!("{}", player.player_name);
# Ok(())
# }
```

## Errors and diagnostics

| Error variant | Meaning | Useful next step |
| --- | --- | --- |
| `InvalidInput { parameter, message }` | Local configuration or parameter validation failed | Correct the input; no request was sent |
| `Http(reqwest::Error)` | Transport, TLS, timeout, or response body read failure | Inspect `is_timeout()` / `is_connect()` and the error source |
| `Api { status, message }` | Non-success status other than 429 | Inspect the HTTP status and API message |
| `RateLimit { retry_after }` | HTTP 429 after any configured retries | Wait or reduce request volume |
| `Decode { message, body, field }` | A 2xx body did not match the response type or was not complete JSON | Inspect the JSON field path and explicitly inspect the body |

Typical API statuses are 400 for invalid server-side parameters, 403 for rejected
access, 404 for missing resources, 429 for rate limits, and 500 for server errors.
The library also retains other non-success statuses, including 3xx responses
when redirects are disabled.

```no_run
use krunker_rs::{Client, Error};

# async fn example(client: &Client) {
match client.get_player("Sidney").await {
    Ok(player) => println!("{}", player.player_name),
    Err(Error::Api { status, message }) => eprintln!("API status {status}: {message}"),
    Err(Error::Decode { message, field, body }) => {
        eprintln!("Cannot decode {field:?}: {message}; received {} bytes", body.len());
    }
    Err(Error::InvalidInput { parameter, message }) => eprintln!("{parameter}: {message}"),
    Err(Error::RateLimit { retry_after }) => eprintln!("Retry in {retry_after}s"),
    Err(Error::Http(error)) => eprintln!("Transport error: {error}"),
}
# }
```

Unknown JSON object fields are accepted for API compatibility. Missing required
fields, wrong types, malformed JSON, and non-whitespace trailing data produce a
decode error. `field` contains a JSON path, such as `matches[0].kills`; trailing
data has no field path.

`Error::source()` exposes a wrapped Reqwest error. Body transport failures remain
`Error::Http` even when the server returned an error status. API errors keep the
full message, but display at most 512 characters. Decode error display omits the
response body; that body remains available in the variant.

`client.set_debug(true)` changes debugging for that client and its clones. Logs
go to stderr and include the URL, status, body size, and retry delay. They do not
print the API key or the response body.

## Response fields and units

All public models derive `Debug`, `Clone`, `Serialize`, and `Deserialize`. Existing
Rust field prefixes are retained. Serde maps them to the API's original JSON
keys, so serialized JSON uses `kills`, not a Rust field such as `pm_kills`.

| Model | Rust field prefix | Contains |
| --- | --- | --- |
| `Player` | `player_` | Identity, clan, badges, ranked profiles, currency, level, and lifetime counters |
| `RankedProfile` | `ranked_` | Region, MMR, wins, losses, combat totals, score, and time played |
| `InventoryItem` | `inventory_` | Skin index and owned quantity |
| `PlayerMatchesResponse` | `pmr_` | Page, page size, and optional `Vec<PlayerMatch>` |
| `PlayerMatch` | `pm_` | Match ID, date, map, region, season, combat statistics, and result |
| `PostsResponse` | `posts_` | Page, page size, and optional `Vec<Post>` |
| `Post` | `post_` | Date, text, votes, and comment count |
| `Match` | `match_` | Match ID, date, map, duration, season, region, and participants |
| `MatchParticipant` | `mp_` | Player name and statistics within one ranked match |
| `Clan` | `clan_` | Name, owner, score, rank, size, creation time, and Discord invite code |
| `ClanMembersResponse` | `cmr_` | Page, page size, and optional `Vec<ClanMember>` |
| `ClanMember` | `cm_` | Player name and numeric role |
| `LeaderboardResponse` | `lr_` | Page, page size, current season, region, and entries |
| `LeaderboardEntry` | `le_` | Regional position, player, MMR, and ranked statistics |
| `GameMap` | `gm_` | Map identity, creator, activity, category, timestamps, and leaderboard settings |
| `MapLeaderboardResponse` | `mlr_` | Page, page size, map name, leaderboard settings, and entries |
| `MapLeaderboardEntry` | `mle_` | Position, player, score or time value, and recorded date |
| `ModsResponse` | `mods_` | Page, page size, and optional `Vec<Mod>` |
| `Mod` | `mod_` | Identity, creator, votes, featured flag, version, and timestamps |
| `MarketResponse` | `mr_` | Skin, listing count, pricing, circulation, listings, owners, and history |
| `MarketListing` | `ml_` | Integer KR price, seller, and listing timestamp |
| `MarketOwner` | `mo_` | Player name and owned quantity |
| `PriceHistory` | `ph_` | Day, average sale price, and sales count |
| `RateLimitInfo` | None | Limit, remaining requests, and reset timestamp |
| `RateLimitResponse` | None | JSON error text and retry delay |
| `GenericErrorResponse` | None | JSON error text |

Each field's Rust type, JSON key, and meaning is documented in the generated
API reference. Representative JSON payloads are kept in
[`tests/fixtures`](https://github.com/IrregularPersona/krunker-rs/tree/HEAD/tests/fixtures)
and checked against these public models.

### Time, counters, and special values

| Fields | Representation and unit |
| --- | --- |
| `player_time_played`, `ranked_time_played` | Integer seconds |
| `pm_duration`, `pm_play_time`, `match_duration` | Integer seconds |
| `gm_playtime` | Integer milliseconds |
| Creation, update, listing, post, and match dates | API timestamp strings, normally RFC3339 |
| `ph_date` | Calendar date string in `YYYY-MM-DD` form |
| `RateLimitInfo::reset` | Unix timestamp in seconds |
| `Error::RateLimit::retry_after` | Delay in seconds |
| KR balances and listing prices | Integer KR amounts |
| Market and historical average prices | Floating-point KR amounts |
| `pm_accuracy`, `mp_accuracy` | Integer percentage values |
| `pm_victory` | Numeric result; 1 indicates victory |
| `gm_leaderboard_order`, `mlr_leaderboard_order` | 0 = ascending, 1 = descending |
| `gm_leaderboard_type`, `mlr_leaderboard_type` | API value such as `"time"` or `"score"`; a map can have an empty type |

Timestamps remain strings so callers can choose their date library. Do not treat
map playtime as seconds or infer a map leaderboard value's unit without checking
its type. IDs, roles, flags, and categories remain numeric API values.

A player's clan name can be empty when they have no clan. A clan's Discord code
can be empty when unset. Ranked profiles include regions where the player has
completed the required placement matches. The API controls placement thresholds.
Player history and match details cover ranked games, not all public matches.

Market listings are ordered by price, lowest first. `mr_lowest_price` is zero
when there are no listings. `mr_average_price` reflects recent sales, typically
the last seven days. Owners are ordered by quantity, highest first, and history
contains daily averages for days with sales, typically within the last 30 days.

## Examples and troubleshooting

All example commands accept an environment key or a first positional API key.
A positional key takes precedence when supplied. These commands assume
`KRUNKER_API_KEY` is already set:

| Command | Output |
| --- | --- |
| `cargo run --example fetch_player -- Sidney` | Profile and rate limit snapshot |
| `cargo run --example fetch_post -- IshaqAyubi` | First page of posts |
| `cargo run --example last_5_games -- IshaqAyubi` | Five recent ranked matches and details for the newest one |
| `cargo run --example fetch_ranked_history -- IshaqAyubi` | Up to five history pages, then games ranked by kills |
| `cargo run --example global_leaderboard` | Ten unique players with their highest regional MMR |
| `cargo run --example ishaq_posts` | First page of IshaqAyubi's posts |

Append `--debug` for request metadata. Names containing spaces should be quoted.
Examples return a nonzero exit code on API errors or invalid command-line input.
KDA is displayed as `N/A` when there are no deaths.

| Symptom | Check |
| --- | --- |
| Compiler expects a future instead of a result | Make the caller async and add `.await` |
| No Tokio runtime available | Use `#[tokio::main]` or run the future inside your existing Tokio runtime |
| HTTP 403 | Check the supplied developer key and its access |
| HTTP 404 | Check spelling, map case, and whether the resource is public or still exists |
| HTTP 429 | Respect the reported delay, reduce concurrency, or enable bounded rate limit retries |
| Transport timeout | Retry deliberately or increase the configured timeout for a slow endpoint |
| Decode error | Inspect the field path and body; the server may have changed its schema |
| Empty history or leaderboard | The player may lack ranked games or placements in that region |

## Testing and compatibility

Local tests require no developer key. They use synthetic JSON fixtures and mock
HTTP servers to check routes, encoding, query parameters, errors, timeouts,
retries, and rate limit state. Documentation snippets compile without making
network requests.

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo test --locked --doc
cargo doc --locked --no-deps
```

To check the live service explicitly with `KRUNKER_API_KEY` set:

```bash
cargo test --locked --test live_api -- --ignored --nocapture
```

The live test uses `KRUNKER_PLAYER` (default `IshaqAyubi`) and `KRUNKER_MAP`
(default `Burg`). Mod details, match details, and clan calls depend on returned
data being available. The market skin is selected from the player's inventory,
with a fallback index of 3973. Live tests are ignored by default and in CI.

Existing endpoint signatures and response field names are preserved. Changes
to account for when updating from the earlier implementation:

- Requests now have a finite default timeout.
- Names must be passed without percent encoding.
- Invalid local parameters return the new `Error::InvalidInput` variant; add
  that case to any exhaustive matches on `Error`.
- Malformed success responses with trailing data are rejected.
- Debugging logs metadata rather than raw response bodies.
- Missing rate limit headers clear the previous snapshot.
- The default HTTP client returns redirects as status errors.

The repository declares Rust 1.85 as its minimum version and is licensed under
[MIT](https://opensource.org/licenses/MIT); see `LICENSE` for the full text.

[readme]: README.md
