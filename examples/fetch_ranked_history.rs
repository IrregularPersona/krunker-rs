mod common;

use krunker_rs::PlayerMatch;
use std::cmp::Reverse;

fn get_kda(kills: i64, deaths: i64, assists: i64) -> Option<f64> {
    (deaths > 0).then(|| (kills as f64 + assists as f64) / deaths as f64)
}

fn print_match(game: &PlayerMatch) {
    println!("Match ID: {}", game.pm_match_id);
    println!("Date: {}", game.pm_date);
    println!("Map ID: {}", game.pm_map);
    println!("Duration: {} seconds", game.pm_duration);
    println!("Region: {}", game.pm_region);
    println!("Kills: {}", game.pm_kills);
    println!("Deaths: {}", game.pm_deaths);
    match get_kda(game.pm_kills, game.pm_deaths, game.pm_assists) {
        Some(kda) => println!("KDA: {kda:.2}"),
        None => println!("KDA: N/A (no deaths)"),
    }
    println!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, args) = common::client_and_args(
        1,
        "Usage: cargo run --example fetch_ranked_history -- [api-key] <player-name> [--debug]",
    )?;
    let mut all_matches = Vec::new();
    for page in 1..=5 {
        let matches = client
            .get_player_matches(&args[0], Some(page), None)
            .await?
            .pmr_matches
            .unwrap_or_default();
        if matches.is_empty() {
            break;
        }
        all_matches.extend(matches);
    }
    if all_matches.is_empty() {
        println!("No matches found for {}.", args[0]);
        return Ok(());
    }
    println!("Collected {} matches.", all_matches.len());
    let mut games: Vec<_> = all_matches.iter().collect();
    games.sort_by_key(|game| Reverse(game.pm_kills));
    println!("\n=== TOP 10 GAMES BY KILLS ===");
    for game in games.iter().take(10) {
        print_match(game);
    }
    games.sort_by_key(|game| game.pm_kills);
    println!("\n=== BOTTOM 5 GAMES BY KILLS ===");
    for game in games.iter().take(5) {
        print_match(game);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::get_kda;

    #[test]
    fn zero_deaths_have_no_ratio() {
        assert_eq!(get_kda(10, 0, 2), None);
        assert_eq!(get_kda(0, 0, 0), None);
        assert_eq!(get_kda(10, 4, 2), Some(3.0));
    }

    #[test]
    fn large_counts_do_not_overflow() {
        assert!(get_kda(i64::MAX, 1, i64::MAX).unwrap().is_finite());
    }
}
