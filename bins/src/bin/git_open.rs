use anyhow::Result;
use std::process::Command;
use std::os::unix::process::CommandExt;

#[allow(non_camel_case_types)]
trait Vec_u8_ToString {
    fn to_string(&self) -> String;
}

impl Vec_u8_ToString for Vec<u8> {
    fn to_string(&self) -> String {
        String::from_utf8(self.clone()).unwrap()
    }
}

fn main() -> Result<()> {
    let output = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .output()?
        .stdout
        .to_string();
    let url = format!(
        "https://{}",
        output.split("@").collect::<Vec<_>>()[1].replacen(":", "/", 1)
    );
    Command::new("open")
        .arg(url)
        .exec();
    Ok(())
}
