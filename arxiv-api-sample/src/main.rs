use anyhow::{anyhow, Result};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
struct Arxiv {
    id: String,
    title: String,
    summary: String,
}

impl Arxiv {
    fn new() -> Self {
        Arxiv {
            id: "".to_string(),
            title: "".to_string(),
            summary: "".to_string(),
        }
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let base_url = "http://export.arxiv.org/api/query?";
    let url = format!("{}{}", base_url, "search_query=cat:cs.CL");
    let mut response = surf::get(url).await.map_err(|err| anyhow!(err))?;
    let body = response.body_string().await.map_err(|err| anyhow!(err))?;

    println!("{}", body);

    let mut parser = EventReader::from_str(&body);
    let mut arxiv = Arxiv::new();
    let mut arxivs = Vec::new();

    'outer: loop {
        if let Ok(element) = parser.next() {
            match element {
                XmlEvent::StartElement { name, .. } => {
                    match &name.local_name[..] {
                        "entry" => {
                            arxiv = Arxiv::new();
                        }
                        "id" => {
                            if let Ok(XmlEvent::Characters(id)) = parser.next() {
                                arxiv.id = id;
                            }
                        }
                        "title" => {
                            if let Ok(XmlEvent::Characters(title)) = parser.next() {
                                arxiv.title = title;
                            }
                        }
                        "summary" => {
                            if let Ok(XmlEvent::Characters(summary)) = parser.next() {
                                arxiv.summary = summary;
                            }
                        }
                        _ => ()
                    }
                }
                XmlEvent::EndElement { name } => {
                    match &name.local_name[..] {
                        "entry" => {
                            arxivs.push(arxiv.clone());
                        }
                        "feed" => {
                            break 'outer;
                        }
                        _ => ()
                    }
                }
                _ => {
                    ()
                }
            }
        } else {
            break;
        }
    }
    for i in 0..arxivs.len() {
        println!("{:?}", arxivs[i]);
    }
    Ok(())
}
