# krunker-rs

An asynchronous Rust client for the Krunker.io Game API. Fetch player statistics,
inventories, ranked matches, clans, leaderboards, maps, mods, and market data
through 13 typed, read-only endpoint methods.

The library requires **Rust 1.85 or newer** and a **Tokio runtime**. You need a
developer API key issued for the Krunker Game API.

## Installation

```toml
[dependencies]
krunker-rs = { git = "https://github.com/IrregularPersona/krunker-rs" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick start

Set `KRUNKER_API_KEY` in your environment, then await an endpoint method:

```no_run
use krunker_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("KRUNKER_API_KEY")?)?;
    let player = client.get_player("Sidney").await?;

    println!("{} — level {}", player.player_name, player.player_level);
    println!("K/D: {}", player.player_kdr);

    if let Some(limit) = client.last_rate_limit().await {
        println!("{} of {} requests remaining", limit.remaining, limit.limit);
    }
    Ok(())
}
```

Pass names as ordinary strings, such as `"Arena #1"`. The client encodes each
name as one URL segment; do not percent-encode it yourself. Map names are
case-sensitive.

## Configuration

`Client::new` uses a 30-second timeout per attempt, a 10-second connection timeout,
and no automatic retries. Use the builder to change these defaults:

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

Only HTTP 429 responses are retried, with the server's recommended delay and a
bounded number of attempts. Waiting between retries is outside the request
timeout. Rate limit snapshots come from response headers; the client does not
reserve a quota or prevent concurrent tasks from exhausting it.

Clone a client to share its HTTP connection pool, debug setting, and rate limit
snapshot between tasks. Separate calls to `Client::new` create independent clients.

## Endpoint overview

Every method is asynchronous and returns `krunker_rs::Result<T>`. A `None` page
omits the query parameter and uses the API default; supplied pages start at 1.

| Method | Returns | Purpose |
| --- | --- | --- |
| `get_player(name)` | `Player` | Profile and lifetime statistics |
| `get_player_inventory(name)` | `Vec<InventoryItem>` | Owned skin counts |
| `get_player_matches(name, page, season)` | `PlayerMatchesResponse` | Ranked match history |
| `get_player_posts(name, page)` | `PostsResponse` | Social posts |
| `get_match(match_id)` | `Match` | Ranked match participants and statistics |
| `get_clan(name)` | `Clan` | Public clan profile |
| `get_clan_members(name, page)` | `ClanMembersResponse` | Clan membership |
| `get_leaderboard(region, page)` | `LeaderboardResponse` | Regional ranked standings |
| `get_map(name)` | `GameMap` | Map details |
| `get_map_leaderboard(name, page)` | `MapLeaderboardResponse` | Map standings |
| `get_mods(page)` | `ModsResponse` | Popular active mods |
| `get_mod(name)` | `Mod` | Mod details |
| `get_market_skin(skin_index, page)` | `MarketResponse` | Listings, owners, and price history |

Known ranked region IDs are **2 = Asia**, **3 = Europe**, and **4 = North America**.
Response arrays may be absent or null and are represented by `Option<Vec<T>>`.
Use `unwrap_or_default()` when your application treats those cases as empty.

## Errors

```no_run
use krunker_rs::{Client, Error};

# async fn example(client: &Client) {
match client.get_player("Sidney").await {
    Ok(player) => println!("{}", player.player_name),
    Err(Error::RateLimit { retry_after }) => {
        eprintln!("Try again in {retry_after} seconds");
    }
    Err(Error::Http(error)) if error.is_timeout() => eprintln!("Request timed out"),
    Err(error) => eprintln!("{error}"),
}
# }
```

Errors distinguish invalid local input, transport failures, API status errors,
rate limits, and response decoding failures. Decode errors retain the full body
and JSON field path for explicit inspection; normal display output omits that body.

## Examples

From a checkout, set the key once:

```bash
export KRUNKER_API_KEY="your-api-key"
cargo run --example fetch_player -- Sidney
cargo run --example fetch_post -- IshaqAyubi
cargo run --example last_5_games -- IshaqAyubi
cargo run --example fetch_ranked_history -- IshaqAyubi
cargo run --example global_leaderboard
cargo run --example ishaq_posts
```

In PowerShell, use `$env:KRUNKER_API_KEY = "your-api-key"` before the same commands.
All examples also accept an API key as their first positional argument for
compatibility. Add `--debug` to log request metadata to stderr. API failures and
invalid arguments result in a nonzero exit status.

## Documentation and development

Read the [usage guide and API reference][guide] for pagination,
configuration, response fields, timestamp units, and troubleshooting.
`cargo doc --no-deps --open` builds the Rust API reference locally. The README,
guide, and Rust examples are compiled by documentation tests without contacting
the API.

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo test --locked --doc
```

The default tests use local mock servers and fixtures. A separate live smoke test
is ignored unless requested explicitly; see the guide for its command. CI runs
formatting, Clippy, tests, and documentation checks on Linux and Windows, and
checks the declared minimum Rust version.

## License

[MIT](https://opensource.org/licenses/MIT). The full text is in the repository's `LICENSE` file.

[guide]: documentation.md
