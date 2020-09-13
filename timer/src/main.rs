use clap::Clap;
use std::{fmt, thread, time};

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
        if self.day == 0 && self.hour == 0 && self.minute == 0 && self.second == 0 {
            false
        } else {
            if self.second != 0 {
                self.second -= 1;
                return true;
            } else {
                self.second = 59;
            }

            if self.minute != 0 {
                self.minute -= 1;
                return true;
            } else {
                self.minute = 59;
            }

            if self.hour != 0 {
                self.hour -= 1;
                return true;
            } else {
                self.hour = 59;
            }

            self.day -= 1;
            true
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}d:{}h:{}m:{}s",
            self.day, self.hour, self.minute, self.second
        )
    }
}

fn main() {
    let mut time = Time::parse();
    let one_second = time::Duration::from_secs(1);
    while time.advance() {
        // Clear display
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", time);
        thread::sleep(one_second);
    }
}
