use std::{thread, time};
use clap::Clap;

#[derive(Clap)]
struct Time {
    #[clap(short, long, default_value = "0")]
    day: u128,
    #[clap(short, long, default_value = "0")]
    hour: u128,
    #[clap(short, long, default_value = "0")]
    minute: u128,
    #[clap(short, long, default_value = "0")]
    second: u128,
}

impl Time {
    fn advance(&mut self) -> bool {
        if self.second != 0 {
            self.second -= 1;
            return true;
        }
        if self.minute != 0 {
            self.minute -= 1;
            return true;
        }
        if self.hour != 0 {
            self.hour -= 1;
            return true;
        }
        if self.day != 0 {
            self.day -= 1;
            return true;
        }
        false
    }
}

impl ToString for Time {
    fn to_string(&self) -> String {
        format!("{}:{}:{}:{}", self.day, self.hour, self.minute, self.second)
    }
}

fn main() {
    let mut time = Time::parse();
    let one_second = time::Duration::from_secs(1);
    while time.advance() {
        // Clear display
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", time.to_string());
        thread::sleep(one_second);
    }
}
