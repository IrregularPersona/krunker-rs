use serde::{Deserialize, Serialize};

/// Regional ranked statistics for the current season.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedProfile {
    #[serde(rename = "region")]
    /// Numeric region ID as returned by the API.
    /// JSON key: `region`.
    pub ranked_region: i32,
    #[serde(rename = "mmr")]
    /// Matchmaking rating.
    /// JSON key: `mmr`.
    pub ranked_mmr: i32,
    #[serde(rename = "wins")]
    /// Number of wins.
    /// JSON key: `wins`.
    pub ranked_wins: i32,
    #[serde(rename = "losses")]
    /// Number of losses.
    /// JSON key: `losses`.
    pub ranked_losses: i32,
    #[serde(rename = "kills")]
    /// Kill count.
    /// JSON key: `kills`.
    pub ranked_kills: i32,
    #[serde(rename = "deaths")]
    /// Death count.
    /// JSON key: `deaths`.
    pub ranked_deaths: i32,
    #[serde(rename = "assists")]
    /// Assist count.
    /// JSON key: `assists`.
    pub ranked_assists: i32,
    #[serde(rename = "score")]
    /// Score total.
    /// JSON key: `score`.
    pub ranked_score: i64,
    #[serde(rename = "damage_done")]
    /// Total damage dealt.
    /// JSON key: `damage_done`.
    pub ranked_damage_done: i64,
    #[serde(rename = "time_played")]
    /// Total time played in seconds.
    /// JSON key: `time_played`.
    pub ranked_time_played: i64,
}

/// A player profile and lifetime statistics returned by `Client::get_player`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// In-game username.
    pub player_name: String,
    #[serde(rename = "clan")]
    /// Clan name; empty when the player has no clan.
    /// JSON key: `clan`.
    pub player_clan: String,
    #[serde(rename = "verified")]
    /// Whether the player is verified.
    /// JSON key: `verified`.
    pub player_verified: bool,
    #[serde(rename = "flag")]
    /// Country flag index.
    /// JSON key: `flag`.
    pub player_flag: i32,
    #[serde(rename = "badges")]
    /// Optional badge ID list; absent or null values deserialize as `None`.
    /// JSON key: `badges`.
    pub player_badges: Option<Vec<i32>>,
    #[serde(rename = "following")]
    /// Number of players followed.
    /// JSON key: `following`.
    pub player_following: i32,
    #[serde(rename = "followers")]
    /// Follower count.
    /// JSON key: `followers`.
    pub player_followers: i32,
    #[serde(rename = "ranked")]
    /// Optional current-season ranked profiles for regions with completed placements.
    /// JSON key: `ranked`.
    pub player_ranked: Option<Vec<RankedProfile>>,
    #[serde(rename = "kr")]
    /// Currency balance in KR.
    /// JSON key: `kr`.
    pub player_kr: u64,
    #[serde(rename = "level")]
    /// Player level calculated by the API.
    /// JSON key: `level`.
    pub player_level: i32,
    #[serde(rename = "junk")]
    /// Junk/ELO rating.
    /// JSON key: `junk`.
    pub player_junk: f64,
    #[serde(rename = "inventory")]
    /// Total inventory skin value in KR.
    /// JSON key: `inventory`.
    pub player_inventory: u64,
    #[serde(rename = "score")]
    /// Score total.
    /// JSON key: `score`.
    pub player_score: u64,
    #[serde(rename = "spk")]
    /// Score per kill.
    /// JSON key: `spk`.
    pub player_spk: f64,
    #[serde(rename = "kills")]
    /// Kill count.
    /// JSON key: `kills`.
    pub player_kills: u64,
    #[serde(rename = "deaths")]
    /// Death count.
    /// JSON key: `deaths`.
    pub player_deaths: u64,
    #[serde(rename = "kdr")]
    /// Kill/death ratio calculated by the API.
    /// JSON key: `kdr`.
    pub player_kdr: f64,
    #[serde(rename = "kpg")]
    /// Kills per game calculated by the API.
    /// JSON key: `kpg`.
    pub player_kpg: f64,
    #[serde(rename = "games")]
    /// Games played.
    /// JSON key: `games`.
    pub player_games: i32,
    #[serde(rename = "wins")]
    /// Number of wins.
    /// JSON key: `wins`.
    pub player_wins: i32,
    #[serde(rename = "losses")]
    /// Number of losses.
    /// JSON key: `losses`.
    pub player_losses: i32,
    #[serde(rename = "assists")]
    /// Assist count.
    /// JSON key: `assists`.
    pub player_assists: i32,
    #[serde(rename = "melees")]
    /// Melee kill count.
    /// JSON key: `melees`.
    pub player_melees: i32,
    #[serde(rename = "beatdowns")]
    /// Fist kill count.
    /// JSON key: `beatdowns`.
    pub player_beatdowns: i32,
    #[serde(rename = "bullseyes")]
    /// Thrown weapon kill count.
    /// JSON key: `bullseyes`.
    pub player_bullseyes: i32,
    #[serde(rename = "headshots")]
    /// Headshot kill count.
    /// JSON key: `headshots`.
    pub player_headshots: i32,
    #[serde(rename = "legshots")]
    /// Leg shot kill count.
    /// JSON key: `legshots`.
    pub player_legshots: i32,
    #[serde(rename = "wallbangs")]
    /// Wallbang kill count.
    /// JSON key: `wallbangs`.
    pub player_wallbangs: i32,
    #[serde(rename = "shots")]
    /// Shots fired.
    /// JSON key: `shots`.
    pub player_shots: u64,
    #[serde(rename = "hits")]
    /// Shots that hit.
    /// JSON key: `hits`.
    pub player_hits: u64,
    #[serde(rename = "misses")]
    /// Shots that missed.
    /// JSON key: `misses`.
    pub player_misses: u64,
    #[serde(rename = "time_played")]
    /// Total time played in seconds.
    /// JSON key: `time_played`.
    pub player_time_played: i64,
    #[serde(rename = "nukes")]
    /// Nukes earned.
    /// JSON key: `nukes`.
    pub player_nukes: i32,
    #[serde(rename = "airdrops")]
    /// Airdrops earned.
    /// JSON key: `airdrops`.
    pub player_airdrops: i32,
    #[serde(rename = "airdrops_stolen")]
    /// Airdrops stolen.
    /// JSON key: `airdrops_stolen`.
    pub player_airdrops_stolen: i32,
    #[serde(rename = "slimes")]
    /// Slimes earned.
    /// JSON key: `slimes`.
    pub player_slimes: i32,
    #[serde(rename = "juggernauts")]
    /// Juggernauts earned.
    /// JSON key: `juggernauts`.
    pub player_juggernauts: i32,
    #[serde(rename = "juggernauts_killed")]
    /// Juggernauts killed.
    /// JSON key: `juggernauts_killed`.
    pub player_juggernauts_killed: i32,
    #[serde(rename = "warmachines")]
    /// Warmachines earned.
    /// JSON key: `warmachines`.
    pub player_warmachines: i32,
    #[serde(rename = "hacker_tagged")]
    /// Whether the API has flagged the player as a cheater.
    /// JSON key: `hacker_tagged`.
    pub player_hacker_tagged: bool,
    #[serde(rename = "created_at")]
    /// Creation timestamp as an API string, normally RFC3339.
    /// JSON key: `created_at`.
    pub player_created_at: String,
}

/// Owned quantity of one skin, excluding copies listed on the market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    #[serde(rename = "skin_index")]
    /// Numeric skin/item type index.
    /// JSON key: `skin_index`.
    pub inventory_skin_index: i32,
    #[serde(rename = "count")]
    /// Copies owned, excluding copies listed on the market.
    /// JSON key: `count`.
    pub inventory_count: i32,
}

/// One entry in a player's ranked match history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatch {
    #[serde(rename = "match_id")]
    /// Unique ranked match identifier.
    /// JSON key: `match_id`.
    pub pm_match_id: i64,
    #[serde(rename = "date")]
    /// Recorded timestamp as an API string, normally RFC3339.
    /// JSON key: `date`.
    pub pm_date: String,
    #[serde(rename = "map")]
    /// Numeric map ID.
    /// JSON key: `map`.
    pub pm_map: i32,
    #[serde(rename = "duration")]
    /// Match duration in seconds.
    /// JSON key: `duration`.
    pub pm_duration: i64,
    #[serde(rename = "season")]
    /// Ranked season ID.
    /// JSON key: `season`.
    pub pm_season: i32,
    #[serde(rename = "region")]
    /// Numeric region ID as returned by the API.
    /// JSON key: `region`.
    pub pm_region: i32,
    #[serde(rename = "kills")]
    /// Kill count.
    /// JSON key: `kills`.
    pub pm_kills: i64,
    #[serde(rename = "deaths")]
    /// Death count.
    /// JSON key: `deaths`.
    pub pm_deaths: i64,
    #[serde(rename = "assists")]
    /// Assist count.
    /// JSON key: `assists`.
    pub pm_assists: i64,
    #[serde(rename = "score")]
    /// Score total.
    /// JSON key: `score`.
    pub pm_score: i64,
    #[serde(rename = "damage_done")]
    /// Total damage dealt.
    /// JSON key: `damage_done`.
    pub pm_damage_done: i64,
    #[serde(rename = "headshots")]
    /// Headshot kill count.
    /// JSON key: `headshots`.
    pub pm_headshots: i32,
    #[serde(rename = "accuracy")]
    /// Shot accuracy as an integer percentage.
    /// JSON key: `accuracy`.
    pub pm_accuracy: i32,
    #[serde(rename = "objective_score")]
    /// Objective score.
    /// JSON key: `objective_score`.
    pub pm_objective_score: i32,
    #[serde(rename = "kr")]
    /// KR earned in this match.
    /// JSON key: `kr`.
    pub pm_kr: i32,
    #[serde(rename = "victory")]
    /// Numeric match result; 1 indicates victory.
    /// JSON key: `victory`.
    pub pm_victory: i32,
    #[serde(rename = "rounds_won")]
    /// Rounds won.
    /// JSON key: `rounds_won`.
    pub pm_rounds_won: i32,
    #[serde(rename = "team")]
    /// Numeric team ID.
    /// JSON key: `team`.
    pub pm_team: i32,
    #[serde(rename = "play_time")]
    /// Time played within this match in seconds.
    /// JSON key: `play_time`.
    pub pm_play_time: i64,
}

/// A page of ranked match history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchesResponse {
    #[serde(rename = "page")]
    /// Current page number, starting at 1.
    /// JSON key: `page`.
    pub pmr_page: i32,
    #[serde(rename = "per_page")]
    /// Page size reported by the server.
    /// JSON key: `per_page`.
    pub pmr_per_page: i32,
    #[serde(rename = "matches")]
    /// Optional ranked matches on this page, ordered newest first.
    /// JSON key: `matches`.
    pub pmr_matches: Option<Vec<PlayerMatch>>,
}

/// A social post with votes and comment count.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "date")]
    /// Recorded timestamp as an API string, normally RFC3339.
    /// JSON key: `date`.
    pub post_date: String,
    #[serde(rename = "text")]
    /// Post text.
    /// JSON key: `text`.
    pub post_text: String,
    #[serde(rename = "votes")]
    /// Upvote count.
    /// JSON key: `votes`.
    pub post_votes: i32,
    #[serde(rename = "comment_count")]
    /// Number of comments.
    /// JSON key: `comment_count`.
    pub post_comment_count: i32,
}

/// A page of social posts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostsResponse {
    #[serde(rename = "page")]
    /// Current page number, starting at 1.
    /// JSON key: `page`.
    pub posts_page: i32,
    #[serde(rename = "per_page")]
    /// Page size reported by the server.
    /// JSON key: `per_page`.
    pub posts_per_page: i32,
    #[serde(rename = "posts")]
    /// Optional social posts on this page.
    /// JSON key: `posts`.
    pub posts_posts: Option<Vec<Post>>,
}

/// A player's statistics within one ranked match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchParticipant {
    #[serde(rename = "player_name")]
    /// In-game username.
    /// JSON key: `player_name`.
    pub mp_player_name: String,
    #[serde(rename = "kills")]
    /// Kill count.
    /// JSON key: `kills`.
    pub mp_kills: i32,
    #[serde(rename = "deaths")]
    /// Death count.
    /// JSON key: `deaths`.
    pub mp_deaths: i32,
    #[serde(rename = "assists")]
    /// Assist count.
    /// JSON key: `assists`.
    pub mp_assists: i32,
    #[serde(rename = "score")]
    /// Score total.
    /// JSON key: `score`.
    pub mp_score: i64,
    #[serde(rename = "damage_done")]
    /// Total damage dealt.
    /// JSON key: `damage_done`.
    pub mp_damage_done: i64,
    #[serde(rename = "headshots")]
    /// Headshot kill count.
    /// JSON key: `headshots`.
    pub mp_headshots: i32,
    #[serde(rename = "accuracy")]
    /// Shot accuracy as an integer percentage.
    /// JSON key: `accuracy`.
    pub mp_accuracy: i32,
    #[serde(rename = "objective_score")]
    /// Objective score.
    /// JSON key: `objective_score`.
    pub mp_objective_score: i32,
    #[serde(rename = "victory")]
    /// Numeric match result; 1 indicates victory.
    /// JSON key: `victory`.
    pub mp_victory: i32,
    #[serde(rename = "rounds_won")]
    /// Rounds won.
    /// JSON key: `rounds_won`.
    pub mp_rounds_won: i32,
    #[serde(rename = "team")]
    /// Numeric team ID.
    /// JSON key: `team`.
    pub mp_team: i32,
    #[serde(rename = "play_time")]
    /// Time played within this match in seconds.
    /// JSON key: `play_time`.
    pub mp_play_time: i64,
}

/// Ranked match metadata and optional participant statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    #[serde(rename = "match_id")]
    /// Unique ranked match identifier.
    pub match_id: i64,
    #[serde(rename = "date")]
    /// Recorded timestamp as an API string, normally RFC3339.
    /// JSON key: `date`.
    pub match_date: String,
    #[serde(rename = "map")]
    /// Numeric map ID.
    /// JSON key: `map`.
    pub match_map: i32,
    #[serde(rename = "duration")]
    /// Match duration in seconds.
    /// JSON key: `duration`.
    pub match_duration: i32,
    #[serde(rename = "season")]
    /// Ranked season ID.
    /// JSON key: `season`.
    pub match_season: i32,
    #[serde(rename = "region")]
    /// Numeric region ID as returned by the API.
    /// JSON key: `region`.
    pub match_region: i32,
    #[serde(rename = "participants")]
    /// Optional match participants, ordered by team then score.
    /// JSON key: `participants`.
    pub match_participants: Option<Vec<MatchParticipant>>,
}

/// A public clan profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clan {
    #[serde(rename = "name")]
    /// Resource name.
    /// JSON key: `name`.
    pub clan_name: String,
    #[serde(rename = "owner_name")]
    /// Clan owner's in-game username.
    /// JSON key: `owner_name`.
    pub clan_owner_name: String,
    #[serde(rename = "score")]
    /// Score total.
    /// JSON key: `score`.
    pub clan_score: i64,
    #[serde(rename = "rank")]
    /// Clan rank.
    /// JSON key: `rank`.
    pub clan_rank: i32,
    #[serde(rename = "member_count")]
    /// Number of clan members.
    /// JSON key: `member_count`.
    pub clan_member_count: i32,
    #[serde(rename = "created_at")]
    /// Creation timestamp as an API string, normally RFC3339.
    /// JSON key: `created_at`.
    pub clan_created_at: String,
    #[serde(rename = "discord")]
    /// Discord invite code; empty when unset.
    /// JSON key: `discord`.
    pub clan_discord: String,
}

/// A clan member and their numeric API role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanMember {
    #[serde(rename = "player_name")]
    /// In-game username.
    /// JSON key: `player_name`.
    pub cm_player_name: String,
    #[serde(rename = "role")]
    /// Numeric clan role as returned by the API.
    /// JSON key: `role`.
    pub cm_role: i32,
}

/// A page of clan members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanMembersResponse {
    #[serde(rename = "page")]
    /// Current page number, starting at 1.
    /// JSON key: `page`.
    pub cmr_page: i32,
    #[serde(rename = "per_page")]
    /// Page size reported by the server.
    /// JSON key: `per_page`.
    pub cmr_per_page: i32,
    #[serde(rename = "members")]
    /// Optional clan members on this page.
    /// JSON key: `members`.
    pub cmr_members: Option<Vec<ClanMember>>,
}

/// One player's regional ranked standing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    #[serde(rename = "position")]
    /// Position on the returned leaderboard.
    /// JSON key: `position`.
    pub le_position: i32,
    #[serde(rename = "player_name")]
    /// In-game username.
    /// JSON key: `player_name`.
    pub le_player_name: String,
    #[serde(rename = "mmr")]
    /// Matchmaking rating.
    /// JSON key: `mmr`.
    pub le_mmr: i32,
    #[serde(rename = "wins")]
    /// Number of wins.
    /// JSON key: `wins`.
    pub le_wins: i32,
    #[serde(rename = "losses")]
    /// Number of losses.
    /// JSON key: `losses`.
    pub le_losses: i32,
    #[serde(rename = "kills")]
    /// Kill count.
    /// JSON key: `kills`.
    pub le_kills: i32,
    #[serde(rename = "deaths")]
    /// Death count.
    /// JSON key: `deaths`.
    pub le_deaths: i32,
    #[serde(rename = "assists")]
    /// Assist count.
    /// JSON key: `assists`.
    pub le_assists: i32,
    #[serde(rename = "score")]
    /// Score total.
    /// JSON key: `score`.
    pub le_score: i64,
    #[serde(rename = "damage_done")]
    /// Total damage dealt.
    /// JSON key: `damage_done`.
    pub le_damage_done: i64,
}

/// A page of ranked standings for a region and season.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    #[serde(rename = "page")]
    /// Current page number, starting at 1.
    /// JSON key: `page`.
    pub lr_page: i32,
    #[serde(rename = "per_page")]
    /// Page size reported by the server.
    /// JSON key: `per_page`.
    pub lr_per_page: i32,
    #[serde(rename = "season")]
    /// Current ranked season ID.
    /// JSON key: `season`.
    pub lr_season: i32,
    #[serde(rename = "region")]
    /// Numeric region ID as returned by the API.
    /// JSON key: `region`.
    pub lr_region: i32,
    #[serde(rename = "entries")]
    /// Optional leaderboard entries on this page.
    /// JSON key: `entries`.
    pub lr_entries: Option<Vec<LeaderboardEntry>>,
}

/// Map metadata, activity totals, and leaderboard settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMap {
    #[serde(rename = "map_id")]
    /// Unique map identifier.
    /// JSON key: `map_id`.
    pub gm_map_id: i32,
    #[serde(rename = "name")]
    /// Resource name.
    /// JSON key: `name`.
    pub gm_name: String,
    #[serde(rename = "description")]
    /// Creator-supplied description.
    /// JSON key: `description`.
    pub gm_description: String,
    #[serde(rename = "creator_name")]
    /// Creator's in-game username.
    /// JSON key: `creator_name`.
    pub gm_creator_name: String,
    #[serde(rename = "votes")]
    /// Upvote count.
    /// JSON key: `votes`.
    pub gm_votes: i32,
    #[serde(rename = "gameplays")]
    /// Number of times the map has been played.
    /// JSON key: `gameplays`.
    pub gm_gameplays: i32,
    #[serde(rename = "playtime")]
    /// Total map playtime in milliseconds.
    /// JSON key: `playtime`.
    pub gm_playtime: i64,
    #[serde(rename = "category")]
    /// Numeric map category ID.
    /// JSON key: `category`.
    pub gm_category: i32,
    #[serde(rename = "created_at")]
    /// Creation timestamp as an API string, normally RFC3339.
    /// JSON key: `created_at`.
    pub gm_created_at: String,
    #[serde(rename = "updated_at")]
    /// Last update timestamp as an API string, normally RFC3339.
    /// JSON key: `updated_at`.
    pub gm_updated_at: String,
    #[serde(rename = "leaderboard_type")]
    /// API leaderboard type, such as time or score; can be empty when unset.
    /// JSON key: `leaderboard_type`.
    pub gm_leaderboard_type: String,
    #[serde(rename = "leaderboard_order")]
    /// Sort direction: 0 = ascending, 1 = descending.
    /// JSON key: `leaderboard_order`.
    pub gm_leaderboard_order: i32,
}

/// One recorded score or time on a map leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLeaderboardEntry {
    #[serde(rename = "position")]
    /// Position on the returned leaderboard.
    /// JSON key: `position`.
    pub mle_position: i32,
    #[serde(rename = "player_name")]
    /// In-game username.
    /// JSON key: `player_name`.
    pub mle_player_name: String,
    #[serde(rename = "value")]
    /// Score or time value; interpret it using the response's leaderboard type.
    /// JSON key: `value`.
    pub mle_value: i32,
    #[serde(rename = "date")]
    /// Recorded timestamp as an API string, normally RFC3339.
    /// JSON key: `date`.
    pub mle_date: String,
}

/// A page of standings for a case-sensitive map name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLeaderboardResponse {
    #[serde(rename = "page")]
    /// Current page number, starting at 1.
    /// JSON key: `page`.
    pub mlr_page: i32,
    #[serde(rename = "per_page")]
    /// Page size reported by the server.
    /// JSON key: `per_page`.
    pub mlr_per_page: i32,
    #[serde(rename = "map_name")]
    /// Case-sensitive map name.
    /// JSON key: `map_name`.
    pub mlr_map_name: String,
    #[serde(rename = "leaderboard_type")]
    /// API leaderboard type, such as time or score; can be empty when unset.
    /// JSON key: `leaderboard_type`.
    pub mlr_leaderboard_type: String,
    #[serde(rename = "leaderboard_order")]
    /// Sort direction: 0 = ascending, 1 = descending.
    /// JSON key: `leaderboard_order`.
    pub mlr_leaderboard_order: i32,
    #[serde(rename = "entries")]
    /// Optional leaderboard entries on this page.
    /// JSON key: `entries`.
    pub mlr_entries: Option<Vec<MapLeaderboardEntry>>,
}

/// An active mod's metadata and publication details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    #[serde(rename = "mod_id")]
    /// Unique mod identifier.
    pub mod_id: i32,
    #[serde(rename = "name")]
    /// Resource name.
    /// JSON key: `name`.
    pub mod_name: String,
    #[serde(rename = "description")]
    /// Creator-supplied description.
    /// JSON key: `description`.
    pub mod_description: String,
    #[serde(rename = "creator_name")]
    /// Creator's in-game username.
    /// JSON key: `creator_name`.
    pub mod_creator_name: String,
    #[serde(rename = "votes")]
    /// Upvote count.
    /// JSON key: `votes`.
    pub mod_votes: i32,
    #[serde(rename = "featured")]
    /// Whether the mod is featured.
    /// JSON key: `featured`.
    pub mod_featured: bool,
    #[serde(rename = "version")]
    /// Mod version number.
    /// JSON key: `version`.
    pub mod_version: i32,
    #[serde(rename = "created_at")]
    /// Creation timestamp as an API string, normally RFC3339.
    /// JSON key: `created_at`.
    pub mod_created_at: String,
    #[serde(rename = "updated_at")]
    /// Last update timestamp as an API string, normally RFC3339.
    /// JSON key: `updated_at`.
    pub mod_updated_at: String,
}

/// A page of active mods sorted by votes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModsResponse {
    #[serde(rename = "page")]
    /// Current page number, starting at 1.
    /// JSON key: `page`.
    pub mods_page: i32,
    #[serde(rename = "per_page")]
    /// Page size reported by the server.
    /// JSON key: `per_page`.
    pub mods_per_page: i32,
    #[serde(rename = "mods")]
    /// Optional active mods on this page, sorted by votes.
    /// JSON key: `mods`.
    pub mods_mods: Option<Vec<Mod>>,
}

/// One active market listing with an integer price in KR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketListing {
    #[serde(rename = "price")]
    /// Integer listing price in KR.
    /// JSON key: `price`.
    pub ml_price: i32,
    #[serde(rename = "seller_name")]
    /// Seller's in-game username.
    /// JSON key: `seller_name`.
    pub ml_seller_name: String,
    #[serde(rename = "listed_at")]
    /// Listing timestamp as an API string, normally RFC3339.
    /// JSON key: `listed_at`.
    pub ml_listed_at: String,
}

/// A player and the number of copies of a skin they own.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOwner {
    #[serde(rename = "player_name")]
    /// In-game username.
    /// JSON key: `player_name`.
    pub mo_player_name: String,
    #[serde(rename = "count")]
    /// Number of copies owned.
    /// JSON key: `count`.
    pub mo_count: i32,
}

/// Daily sale statistics for one skin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    #[serde(rename = "date")]
    /// Calendar date as a YYYY-MM-DD string.
    /// JSON key: `date`.
    pub ph_date: String,
    #[serde(rename = "average_price")]
    /// Average sale price in KR on this calendar day.
    /// JSON key: `average_price`.
    pub ph_average_price: f64,
    #[serde(rename = "sales")]
    /// Number of sales on this day.
    /// JSON key: `sales`.
    pub ph_sales: i32,
}

/// Market prices, paginated listings, owners, and recent sale history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketResponse {
    #[serde(rename = "skin_index")]
    /// Numeric skin/item type index.
    /// JSON key: `skin_index`.
    pub mr_skin_index: i32,
    #[serde(rename = "total_listings")]
    /// Total active listings across all listing pages.
    /// JSON key: `total_listings`.
    pub mr_total_listings: i32,
    #[serde(rename = "lowest_price")]
    /// Lowest active listing price in KR; zero when there are no listings.
    /// JSON key: `lowest_price`.
    pub mr_lowest_price: i32,
    #[serde(rename = "average_price")]
    /// Average sale price in KR over the API's recent sales window, typically seven days.
    /// JSON key: `average_price`.
    pub mr_average_price: f64,
    #[serde(rename = "total_circulating")]
    /// Total quantity of this skin in circulation.
    /// JSON key: `total_circulating`.
    pub mr_total_circulating: i32,
    #[serde(rename = "listings")]
    /// Optional listing page, sorted by price from lowest to highest.
    /// JSON key: `listings`.
    pub mr_listings: Option<Vec<MarketListing>>,
    #[serde(rename = "owners")]
    /// Optional top owners, sorted by quantity from highest to lowest.
    /// JSON key: `owners`.
    pub mr_owners: Option<Vec<MarketOwner>>,
    #[serde(rename = "price_history")]
    /// Optional daily sale averages for recent days with sales.
    /// JSON key: `price_history`.
    pub mr_price_history: Option<Vec<PriceHistory>>,
}

/// The API's JSON response to HTTP 429; client calls return `Error::RateLimit`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResponse {
    /// API error message.
    pub error: String,
    /// Recommended retry delay in seconds.
    pub retry_after: u64,
}

/// The API's standard JSON error payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericErrorResponse {
    /// API error message.
    pub error: String,
}

/// Advisory rate limit information from response headers; shared by client clones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Maximum requests in the server's current window.
    pub limit: u32,
    /// Requests remaining when the response was generated.
    pub remaining: u32,
    /// Rate limit reset time as Unix seconds.
    pub reset: u64,
}
