extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{}", ctx.get_contents().unwrap());
    ctx.set_contents("sample".to_string()).unwrap();
}
