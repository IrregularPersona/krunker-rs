use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedProfile {
    #[serde(rename = "region")]
    pub ranked_region: i32,
    #[serde(rename = "mmr")]
    pub ranked_mmr: i32,
    #[serde(rename = "wins")]
    pub ranked_wins: i32,
    #[serde(rename = "losses")]
    pub ranked_losses: i32,
    #[serde(rename = "kills")]
    pub ranked_kills: i32,
    #[serde(rename = "deaths")]
    pub ranked_deaths: i32,
    #[serde(rename = "assists")]
    pub ranked_assists: i32,
    #[serde(rename = "score")]
    pub ranked_score: i32,
    #[serde(rename = "damage_done")]
    pub ranked_damage_done: i32,
    #[serde(rename = "time_played")]
    pub ranked_time_played: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub player_name: String,
    #[serde(rename = "clan")]
    pub player_clan: String,
    #[serde(rename = "verified")]
    pub player_verified: bool,
    #[serde(rename = "flag")]
    pub player_flag: i32,
    #[serde(rename = "badges")]
    pub player_badges: Vec<i32>,
    #[serde(rename = "following")]
    pub player_following: i32,
    #[serde(rename = "followers")]
    pub player_followers: i32,
    #[serde(rename = "ranked")]
    pub player_ranked: Vec<RankedProfile>,
    #[serde(rename = "kr")]
    pub player_kr: i32,
    #[serde(rename = "level")]
    pub player_level: i32,
    #[serde(rename = "junk")]
    pub player_junk: f64,
    #[serde(rename = "inventory")]
    pub player_inventory: i32,
    #[serde(rename = "score")]
    pub player_score: i32,
    #[serde(rename = "spk")]
    pub player_spk: f64,
    #[serde(rename = "kills")]
    pub player_kills: i32,
    #[serde(rename = "deaths")]
    pub player_deaths: i32,
    #[serde(rename = "kdr")]
    pub player_kdr: f64,
    #[serde(rename = "kpg")]
    pub player_kpg: f64,
    #[serde(rename = "games")]
    pub player_games: i32,
    #[serde(rename = "wins")]
    pub player_wins: i32,
    #[serde(rename = "losses")]
    pub player_losses: i32,
    #[serde(rename = "assists")]
    pub player_assists: i32,
    #[serde(rename = "melees")]
    pub player_melees: i32,
    #[serde(rename = "beatdowns")]
    pub player_beatdowns: i32,
    #[serde(rename = "bullseyes")]
    pub player_bullseyes: i32,
    #[serde(rename = "headshots")]
    pub player_headshots: i32,
    #[serde(rename = "legshots")]
    pub player_legshots: i32,
    #[serde(rename = "wallbangs")]
    pub player_wallbangs: i32,
    #[serde(rename = "shots")]
    pub player_shots: i32,
    #[serde(rename = "hits")]
    pub player_hits: i32,
    #[serde(rename = "misses")]
    pub player_misses: i32,
    #[serde(rename = "time_played")]
    pub player_time_played: i32,
    #[serde(rename = "nukes")]
    pub player_nukes: i32,
    #[serde(rename = "airdrops")]
    pub player_airdrops: i32,
    #[serde(rename = "airdrops_stolen")]
    pub player_airdrops_stolen: i32,
    #[serde(rename = "slimes")]
    pub player_slimes: i32,
    #[serde(rename = "juggernauts")]
    pub player_juggernauts: i32,
    #[serde(rename = "juggernauts_killed")]
    pub player_juggernauts_killed: i32,
    #[serde(rename = "warmachines")]
    pub player_warmachines: i32,
    #[serde(rename = "hacker_tagged")]
    pub player_hacker_tagged: bool,
    #[serde(rename = "created_at")]
    pub player_created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    #[serde(rename = "skin_index")]
    pub inventory_skin_index: i32,
    #[serde(rename = "count")]
    pub inventory_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatch {
    #[serde(rename = "match_id")]
    pub pm_match_id: i32,
    #[serde(rename = "date")]
    pub pm_date: String,
    #[serde(rename = "map")]
    pub pm_map: i32,
    #[serde(rename = "duration")]
    pub pm_duration: i32,
    #[serde(rename = "season")]
    pub pm_season: i32,
    #[serde(rename = "region")]
    pub pm_region: i32,
    #[serde(rename = "kills")]
    pub pm_kills: i32,
    #[serde(rename = "deaths")]
    pub pm_deaths: i32,
    #[serde(rename = "assists")]
    pub pm_assists: i32,
    #[serde(rename = "score")]
    pub pm_score: i32,
    #[serde(rename = "damage_done")]
    pub pm_damage_done: i32,
    #[serde(rename = "headshots")]
    pub pm_headshots: i32,
    #[serde(rename = "accuracy")]
    pub pm_accuracy: i32,
    #[serde(rename = "objective_score")]
    pub pm_objective_score: i32,
    #[serde(rename = "kr")]
    pub pm_kr: i32,
    #[serde(rename = "victory")]
    pub pm_victory: i32,
    #[serde(rename = "rounds_won")]
    pub pm_rounds_won: i32,
    #[serde(rename = "team")]
    pub pm_team: i32,
    #[serde(rename = "play_time")]
    pub pm_play_time: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchesResponse {
    #[serde(rename = "page")]
    pub pmr_page: i32,
    #[serde(rename = "per_page")]
    pub pmr_per_page: i32,
    #[serde(rename = "matches")]
    pub pmr_matches: Vec<PlayerMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "date")]
    pub post_date: String,
    #[serde(rename = "text")]
    pub post_text: String,
    #[serde(rename = "votes")]
    pub post_votes: i32,
    #[serde(rename = "comment_count")]
    pub post_comment_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostsResponse {
    #[serde(rename = "page")]
    pub posts_page: i32,
    #[serde(rename = "per_page")]
    pub posts_per_page: i32,
    #[serde(rename = "posts")]
    pub posts_posts: Vec<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchParticipant {
    #[serde(rename = "player_name")]
    pub mp_player_name: String,
    #[serde(rename = "kills")]
    pub mp_kills: i32,
    #[serde(rename = "deaths")]
    pub mp_deaths: i32,
    #[serde(rename = "assists")]
    pub mp_assists: i32,
    #[serde(rename = "score")]
    pub mp_score: i32,
    #[serde(rename = "damage_done")]
    pub mp_damage_done: i32,
    #[serde(rename = "headshots")]
    pub mp_headshots: i32,
    #[serde(rename = "accuracy")]
    pub mp_accuracy: i32,
    #[serde(rename = "objective_score")]
    pub mp_objective_score: i32,
    #[serde(rename = "victory")]
    pub mp_victory: i32,
    #[serde(rename = "rounds_won")]
    pub mp_rounds_won: i32,
    #[serde(rename = "team")]
    pub mp_team: i32,
    #[serde(rename = "play_time")]
    pub mp_play_time: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    #[serde(rename = "match_id")]
    pub match_id: i32,
    #[serde(rename = "date")]
    pub match_date: String,
    #[serde(rename = "map")]
    pub match_map: i32,
    #[serde(rename = "duration")]
    pub match_duration: i32,
    #[serde(rename = "season")]
    pub match_season: i32,
    #[serde(rename = "region")]
    pub match_region: i32,
    #[serde(rename = "participants")]
    pub match_participants: Vec<MatchParticipant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clan {
    #[serde(rename = "name")]
    pub clan_name: String,
    #[serde(rename = "owner_name")]
    pub clan_owner_name: String,
    #[serde(rename = "score")]
    pub clan_score: i32,
    #[serde(rename = "rank")]
    pub clan_rank: i32,
    #[serde(rename = "member_count")]
    pub clan_member_count: i32,
    #[serde(rename = "created_at")]
    pub clan_created_at: String,
    #[serde(rename = "discord")]
    pub clan_discord: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanMember {
    #[serde(rename = "player_name")]
    pub cm_player_name: String,
    #[serde(rename = "role")]
    pub cm_role: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanMembersResponse {
    #[serde(rename = "page")]
    pub cmr_page: i32,
    #[serde(rename = "per_page")]
    pub cmr_per_page: i32,
    #[serde(rename = "members")]
    pub cmr_members: Vec<ClanMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    #[serde(rename = "position")]
    pub le_position: i32,
    #[serde(rename = "player_name")]
    pub le_player_name: String,
    #[serde(rename = "mmr")]
    pub le_mmr: i32,
    #[serde(rename = "wins")]
    pub le_wins: i32,
    #[serde(rename = "losses")]
    pub le_losses: i32,
    #[serde(rename = "kills")]
    pub le_kills: i32,
    #[serde(rename = "deaths")]
    pub le_deaths: i32,
    #[serde(rename = "assists")]
    pub le_assists: i32,
    #[serde(rename = "score")]
    pub le_score: i32,
    #[serde(rename = "damage_done")]
    pub le_damage_done: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    #[serde(rename = "page")]
    pub lr_page: i32,
    #[serde(rename = "per_page")]
    pub lr_per_page: i32,
    #[serde(rename = "season")]
    pub lr_season: i32,
    #[serde(rename = "region")]
    pub lr_region: i32,
    #[serde(rename = "entries")]
    pub lr_entries: Vec<LeaderboardEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMap {
    #[serde(rename = "map_id")]
    pub gm_map_id: i32,
    #[serde(rename = "name")]
    pub gm_name: String,
    #[serde(rename = "description")]
    pub gm_description: String,
    #[serde(rename = "creator_name")]
    pub gm_creator_name: String,
    #[serde(rename = "votes")]
    pub gm_votes: i32,
    #[serde(rename = "gameplays")]
    pub gm_gameplays: i32,
    #[serde(rename = "playtime")]
    pub gm_playtime: i32,
    #[serde(rename = "category")]
    pub gm_category: i32,
    #[serde(rename = "created_at")]
    pub gm_created_at: String,
    #[serde(rename = "updated_at")]
    pub gm_updated_at: String,
    #[serde(rename = "leaderboard_type")]
    pub gm_leaderboard_type: String,
    #[serde(rename = "leaderboard_order")]
    pub gm_leaderboard_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLeaderboardEntry {
    #[serde(rename = "position")]
    pub mle_position: i32,
    #[serde(rename = "player_name")]
    pub mle_player_name: String,
    #[serde(rename = "value")]
    pub mle_value: i32,
    #[serde(rename = "date")]
    pub mle_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLeaderboardResponse {
    #[serde(rename = "page")]
    pub mlr_page: i32,
    #[serde(rename = "per_page")]
    pub mlr_per_page: i32,
    #[serde(rename = "map_name")]
    pub mlr_map_name: String,
    #[serde(rename = "leaderboard_type")]
    pub mlr_leaderboard_type: String,
    #[serde(rename = "leaderboard_order")]
    pub mlr_leaderboard_order: i32,
    #[serde(rename = "entries")]
    pub mlr_entries: Vec<MapLeaderboardEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    #[serde(rename = "mod_id")]
    pub mod_id: i32,
    #[serde(rename = "name")]
    pub mod_name: String,
    #[serde(rename = "description")]
    pub mod_description: String,
    #[serde(rename = "creator_name")]
    pub mod_creator_name: String,
    #[serde(rename = "votes")]
    pub mod_votes: i32,
    #[serde(rename = "featured")]
    pub mod_featured: bool,
    #[serde(rename = "version")]
    pub mod_version: i32,
    #[serde(rename = "created_at")]
    pub mod_created_at: String,
    #[serde(rename = "updated_at")]
    pub mod_updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModsResponse {
    #[serde(rename = "page")]
    pub mods_page: i32,
    #[serde(rename = "per_page")]
    pub mods_per_page: i32,
    #[serde(rename = "mods")]
    pub mods_mods: Vec<Mod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketListing {
    #[serde(rename = "price")]
    pub ml_price: i32,
    #[serde(rename = "seller_name")]
    pub ml_seller_name: String,
    #[serde(rename = "listed_at")]
    pub ml_listed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOwner {
    #[serde(rename = "player_name")]
    pub mo_player_name: String,
    #[serde(rename = "count")]
    pub mo_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    #[serde(rename = "date")]
    pub ph_date: String,
    #[serde(rename = "average_price")]
    pub ph_average_price: f64,
    #[serde(rename = "sales")]
    pub ph_sales: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketResponse {
    #[serde(rename = "skin_index")]
    pub mr_skin_index: i32,
    #[serde(rename = "total_listings")]
    pub mr_total_listings: i32,
    #[serde(rename = "lowest_price")]
    pub mr_lowest_price: i32,
    #[serde(rename = "average_price")]
    pub mr_average_price: f64,
    #[serde(rename = "total_circulating")]
    pub mr_total_circulating: i32,
    #[serde(rename = "listings")]
    pub mr_listings: Vec<MarketListing>,
    #[serde(rename = "owners")]
    pub mr_owners: Vec<MarketOwner>,
    #[serde(rename = "price_history")]
    pub mr_price_history: Vec<PriceHistory>,
}
