use std::io::Result;
use std::net::TcpListener;

fn available_port() -> Result<u16> {
    TcpListener::bind("localhost:0")?.local_addr().map(|addr| addr.port())
}

fn main() -> Result<()> {
    println!("{}", available_port()?);
    Ok(())
}
