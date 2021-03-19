use anyhow::Result;
use dotenv::dotenv;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let api = kuon::TwitterAPI::new_using_env().await?;
    let tweet_result = api.tweet("Test Tweet").await?;
    let in_reply_to_status_id = &tweet_result.id.to_string()[..];
    let params: HashMap<&str, &str> = vec![
        ("in_reply_to_status_id", in_reply_to_status_id),
        ("auto_populate_reply_metadata", "true"),
    ]
    .into_iter()
    .collect();
    api.tweet_with_params("Reply", &params).await?;
    Ok(())
}
