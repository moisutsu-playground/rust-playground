use scraper::{Html, Selector};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let url = "https://ja.wikipedia.org/wiki/%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E%E4%B8%80%E8%A6%A7";
    let body = get_html_body(url).await?;
    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("#mw-content-text > div.mw-parser-output > ul > li > a").unwrap();
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
