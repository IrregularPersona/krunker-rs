mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, args) = common::client_and_args(
        1,
        "Usage: cargo run --example fetch_post -- [api-key] <player-name> [--debug]",
    )?;
    let posts = client
        .get_player_posts(&args[0], Some(1))
        .await?
        .posts_posts
        .unwrap_or_default();
    if posts.is_empty() {
        println!("No posts found for {}.", args[0]);
    }
    for post in posts {
        println!("Date: {}", post.post_date);
        println!("Text: {}", post.post_text);
        println!("Votes: {}", post.post_votes);
        println!("Comments: {}\n", post.post_comment_count);
    }
    Ok(())
}
