mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, args) = common::client_and_args(
        1,
        "Usage: cargo run --example last_5_games -- [api-key] <player-name> [--debug]",
    )?;
    let matches = client
        .get_player_matches(&args[0], Some(1), None)
        .await?
        .pmr_matches
        .unwrap_or_default();
    if matches.is_empty() {
        println!("No matches found for {}.", args[0]);
        return Ok(());
    }
    println!("=== Last 5 Games for {} ===", args[0]);
    for (index, game) in matches.iter().take(5).enumerate() {
        println!("{}. Match ID: {}", index + 1, game.pm_match_id);
        println!("   Date: {}", game.pm_date);
        println!("   Score: {}", game.pm_score);
        println!(
            "   K/D/A: {}/{}/{}",
            game.pm_kills, game.pm_deaths, game.pm_assists
        );
        println!(
            "   Result: {}\n",
            if game.pm_victory == 1 {
                "Victory"
            } else {
                "Defeat"
            }
        );
    }
    if let Some(first) = matches.first() {
        println!("--- Full Data for Match {} ---", first.pm_match_id);
        println!("{:#?}", client.get_match(first.pm_match_id).await?);
    }
    Ok(())
}
