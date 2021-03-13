use anyhow::Result;
use arxiv::query;

#[tokio::main]
async fn main() -> Result<()> {
    let query = query!(
        search_query = "cat:cs.CL AND submittedDate:[20190101 TO 20190102]",
        sort_by = "submittedDate",
        sort_order = "ascending"
    );
    let arxivs = arxiv::fetch_arxivs(query).await?;
    println!("arxivs length: {}", arxivs.len());
    for i in 0..arxivs.len() {
        println!("{}:", i + 1);
        println!("title: {}", arxivs[i].title);
        println!("published: {}", arxivs[i].published);
        println!("summary: {}", arxivs[i].summary);
        println!("");
    }
    Ok(())
}
