mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, args) = common::client_and_args(
        1,
        "Usage: cargo run --example fetch_player -- [api-key] <player-name> [--debug]",
    )?;
    let player = client.get_player(&args[0]).await?;
    println!("Name: {}", player.player_name);
    println!("Level: {}", player.player_level);
    println!("K/D: {}", player.player_kdr);
    let seconds = player.player_time_played;
    println!(
        "Time played: {}d {}h {}m",
        seconds / 86_400,
        (seconds % 86_400) / 3_600,
        (seconds % 3_600) / 60
    );

    if let Some(limit) = client.last_rate_limit().await {
        println!(
            "Rate limit: {}/{} remaining (reset: {})",
            limit.remaining, limit.limit, limit.reset
        );
    }
    Ok(())
}
