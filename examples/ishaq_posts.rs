mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = common::client_and_args(
        0,
        "Usage: cargo run --example ishaq_posts -- [api-key] [--debug]",
    )?;
    let name = "IshaqAyubi";
    let posts = client
        .get_player_posts(name, Some(1))
        .await?
        .posts_posts
        .unwrap_or_default();
    if posts.is_empty() {
        println!("No posts found for {name}.");
    }
    for post in posts {
        println!("Date: {}", post.post_date);
        println!("Votes: {}", post.post_votes);
        println!("Replies: {}", post.post_comment_count);
        println!("\n{}\n", post.post_text);
    }
    Ok(())
}
