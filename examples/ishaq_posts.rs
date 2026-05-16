use krunker_rs::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable or command line
    let api_key = env::var("KRUNKER_API_KEY").unwrap_or_else(|_| {
        env::args().nth(1).expect("Please provide an API key as the first argument or set KRUNKER_API_KEY environment variable")
    });

    // Initialize the Krunker client
    let client = Client::new(api_key)?;

    let target_player = "IshaqAyubi";
    println!("Fetching social posts for: {}...", target_player);

    // Fetch the first page of posts
    match client.get_player_posts(target_player, Some(1)).await {
        Ok(response) => {
            if let Some(posts) = response.posts_posts {
                if posts.is_empty() {
                    println!("No posts found for {}.", target_player);
                } else {
                    println!("Found {} posts:\n", posts.len());
                    for post in posts {
                        println!("----------------------------------------");
                        println!("Date:    {}", post.post_date);
                        println!("Votes:   {}", post.post_votes);
                        println!("Replies: {}", post.post_comment_count);
                        println!("\nContent:\n{}", post.post_text);
                        println!("----------------------------------------\n");
                    }
                }
            } else {
                println!("No posts data returned.");
            }
        }
        Err(e) => {
            eprintln!("Error fetching posts: {}", e);
        }
    }

    Ok(())
}
