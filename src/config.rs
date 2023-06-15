use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(long, short, default_value = "./")]
    pub src: PathBuf,
    #[clap(long, short)]
    pub ignore_aspect: bool,
    #[clap(long, default_value_t = 1920u32)]
    pub width: u32,
    #[clap(long, default_value_t = 1080u32)]
    pub height: u32,
    #[clap(long, short)]
    pub recursive: bool,
}
