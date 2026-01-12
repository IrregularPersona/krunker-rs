# krunker-rs

A Rust client library for the Krunker.io Game API.

## Features

- **Error Handling**: Nice error handling. lol.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
krunker-rs = { git = "https://github.com/IrregularPersona/krunker-rs" }
```

## Usage

### Basic Example

```rust
use krunker_rs::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "your_api_key_here";
    let client = Client::new(api_key)?;

    match client.get_player("Sidney") {
        Ok(player) => {
            println!("Player: {}", player.player_name);
            println!("Level: {}", player.player_level);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
```

### Handling Rate Limits

The client automatically tracks rate limit information from the latest response.

```rust
if let Some(rl) = client.last_rate_limit() {
    println!("Remaining: {}/{}", rl.remaining, rl.limit);
    println!("Resets at: {}", rl.reset);
}
```

If you exceed the rate limit, the client will return a `RateLimit` error variant:

```rust
use krunker_rs::Error;

match client.get_player("Sidney") {
    Err(Error::RateLimit { retry_after }) => {
        println!("Too many requests! Retry in {} seconds", retry_after);
    }
    // ... handle other errors
    _ => {}
}
```

## Available Methods

The `Client` struct provides the following methods:

- `get_player(name: &str)`
- `get_player_inventory(name: &str)`
- `get_player_matches(name: &str, page, season)`
- `get_player_posts(name: &str, page)`
- `get_match(id: i64)`
- `get_clan(name: &str)`
- `get_clan_members(name: &str, page)`
- `get_leaderboard(region, page)`
- `get_map(name: &str)` (Case-sensitive)
- `get_map_leaderboard(name: &str, page)` (Case-sensitive)
- `get_mods(page)`
- `get_mod(name: &str)`
- `get_market_skin(skin_index, page)`

## License

MIT
