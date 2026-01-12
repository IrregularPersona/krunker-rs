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
            println!("KDR: {}", player.player_kdr);
        }
        Err(e) => eprintln!("Error fetching player: {}", e),
    }

    Ok(())
}
```

## Available Methods

The `Client` struct provides the following methods:

- `get_player(name)`
- `get_player_inventory(name)`
- `get_player_matches(name, page, season)`
- `get_player_posts(name, page)`
- `get_match(id)`
- `get_clan(name)`
- `get_clan_members(name, page)`
- `get_leaderboard(region, page)`
- `get_map(name)`
- `get_map_leaderboard(name, page)`
- `get_mods(page)`
- `get_mod(name)`
- `get_market_skin(skin_index, page)`

## License

MIT
