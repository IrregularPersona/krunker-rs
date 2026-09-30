mod common;

use krunker_rs::LeaderboardEntry;
use std::{cmp::Reverse, collections::HashSet};

fn deduplicate_entries(entries: &mut Vec<LeaderboardEntry>) {
    entries.sort_by_key(|entry| Reverse(entry.le_mmr));
    let mut seen = HashSet::new();
    entries.retain(|entry| seen.insert(entry.le_player_name.clone()));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = common::client_and_args(
        0,
        "Usage: cargo run --example global_leaderboard -- [api-key] [--debug]",
    )?;
    let mut entries = Vec::new();
    for region in [2, 3, 4] {
        entries.extend(
            client
                .get_leaderboard(region, Some(1))
                .await?
                .lr_entries
                .unwrap_or_default(),
        );
    }
    if entries.is_empty() {
        println!("No leaderboard entries found.");
        return Ok(());
    }
    deduplicate_entries(&mut entries);
    println!("=== Top 10 Players Across Asia, Europe, and North America ===");
    for (index, entry) in entries.iter().take(10).enumerate() {
        println!(
            "{:2}. {:<20} | MMR: {:<5} | Win/Loss: {}/{}",
            index + 1,
            entry.le_player_name,
            entry.le_mmr,
            entry.le_wins,
            entry.le_losses
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::deduplicate_entries;
    use krunker_rs::LeaderboardResponse;

    #[test]
    fn keeps_highest_mmr_when_duplicate_names_are_separated() {
        let fixture: LeaderboardResponse =
            serde_json::from_str(include_str!("../tests/fixtures/leaderboard.json")).unwrap();
        let base = fixture.lr_entries.unwrap().remove(0);
        let mut entries = Vec::new();
        for (name, mmr) in [("A", 2300), ("B", 2400), ("A", 2500), ("B", 2200)] {
            let mut entry = base.clone();
            entry.le_player_name = name.to_owned();
            entry.le_mmr = mmr;
            entries.push(entry);
        }
        deduplicate_entries(&mut entries);
        assert_eq!(entries.len(), 2);
        assert_eq!(
            (&*entries[0].le_player_name, entries[0].le_mmr),
            ("A", 2500)
        );
        assert_eq!(
            (&*entries[1].le_player_name, entries[1].le_mmr),
            ("B", 2400)
        );
    }
}
