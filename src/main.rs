use clap::{App, Arg};
use std::{ffi::OsStr, path::PathBuf};
use rsize::resize;

mod conferror;
use conferror::ConfigErr;

struct Config {
    src: PathBuf,
    ignore_aspect: bool,
    width: u32,
    height: u32,
}

fn get_config() -> Result<Config, ConfigErr> {
    let current_dir = OsStr::new(".").to_str().unwrap();
    let matches = App::new("rsize")
        .version("0.1.0")
        .author("koopa1338 <koopa1338@yandex.com>")
        .about("resizes images")
        .arg(
            Arg::with_name("src")
                .short("s")
                .long("src")
                .value_name("FILEs")
                .help("Resizes a single file or multiple by applying a directory")
                .takes_value(true)
                .default_value(current_dir),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .takes_value(true)
                .help("desired width")
                .default_value("1920"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .takes_value(true)
                .help("desired height")
                .default_value("1080"),
        )
        .arg(
            Arg::with_name("ignore-aspect")
                .short("i")
                .takes_value(false)
                .help("ignore the aspect ratio and resize exactly to the width and height"),
        )
        .get_matches();

    let src = PathBuf::from(matches.value_of("src").ok_or(current_dir).unwrap());
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
