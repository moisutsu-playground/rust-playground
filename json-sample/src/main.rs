use json_sample::model::JsonSample;
use std::fs;

fn main() -> anyhow::Result<()> {
    let file = fs::read_to_string("sample.json")?;
    let sample: JsonSample = serde_json::from_str(&file)?;
    println!("{:?}", sample.address);
    Ok(())
}
