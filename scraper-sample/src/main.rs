use scraper::{Html, Selector};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let url = "https://ja.wikipedia.org/wiki/%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E%E4%B8%80%E8%A6%A7";
    let selector = "#mw-content-text > div.mw-parser-output > ul > li > a";
    let elements = fetch_html_element_by_url_and_selector(url, selector).await?;
    println!("{}", elements.join("\n"));
    Ok(())
}

async fn fetch_html_element_by_url_and_selector(
    url: &str,
    selector: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let body = fetch_html_body(url).await?;
    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse(selector).unwrap();

    Ok(fragment
        .select(&selector)
        .map(|element| element.inner_html())
        .collect::<Vec<String>>())
}

async fn fetch_html_body(
    url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut response = surf::get(url).await?;
    let body = response.body_string().await?;
    Ok(body)
}
