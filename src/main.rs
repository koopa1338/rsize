use clap::{App, Arg};
use std::path::PathBuf;
use threadpool::ThreadPool;
use glob::glob;

use image::{imageops::FilterType, open};

fn resize_pattern(pattern: &str, width: u32, height: u32, filepath: &PathBuf) {
    let pool = ThreadPool::new(30);
    for entry in glob(&pattern).unwrap() {
            let tmpfile = entry.unwrap();
            let img = open(&tmpfile).unwrap();
            let (dim_w, _) = img.to_rgb16().dimensions();
            let fp = filepath.to_owned();
            // only resize if the desired width is different
            if dim_w != width {
                pool.execute(move || {
                    let resized_img = img.resize(width, height, FilterType::Gaussian);
                    resized_img.save(fp.as_path().join(&tmpfile)).unwrap();
                    println!("Resized file {:?}", tmpfile);
                });
            }
    }
}

fn resize(filepath: PathBuf, width: u32, height: u32) {
    if filepath.is_file() {
        let img = open(filepath.as_path()).unwrap();
        let resized_img = img.resize(width, height, FilterType::Lanczos3);
        resized_img.save(filepath.as_path()).unwrap();
        println!("Resized file {:?}", filepath);
    } else {
        let jpgs = format!("{}{}", filepath.as_path().to_str().unwrap(), "*.jpg");
        let pngs = format!("{}{}", filepath.as_path().to_str().unwrap(), "*.png");
        resize_pattern(&jpgs, width, height, &filepath);
        resize_pattern(&pngs, width, height, &filepath);
    }
}

fn main() {
    let matches = App::new("resizer")
        .version("0.1")
        .author("koopa1338 <koopa1338@yandex.com>")
        .about("resizes images")
        .arg(Arg::with_name("src")
            .short("s")
            .long("src")
            .value_name("FILEs")
            .help("Resizes a single file or multiple by applying a directory")
            .takes_value(true)
            .default_value("./"))
        .arg(Arg::with_name("width")
            .short("w")
            .takes_value(true)
            .help("desired width")
            .default_value("1920"))
        .arg(Arg::with_name("height")
            .short("h")
            .takes_value(true)
            .help("desired height")
            .default_value("1080"))
        .get_matches();


        let filepath = PathBuf::from(matches.value_of("src").unwrap());
        let width: u32 = matches.value_of("width").unwrap().parse::<u32>().unwrap();
        let height: u32 = matches.value_of("height").unwrap().parse::<u32>().unwrap();

        resize(filepath, width, height);
}
