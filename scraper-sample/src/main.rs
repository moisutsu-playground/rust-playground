#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let url = "https://www.google.com/";
    println!("{}", get_html_body(url).await?);
    Ok(())
}

async fn get_html_body(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut response = surf::get(url).await?;
    let body = response.body_string().await?;
    Ok(body)
}
