mod support;

use krunker_rs::{Client, Error};
use reqwest::StatusCode;
use std::error::Error as StdError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use support::{Response, Server};

const PLAYER: &str = include_str!("fixtures/player.json");
const POSTS: &str = include_str!("fixtures/posts.json");
const MAP: &str = include_str!("fixtures/map.json");
const MODS: &str = include_str!("fixtures/mods.json");

#[tokio::test]
async fn every_endpoint_uses_the_correct_route_and_response_type() {
    let fixtures = [
        PLAYER,
        include_str!("fixtures/inventory.json"),
        include_str!("fixtures/player_matches.json"),
        POSTS,
        include_str!("fixtures/match.json"),
        include_str!("fixtures/clan.json"),
        include_str!("fixtures/clan_members.json"),
        include_str!("fixtures/leaderboard.json"),
        MAP,
        include_str!("fixtures/map_leaderboard.json"),
        MODS,
        include_str!("fixtures/mod.json"),
        include_str!("fixtures/market.json"),
    ];
    let server = Server::start(fixtures.into_iter().map(Response::ok).collect()).await;
    let client = server.client();
    assert_eq!(
        client.get_player("Player").await.unwrap().player_name,
        "string"
    );
    assert_eq!(
        client.get_player_inventory("Player").await.unwrap()[0].inventory_count,
        5
    );
    assert_eq!(
        client
            .get_player_matches("Player", Some(2), Some(12))
            .await
            .unwrap()
            .pmr_matches
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        client
            .get_player_posts("Player", Some(2))
            .await
            .unwrap()
            .posts_posts
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        client.get_match(123456789).await.unwrap().match_id,
        123456789
    );
    assert_eq!(
        client.get_clan("Elite Squad").await.unwrap().clan_name,
        "Elite Squad"
    );
    assert_eq!(
        client
            .get_clan_members("Elite Squad", Some(2))
            .await
            .unwrap()
            .cmr_members
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        client
            .get_leaderboard(3, Some(2))
            .await
            .unwrap()
            .lr_entries
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        client.get_map("MyAwesomeMap").await.unwrap().gm_name,
        "MyAwesomeMap"
    );
    assert_eq!(
        client
            .get_map_leaderboard("MyAwesomeMap", Some(2))
            .await
            .unwrap()
            .mlr_entries
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        client
            .get_mods(Some(2))
            .await
            .unwrap()
            .mods_mods
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        client.get_mod("AwesomeMod").await.unwrap().mod_name,
        "AwesomeMod"
    );
    assert_eq!(
        client
            .get_market_skin(123, Some(2))
            .await
            .unwrap()
            .mr_skin_index,
        123
    );
    assert_eq!(
        server.targets(),
        [
            "/api/player/Player",
            "/api/player/Player/inventory",
            "/api/player/Player/matches?page=2&season=12",
            "/api/player/Player/posts?page=2",
            "/api/match/123456789",
            "/api/clan/Elite%20Squad",
            "/api/clan/Elite%20Squad/members?page=2",
            "/api/leaderboard/3?page=2",
            "/api/map/MyAwesomeMap",
            "/api/map/MyAwesomeMap/leaderboard?page=2",
            "/api/mods?page=2",
            "/api/mods/AwesomeMod",
            "/api/market/skin/123?page=2",
        ]
    );
    assert!(
        server
            .request(0)
            .to_ascii_lowercase()
            .contains("x-developer-api-key: test-key\r\n")
    );
}

#[tokio::test]
async fn names_are_encoded_as_single_segments_and_base_paths_are_preserved() {
    let cases = [
        ("Arena #1", "Arena%20%231"),
        ("Arena?mode=1", "Arena%3Fmode=1"),
        ("Arena/Classic", "Arena%2FClassic"),
        ("100% Fun", "100%25%20Fun"),
        ("東京", "%E6%9D%B1%E4%BA%AC"),
        ("%2E%2E", "%252E%252E"),
    ];
    let server = Server::start(cases.iter().map(|_| Response::ok(MAP)).collect()).await;
    let client = Client::builder("test-key")
        .base_url(format!("{}/custom/", server.base_url))
        .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
        .build()
        .unwrap();
    for (name, _) in cases {
        client.get_map(name).await.unwrap();
    }
    let expected: Vec<_> = cases
        .iter()
        .map(|(_, encoded)| format!("/api/custom/map/{encoded}"))
        .collect();
    assert_eq!(server.targets(), expected);
}

#[tokio::test]
async fn omitted_query_parameters_are_not_sent() {
    let server = Server::start(vec![
        Response::ok(include_str!("fixtures/player_matches.json")),
        Response::ok(include_str!("fixtures/leaderboard.json")),
        Response::ok(MODS),
    ])
    .await;
    let client = server.client();
    client
        .get_player_matches("Player", None, None)
        .await
        .unwrap();
    client.get_leaderboard(3, None).await.unwrap();
    client.get_mods(None).await.unwrap();
    assert_eq!(
        server.targets(),
        [
            "/api/player/Player/matches",
            "/api/leaderboard/3",
            "/api/mods"
        ]
    );
}

#[tokio::test]
async fn success_responses_require_complete_json_and_report_the_field_path() {
    let server = Server::start(vec![
        Response::ok(format!("{POSTS} trailing garbage")),
        Response::ok(r#"{"page":1,"per_page":"bad","posts":[]}"#),
        Response::ok(format!("{POSTS} \r\n\t")),
    ])
    .await;
    let client = server.client();
    match client.get_player_posts("Player", None).await.unwrap_err() {
        Error::Decode {
            body,
            field,
            message,
        } => {
            assert!(body.ends_with("trailing garbage"));
            assert!(message.contains("trailing characters"));
            assert_eq!(field, None);
        }
        other => panic!("Unexpected error: {other}"),
    }
    match client.get_player_posts("Player", None).await.unwrap_err() {
        Error::Decode { field, .. } => assert_eq!(field.as_deref(), Some("per_page")),
        other => panic!("Unexpected error: {other}"),
    }
    assert_eq!(
        client
            .get_player_posts("Player", None)
            .await
            .unwrap()
            .posts_page,
        1
    );
}

#[tokio::test]
async fn api_errors_preserve_json_messages_plain_text_and_empty_status_reasons() {
    let server = Server::start(vec![
        Response::new("404 Not Found", r#"{"error":"Missing player"}"#),
        Response::new("502 Bad Gateway", "upstream unavailable"),
        Response::new("500 Internal Server Error", ""),
    ])
    .await;
    let client = server.client();
    for (expected_status, expected_message) in [
        (StatusCode::NOT_FOUND, "Missing player"),
        (StatusCode::BAD_GATEWAY, "upstream unavailable"),
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
    ] {
        match client.get_player("Player").await.unwrap_err() {
            Error::Api { status, message } => {
                assert_eq!(status, expected_status);
                assert_eq!(message, expected_message);
            }
            other => panic!("Unexpected error: {other}"),
        }
    }
}

#[tokio::test]
async fn body_transport_failures_are_not_swallowed_and_keep_their_source() {
    let server = Server::start(vec![
        Response::new("500 Internal Server Error", "partial").truncated(),
    ])
    .await;
    let error = server.client().get_player("Player").await.unwrap_err();
    assert!(matches!(error, Error::Http(_)));
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<reqwest::Error>()
            .is_some()
    );
}

#[tokio::test]
async fn rate_limit_snapshots_are_shared_and_missing_headers_clear_old_values() {
    let server = Server::start(vec![
        Response::ok(POSTS)
            .header("X-RateLimit-Limit", "240")
            .header("X-RateLimit-Remaining", "239")
            .header("X-RateLimit-Reset", "1790804663"),
        Response::ok(POSTS).header("X-RateLimit-Limit", "invalid"),
    ])
    .await;
    let client = server.client();
    let clone = client.clone();
    assert!(clone.last_rate_limit().await.is_none());
    client.get_player_posts("Player", None).await.unwrap();
    let limit = clone.last_rate_limit().await.unwrap();
    assert_eq!(
        (limit.limit, limit.remaining, limit.reset),
        (240, 239, 1790804663)
    );
    clone.get_player_posts("Player", None).await.unwrap();
    assert!(client.last_rate_limit().await.is_none());
}

#[tokio::test]
async fn every_429_is_classified_as_a_rate_limit_with_delay_fallbacks() {
    let future_reset =
        (SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + Duration::from_secs(60)).as_secs();
    let server = Server::start(vec![
        Response::new(
            "429 Too Many Requests",
            r#"{"error":"Limited","retry_after":3}"#,
        )
        .header("Retry-After", "10"),
        Response::new("429 Too Many Requests", "proxy error").header("Retry-After", "7"),
        Response::new("429 Too Many Requests", "proxy error").header(
            "Retry-After",
            httpdate::fmt_http_date(SystemTime::now() + Duration::from_secs(60)),
        ),
        Response::new("429 Too Many Requests", "proxy error")
            .header("Retry-After", "invalid")
            .header("X-RateLimit-Reset", future_reset.to_string()),
        Response::new("429 Too Many Requests", ""),
    ])
    .await;
    let client = server.client();
    for (minimum, maximum) in [(3, 3), (7, 7), (1, 60), (1, 60), (1, 1)] {
        match client.get_player("Player").await.unwrap_err() {
            Error::RateLimit { retry_after } => assert!((minimum..=maximum).contains(&retry_after)),
            other => panic!("Unexpected error: {other}"),
        }
    }
    assert_eq!(server.targets().len(), 5, "Retries are disabled by default");
}

#[tokio::test]
async fn optional_rate_limit_retry_waits_then_succeeds() {
    let server = Server::start(vec![
        Response::new(
            "429 Too Many Requests",
            r#"{"error":"Limited","retry_after":1}"#,
        ),
        Response::ok(PLAYER),
    ])
    .await;
    let client = Client::builder("test-key")
        .base_url(&server.base_url)
        .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
        .retry_rate_limits(1)
        .build()
        .unwrap();
    let start = std::time::Instant::now();
    client.get_player("Player").await.unwrap();
    assert!(start.elapsed() >= Duration::from_secs(1));
    assert_eq!(server.targets().len(), 2);
}

#[tokio::test]
async fn rate_limit_retries_are_bounded_and_other_statuses_are_not_retried() {
    let server = Server::start(
        (0..3)
            .map(|_| {
                Response::new(
                    "429 Too Many Requests",
                    r#"{"error":"Limited","retry_after":0}"#,
                )
            })
            .collect(),
    )
    .await;
    let client = Client::builder("test-key")
        .base_url(&server.base_url)
        .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
        .retry_rate_limits(2)
        .build()
        .unwrap();
    assert!(matches!(
        client.get_player("Player").await,
        Err(Error::RateLimit { retry_after: 0 })
    ));
    assert_eq!(server.targets().len(), 3);

    let server = Server::start(vec![Response::new(
        "503 Service Unavailable",
        "unavailable",
    )])
    .await;
    let client = Client::builder("test-key")
        .base_url(&server.base_url)
        .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
        .retry_rate_limits(2)
        .build()
        .unwrap();
    assert!(matches!(
        client.get_player("Player").await,
        Err(Error::Api {
            status: StatusCode::SERVICE_UNAVAILABLE,
            ..
        })
    ));
    assert_eq!(server.targets().len(), 1);
}

#[tokio::test]
async fn request_timeout_also_applies_to_an_injected_http_client() {
    let server = Server::start(vec![
        Response::ok(PLAYER).delayed(Duration::from_millis(200)),
    ])
    .await;
    let client = Client::builder("test-key")
        .base_url(&server.base_url)
        .http_client(
            reqwest::Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
        )
        .timeout(Duration::from_millis(20))
        .build()
        .unwrap();
    match client.get_player("Player").await.unwrap_err() {
        Error::Http(error) => assert!(error.is_timeout()),
        other => panic!("Unexpected error: {other}"),
    }
}

#[tokio::test]
async fn the_default_client_does_not_follow_redirects_with_the_api_key() {
    let destination = Server::start(vec![Response::ok(PLAYER)]).await;
    let server = Server::start(vec![Response::new("302 Found", "").header(
        "Location",
        format!("{}/player/Player", destination.base_url),
    )])
    .await;
    let client = Client::builder("test-key")
        .base_url(&server.base_url)
        .build()
        .unwrap();
    assert!(matches!(
        client.get_player("Player").await,
        Err(Error::Api {
            status: StatusCode::FOUND,
            ..
        })
    ));
    assert!(destination.targets().is_empty());
}

#[tokio::test]
async fn invalid_endpoint_parameters_fail_before_sending_a_request() {
    let server = Server::start(Vec::new()).await;
    let client = server.client();
    for name in ["", " ", ".", "..", "line\nbreak"] {
        assert!(matches!(
            client.get_map(name).await,
            Err(Error::InvalidInput {
                parameter: "name",
                ..
            })
        ));
    }
    assert!(matches!(
        client.get_mods(Some(0)).await,
        Err(Error::InvalidInput {
            parameter: "page",
            ..
        })
    ));
    assert!(matches!(
        client.get_player_matches("Player", Some(-1), None).await,
        Err(Error::InvalidInput {
            parameter: "page",
            ..
        })
    ));
    assert!(matches!(
        client.get_player_matches("Player", None, Some(-1)).await,
        Err(Error::InvalidInput {
            parameter: "season",
            ..
        })
    ));
    assert!(matches!(
        client.get_match(0).await,
        Err(Error::InvalidInput {
            parameter: "match_id",
            ..
        })
    ));
    assert!(matches!(
        client.get_market_skin(-1, None).await,
        Err(Error::InvalidInput {
            parameter: "skin_index",
            ..
        })
    ));
    assert!(matches!(
        client.get_leaderboard(-1, None).await,
        Err(Error::InvalidInput {
            parameter: "region",
            ..
        })
    ));
    assert!(server.targets().is_empty());
}

#[test]
fn invalid_builder_settings_are_rejected_without_exposing_keys() {
    for key in ["", " ", "secret\ninvalid"] {
        let error = Client::new(key).err().unwrap();
        assert!(matches!(
            error,
            Error::InvalidInput {
                parameter: "api_key",
                ..
            }
        ));
        assert!(!error.to_string().contains("secret"));
    }
    for url in [
        "not a URL",
        "ftp://example.com/api",
        "https://example.com/api?query=1",
        "https://example.com/api#fragment",
        "https://name:password@example.com/api",
    ] {
        assert!(matches!(
            Client::builder("test-key").base_url(url).build(),
            Err(Error::InvalidInput {
                parameter: "base_url",
                ..
            })
        ));
    }
    assert!(matches!(
        Client::builder("test-key").timeout(Duration::ZERO).build(),
        Err(Error::InvalidInput { .. })
    ));
    assert!(matches!(
        Client::builder("test-key")
            .connect_timeout(Duration::ZERO)
            .build(),
        Err(Error::InvalidInput { .. })
    ));
}

#[test]
fn error_display_keeps_full_details_available_without_dumping_response_bodies() {
    let error = Error::Decode {
        message: "invalid type".to_owned(),
        field: Some("page".to_owned()),
        body: "large response contents".to_owned(),
    };
    assert!(error.to_string().contains("page"));
    assert!(!error.to_string().contains("large response contents"));
    let error = Error::Api {
        status: StatusCode::BAD_GATEWAY,
        message: "é".repeat(700),
    };
    assert!(error.to_string().ends_with('…'));
    assert!(error.to_string().chars().count() < 600);
    if let Error::Api { message, .. } = error {
        assert_eq!(message.chars().count(), 700);
    }
}
