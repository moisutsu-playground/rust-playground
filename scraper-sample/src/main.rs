use scraper::{Html, Selector};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let url = "https://www.aclweb.org/anthology/events/acl-2020/";
    let body = get_html_body(url).await?;
    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("#\\32 020-acl-main > p > span:nth-child(2) > strong > a").unwrap();
    for element in fragment.select(&selector) {
        println!("{:?}", element.inner_html());
    }
    Ok(())
}

async fn get_html_body(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut response = surf::get(url).await?;
    let body = response.body_string().await?;
    Ok(body)
}
