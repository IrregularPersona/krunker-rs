use krunker_rs::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let pos_args: Vec<_> = args.iter().filter(|arg| !arg.starts_with("--")).collect();

    match pos_args.as_slice() {
        [_, api_key, target_player] => {
            let client = Client::new(api_key.to_string()).expect("Failed to create client");

            match client
                .get_player_matches(target_player, Some(1), None)
                .await
            {
                Ok(response) => {
                    println!("=== Last 5 Games for {} ===", target_player);
                    if response.pmr_matches.is_empty() {
                        println!("No matches found.");
                        return;
                    }

                    for (i, pmatch) in response.pmr_matches.iter().take(5).enumerate() {
                        println!("{}. Match ID: {}", i + 1, pmatch.pm_match_id);
                        println!("   Date: {}", pmatch.pm_date);
                        println!("   Score: {}", pmatch.pm_score);
                        println!(
                            "   K/D/A: {}/{}/{}",
                            pmatch.pm_kills, pmatch.pm_deaths, pmatch.pm_assists
                        );
                        println!(
                            "   Result: {}",
                            if pmatch.pm_victory == 1 {
                                "Victory"
                            } else {
                                "Defeat"
                            }
                        );
                        println!();
                    }

                    // Dump full data for the most recent game
                    if let Some(first_match) = response.pmr_matches.first() {
                        println!("--- Full Data for Match {} ---", first_match.pm_match_id);
                        match client.get_match(first_match.pm_match_id).await {
                            Ok(match_details) => {
                                println!("{:#?}", match_details);
                            }
                            Err(err) => {
                                println!("Error fetching match details: {}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Error fetching matches: {}", err);
                }
            }
        }
        _ => {
            println!("Usage: cargo run --example last_5_games <api-key> <player-name>");
        }
    }
}
