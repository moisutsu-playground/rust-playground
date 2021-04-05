use log::{debug, error, info, warn};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    debug!("db");
    info!("Hello");
    warn!("ahaha");
    error!("noooo");
}
