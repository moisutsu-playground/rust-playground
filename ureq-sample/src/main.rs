fn main() -> std::io::Result<()> {
    let url = "https://ja.wikipedia.org/wiki/Rust_(%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E)";
    let resp = ureq::get(url).call().into_string()?;
    println!("{}", resp);
    Ok(())
}
