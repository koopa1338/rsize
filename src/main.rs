use clap::{App, Arg};
use image::{imageops::FilterType, open};
use rayon::prelude::*;
use std::{ffi::OsStr, fs::read_dir, path::PathBuf};

mod conferror;
use conferror::ConfigErr;

struct Config {
    src: PathBuf,
    ignore_aspect: bool,
    width: u32,
    height: u32,
}

fn get_config() -> Result<Config, ConfigErr> {
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
                .default_value("./"),
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

    let src = PathBuf::from(matches.value_of("src").ok_or(ConfigErr::EmptyVal)?);
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

fn resize(filepath: PathBuf, width: u32, height: u32, ignore_aspect: bool) {
    if filepath.is_file() {
        let img = open(filepath.as_path()).unwrap();
        let resized_img = img.resize(width, height, FilterType::Lanczos3);
        resized_img.save(filepath.as_path()).unwrap();
        println!("Resized file {:?}", filepath);
    } else {
        //get all files as PathBuf in a vec
        let all_files = read_dir(filepath.as_path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect::<Vec<PathBuf>>();

        let png = OsStr::new("png");
        let jpg = OsStr::new("jpg");

        all_files.into_par_iter().for_each(|p| {
            if p.is_file() {
                if let Some(ext) = p.as_path().extension() {
                    if ext.eq(png) || ext.eq(jpg) {
                        let img_path = filepath.to_owned().as_path().join(&p.as_path());
                        let img = open(&p.as_path()).unwrap();
                        let (dim_w, _) = img.to_rgb16().dimensions();

                        // only resize if the desired width is different
                        if dim_w != width {
                            if ignore_aspect {
                                img.resize_exact(width, height, FilterType::Lanczos3)
                                    .save(&img_path)
                                    .unwrap();
                            } else {
                                img.resize(width, height, FilterType::Lanczos3)
                                    .save(&img_path)
                                    .unwrap();
                            }
                            println!("Resized file {:?}", img_path);
                        }
                    }
                }
            }
        });
    }
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
