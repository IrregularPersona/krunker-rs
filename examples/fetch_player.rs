use krunker_rs::Client;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, api_key, target_player] => {
            let client = Client::new(api_key).expect("Failed to create client");
            println!("Fetch profile: {}", target_player);

            match client.get_player(target_player) {
                Ok(player) => {
                    println!("Name: {:?}", player.player_name);
                    println!("Level: {}", player.player_level);
                    println!("K/D: {}", player.player_kdr);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        _ => {
            println!("usage: cargo run --example fetch_player -- <api-key> <player-name>");
        }
    }
}
