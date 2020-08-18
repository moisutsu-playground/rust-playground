use anyhow::{anyhow, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let base_url = "http://export.arxiv.org/api/query?";
    let url = format!("{}{}", base_url, "search_query=cat:cs.CL");
    let mut response = surf::get(url).await.map_err(|err| anyhow!(err))?;
    let body = response.body_string().await.map_err(|err| anyhow!(err))?;
    println!("{}", body);
    Ok(())
}
