use krunker_rs::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let pos_args: Vec<_> = args.iter().filter(|arg| !arg.starts_with("--")).collect();

    match pos_args.as_slice() {
        [_, api_key] => {
            let client = Client::new(api_key.to_string()).expect("Failed to create client");

            // Regions according to documentation:
            // 2: Asia
            // 3: Europe
            // 4: North America
            let regions = [(2, "Asia"), (3, "Europe"), (4, "North America")];
            let mut all_entries = Vec::new();

            for (id, name) in regions {
                match client.get_leaderboard(id, Some(1)).await {
                    Ok(response) => {
                        all_entries.extend(response.lr_entries);
                    }
                    Err(err) => {
                        println!("Failed to fetch {} leaderboard: {}", name, err);
                    }
                }
            }

            if all_entries.is_empty() {
                println!("No leaderboard entries found.");
                return;
            }

            // dedup for multiple entries of same player
            all_entries.sort_by(|a, b| b.le_mmr.cmp(&a.le_mmr));
            all_entries.dedup_by(|a, b| a.le_player_name == b.le_player_name);

            println!("=== Top 10 Players Overall (By MMR) ===");
            for (i, entry) in all_entries.iter().take(10).enumerate() {
                println!(
                    "{:2}. {:<20} | MMR: {:<5} | Win/Loss: {}/{}",
                    i + 1,
                    entry.le_player_name,
                    entry.le_mmr,
                    entry.le_wins,
                    entry.le_losses
                );
            }
        }
        _ => {
            println!("Usage: cargo run --example global_leaderboard <api-key>");
        }
    }
}
