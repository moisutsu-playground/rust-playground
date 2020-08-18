use anyhow::{anyhow, Result};
use xml::reader::{EventReader, XmlEvent};

#[async_std::main]
async fn main() -> Result<()> {
    let base_url = "http://export.arxiv.org/api/query?";
    let url = format!("{}{}", base_url, "search_query=cat:cs.CL");
    let mut response = surf::get(url).await.map_err(|err| anyhow!(err))?;
    let body = response.body_string().await.map_err(|err| anyhow!(err))?;

    let mut summarys = Vec::new();

    let parser = EventReader::from_str(&body);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::Characters(data)) => {
                println!("{}+{}", indent(depth), data);
                summarys.push(data);
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}
