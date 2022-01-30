use clap::Parser;

use rsize::{resize, Config};

fn main() {
    let config = Config::parse();
    resize(config);
}
