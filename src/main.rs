use clap::Parser;
use std::path::PathBuf;

use rsize::resize;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
struct Config {
    #[clap(long, short, default_value = "./", parse(from_os_str))]
    src: PathBuf,
    #[clap(long, short)]
    ignore_aspect: bool,
    #[clap(long, default_value_t = 1920u32)]
    width: u32,
    #[clap(long, default_value_t = 1080u32)]
    height: u32,
}

fn main() -> Result<(), String> {
    let config = Config::parse();
    resize(
        config.src,
        config.width,
        config.height,
        config.ignore_aspect,
    );
    Ok(())
}
