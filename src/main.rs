use clap::{App, Arg};
use glob::glob;
use image::{imageops::FilterType, open};
use rayon::prelude::*;
use std::path::PathBuf;

fn resize(filepath: PathBuf, width: u32, height: u32, ignore_aspect: bool) {
    if filepath.is_file() {
        let img = open(filepath.as_path()).unwrap();
        let resized_img = img.resize(width, height, FilterType::Lanczos3);
        resized_img.save(filepath.as_path()).unwrap();
        println!("Resized file {:?}", filepath);
    } else {
        let mut images: Vec<PathBuf> = Vec::new();
        let pngs = glob(&format!(
            "{}{}",
            filepath.as_path().to_str().unwrap(),
            "*.png"
        ))
        .unwrap();

        for p in pngs {
            images.push(p.unwrap());
        }
        let jpgs = glob(&format!(
            "{}{}",
            filepath.as_path().to_str().unwrap(),
            "*.jpg"
        ))
        .unwrap();
        for j in jpgs {
            images.push(j.unwrap());
        }

        // resize images
        images.into_par_iter().for_each(|p| {
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
        });
    }
}

fn main() {
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
                .help("ignore the aspect ratio and resice exactly to the width and height"),
        )
        .get_matches();

    let filepath = PathBuf::from(matches.value_of("src").unwrap());
    let ignore_aspect: bool = matches.is_present("ignore-aspect");
    let width: u32 = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    let height: u32 = matches.value_of("height").unwrap().parse::<u32>().unwrap();

    resize(filepath, width, height, ignore_aspect);
}
