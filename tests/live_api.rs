use krunker_rs::Client;

/// Run manually with KRUNKER_API_KEY set. CI uses mock servers and never calls this API.
#[tokio::test]
#[ignore = "requires KRUNKER_API_KEY and network access"]
async fn live_endpoint_smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("KRUNKER_API_KEY")?)?;
    let name = std::env::var("KRUNKER_PLAYER").unwrap_or_else(|_| "IshaqAyubi".to_owned());
    let map = std::env::var("KRUNKER_MAP").unwrap_or_else(|_| "Burg".to_owned());
    let player = client.get_player(&name).await?;
    let inventory = client.get_player_inventory(&name).await?;
    client.get_player_posts(&name, Some(1)).await?;
    let history = client.get_player_matches(&name, Some(1), None).await?;
    client.get_leaderboard(3, Some(1)).await?;
    client.get_map(&map).await?;
    client.get_map_leaderboard(&map, Some(1)).await?;
    let mods = client.get_mods(Some(1)).await?;
    let skin_index = inventory
        .first()
        .map_or(3973, |item| item.inventory_skin_index);
    client.get_market_skin(skin_index, Some(1)).await?;

    if let Some(first) = mods.mods_mods.as_ref().and_then(|mods| mods.first()) {
        client.get_mod(&first.mod_name).await?;
    }
    if let Some(first) = history
        .pmr_matches
        .as_ref()
        .and_then(|matches| matches.first())
    {
        client.get_match(first.pm_match_id).await?;
    }
    if !player.player_clan.is_empty() {
        client.get_clan(&player.player_clan).await?;
        client
            .get_clan_members(&player.player_clan, Some(1))
            .await?;
    }
    let rate_limit = client.last_rate_limit().await;
    println!("Live requests succeeded; rate limit headers: {rate_limit:?}");
    Ok(())
}
