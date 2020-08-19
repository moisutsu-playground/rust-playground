use anyhow::{anyhow, Result};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
struct Arxiv {
    id: String,
    title: String,
    summary: String,
    // authors: Vec<String>,
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
        match parser.next()? {
            XmlEvent::StartElement { name, .. } => match &name.local_name[..] {
                "entry" => {
                    arxiv = Arxiv::new();
                }
                "id" => {
                    arxiv.id = if let XmlEvent::Characters(id) = parser.next()? {
                        id
                    } else {
                        arxiv.id
                    };
                }
                "title" => {
                    arxiv.title = if let XmlEvent::Characters(title) = parser.next()? {
                        title
                    } else {
                        arxiv.title
                    };
                }
                "summary" => {
                    arxiv.summary = if let XmlEvent::Characters(summary) = parser.next()? {
                        summary
                    } else {
                        arxiv.summary
                    };
                }
                _ => (),
            },
            XmlEvent::EndElement { name } => match &name.local_name[..] {
                "entry" => {
                    arxivs.push(arxiv.clone());
                }
                "feed" => {
                    break 'outer;
                }
                _ => (),
            },
            _ => (),
        }
    }
    for i in 0..arxivs.len() {
        println!("{:?}", arxivs[i]);
    }
    Ok(())
}
