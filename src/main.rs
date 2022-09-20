use clap::Parser;

use rsize::{Config, Resizer};

fn main() {
    let config = Config::parse();
    let mut queue = Resizer::new(&config);
    queue.resize();
}
