use std::process::{Command, Stdio};

fn main() -> Result<(), std::io::Error> {
    execute_command_like_exec("python", &["-c", "print('Hello World')"])?;
    Ok(())
}

fn execute_command_like_exec(program: &str, args: &[&str]) -> Result<(), std::io::Error> {
    Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;
    Ok(())
}
