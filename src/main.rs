#[macro_use]
extern crate clap;

use clap::App;
use rsize::resize;
use std::{env::current_dir, path::PathBuf};

mod conferror;
use conferror::ConfigErr;

struct Config {
    src: PathBuf,
    ignore_aspect: bool,
    width: u32,
    height: u32,
}

fn get_config() -> Result<Config, ConfigErr> {
    let current_dir = current_dir().unwrap();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let src = PathBuf::from(matches.value_of("src").ok_or(&current_dir).unwrap());
    let ignore_aspect: bool = matches.is_present("ignore-aspect");
    let width: u32 = matches
        .value_of("width")
        .ok_or(ConfigErr::EmptyVal)?
        .parse::<u32>()?;
    let height: u32 = matches
        .value_of("height")
        .ok_or(ConfigErr::EmptyVal)?
        .parse::<u32>()?;

    Ok(Config {
        src,
        ignore_aspect,
        width,
        height,
    })
}

fn main() -> Result<(), ConfigErr> {
    let config = get_config()?;
    resize(
        config.src,
        config.width,
        config.height,
        config.ignore_aspect,
    );
    Ok(())
}
