use clap::Parser;

use rsize::{resize, Config};

fn main() -> Result<(), String> {
    let config = Config::parse();
    resize(config);
    Ok(())
}
