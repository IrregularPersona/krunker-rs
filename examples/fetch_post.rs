use krunker_rs::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let pos_args: Vec<_> = args.iter().filter(|arg| !arg.starts_with("--")).collect();
    match pos_args.as_slice() {
        [_, api_key, target_player] => {
            let client = Client::new(api_key.to_string()).expect("Failed to create client");

            match client.get_player_posts(&target_player, Some(1)).await {
                Ok(content) => {
                    for post in content.posts_posts {
                        println!("Date: {}", post.post_date);
                        println!("Text: {}", post.post_text);
                        println!("Votes: {}", post.post_votes);
                        println!("Comments: {}", post.post_comment_count);
                        println!();
                    }
                }
                Err(err) => {
                    println!("Error: {}", err)
                }
            }
        }

        _ => {
            println!("usage: cargo run --example fetch_post <api-key> <player-name>");
        }
    }
}
