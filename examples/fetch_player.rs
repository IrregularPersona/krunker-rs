use krunker_rs::Client;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let debug = args.iter().any(|arg| arg == "--debug");
    let positional_args: Vec<_> = args.iter().filter(|arg| !arg.starts_with("--")).collect();

    match positional_args.as_slice() {
        [_, api_key, target_player] => {
            let mut client = Client::new(api_key.to_string()).expect("Failed to create client");
            if debug {
                client.set_debug(true);
            }
            println!("Fetch profile: {}", target_player);

            match client.get_player(target_player) {
                Ok(player) => {
                    println!("Name: {:?}", player.player_name);
                    println!("Level: {}", player.player_level);
                    println!("K/D: {}", player.player_kdr);
                    let total_seconds = player.player_time_played / 1000;
                    let days = total_seconds / 86400;
                    let hours = (total_seconds % 86400) / 3600;
                    let minutes = (total_seconds % 3600) / 60;

                    println!("Time Played: {}d {}h {}m", days, hours, minutes);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        _ => {
            println!("usage: cargo run --example fetch_player -- <api-key> <player-name> [--debug]");
        }
    }
}
