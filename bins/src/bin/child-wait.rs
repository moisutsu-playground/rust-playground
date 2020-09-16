use anyhow::Result;

trait Arrayu8ToString {
    fn to_string(&self) -> String;
}

impl Arrayu8ToString for [u8] {
    fn to_string(&self) -> String {
        let ret = String::from_utf8(self.to_vec()).unwrap();
        ret
    }
}

fn main() -> Result<()> {
    std::process::Command::new("sleep")
        .arg("5")
        .spawn()?
        .wait()?;
    let output = std::process::Command::new("echo")
        .arg("3")
        .spawn()?
        .wait_with_output()?
        .stdout
        .to_string();
    println!("{}", output);
    Ok(())
}
