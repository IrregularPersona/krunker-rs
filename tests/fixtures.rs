use krunker_rs::*;

#[test]
fn documented_response_fixtures_deserialize_into_public_types() {
    macro_rules! fixture {
        ($name:literal, $ty:ty) => {
            serde_json::from_str::<$ty>(include_str!(concat!("fixtures/", $name, ".json")))
                .unwrap();
        };
    }
    fixture!("forbidden", GenericErrorResponse);
    fixture!("rate_limit", RateLimitResponse);
    fixture!("player", Player);
    fixture!("inventory", Vec<InventoryItem>);
    fixture!("player_matches", PlayerMatchesResponse);
    fixture!("posts", PostsResponse);
    fixture!("match", Match);
    fixture!("clan", Clan);
    fixture!("clan_members", ClanMembersResponse);
    fixture!("leaderboard", LeaderboardResponse);
    fixture!("map", GameMap);
    fixture!("map_leaderboard", MapLeaderboardResponse);
    fixture!("mods", ModsResponse);
    fixture!("mod", Mod);
    fixture!("market", MarketResponse);
    fixture!("error", GenericErrorResponse);
}

#[test]
fn absent_null_and_empty_collections_remain_distinguishable() {
    for body in [
        r#"{"page":1,"per_page":10}"#,
        r#"{"page":1,"per_page":10,"posts":null}"#,
    ] {
        assert!(
            serde_json::from_str::<PostsResponse>(body)
                .unwrap()
                .posts_posts
                .is_none()
        );
    }
    let response: PostsResponse =
        serde_json::from_str(r#"{"page":1,"per_page":10,"posts":[]}"#).unwrap();
    assert!(response.posts_posts.unwrap().is_empty());
}
