use std::process::{Command, Stdio};

fn main() {
    let mut cmd = Command::new("fd")
        .arg("python")
        .arg("/")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    // It's streaming here

    cmd.wait().unwrap();
    println!("Exited with status {:?}", status);
}
