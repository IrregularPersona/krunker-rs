use krunker_rs::Client;
use std::env;
use std::time::Duration;

fn get_kda(kill: i64, death: i64, assist: i64) -> f64 {
    (kill + assist) as f64 / death as f64
}

fn format_ms(ms: i64) -> String {
    let duration = Duration::from_millis(ms as u64);
    humantime::format_duration(duration).to_string()
}

fn print_match(pmatch: &krunker_rs::PlayerMatch) {
    println!("Match id: {}", pmatch.pm_match_id);
    println!("Date: {}", pmatch.pm_date);
    println!("Map: {}", pmatch.pm_map);
    // println!("Duration: {}", format_ms(pmatch.pm_duration as i64 * 1000));
    println!("Region: {}", pmatch.pm_region);
    println!("Kills: {}", pmatch.pm_kills);
    println!("Deaths: {}", pmatch.pm_deaths);
    println!(
        "KDA: {:.2}",
        get_kda(
            pmatch.pm_kills as i64,
            pmatch.pm_deaths as i64,
            pmatch.pm_assists as i64
        )
    );
    println!();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let pos_args: Vec<_> = args.iter().filter(|arg| !arg.starts_with("--")).collect();

    match pos_args.as_slice() {
        [_, api_key, target_player] => {
            let client = Client::new(api_key.to_string()).expect("Failed to create client");

            let mut all_matches = Vec::new();

            for page in 1..=5 {
                match client
                    .get_player_matches(&target_player, Some(page), None)
                    .await
                {
                    Ok(matches) => {
                        if matches.pmr_matches.is_empty() {
                            break;
                        }
                        all_matches.extend(matches.pmr_matches);
                    }
                    Err(err) => {
                        println!("Error on page {}: {}", page, err);
                        break;
                    }
                }
            }

            if all_matches.is_empty() {
                println!("No matches found for player: {}", target_player);
                return;
            }

            println!("Collected {} matches total.", all_matches.len());

            // best 5 recent games
            let mut best_games: Vec<_> = all_matches.iter().collect();
            best_games.sort_by(|a, b| b.pm_kills.cmp(&a.pm_kills));
            println!("\n=== TOP 10 BEST GAMES (HIGHEST KILLS) ===");
            for pmatch in best_games.iter().take(10) {
                print_match(pmatch);
            }

            // worst 5 recent games
            let mut worst_games: Vec<_> = all_matches.iter().collect();
            worst_games.sort_by(|a, b| a.pm_kills.cmp(&b.pm_kills));
            println!("\n=== TOP 5 WORST GAMES (LOWEST KILLS) ===");
            for pmatch in worst_games.iter().take(5) {
                print_match(pmatch);
            }
        }
        _ => {
            println!("Usage: cargo run --example fetch_ranked_history <api-key> <player-name>");
        }
    }
}
