use image::{imageops::FilterType, open};
use rayon::prelude::*;
use std::{fs::read_dir, path::PathBuf};

use clap::Parser;

const EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(long, short, default_value = "./", parse(from_os_str))]
    src: PathBuf,
    #[clap(long, short)]
    ignore_aspect: bool,
    #[clap(long, default_value_t = 1920u32)]
    width: u32,
    #[clap(long, default_value_t = 1080u32)]
    height: u32,
}

pub fn resize(config: Config) {
    if config.src.is_file() {
        let filepath = config.src.as_path();
        let img = open(filepath).unwrap_or_else(|_| {
            panic!(
                "Error while opening image file {:?}. Maybe corrupted?",
                filepath
            )
        });
        let resized_img = img.resize(config.width, config.height, FilterType::Lanczos3);
        resized_img
            .save(filepath)
            .unwrap_or_else(|_| panic!("Error while saving resized image {:?}", filepath));
        println!("Resized file {:?}", filepath);
    } else {
        resize_all(config);
    }
}

fn resize_all(config: Config) {
    let filepath = config.src.as_path();
    //get all files as PathBuf in a vec
    let all_files = read_dir(filepath)
        .unwrap_or_else(|_| panic!("couldn't read souce directory {:?}", filepath))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|f| {
            if let Some(extension) = f.as_path().extension() {
                if let Some(ext) = extension.to_str() {
                    return EXTENSIONS.contains(&ext);
                }
            }
            false
        })
        .collect::<Vec<PathBuf>>();

    all_files.into_par_iter().for_each(|p| {
        if p.is_file() {
            let img_path = filepath.join(&p.as_path());
            let img = open(&p.as_path())
                .unwrap_or_else(|_| panic!("Error while saving resized image {:?}", filepath));
            let (dim_w, _) = img.to_rgb16().dimensions();

            // only resize if the desired width is different
            if dim_w != config.width {
                if config.ignore_aspect {
                    img.resize_exact(config.width, config.height, FilterType::Lanczos3)
                        .save(&img_path)
                        .unwrap_or_else(|_| {
                            panic!("Error while saving resized image {:?}", filepath)
                        });
                } else {
                    img.resize(config.width, config.height, FilterType::Lanczos3)
                        .save(&img_path)
                        .unwrap_or_else(|_| {
                            panic!("Error while saving resized image {:?}", filepath)
                        });
                }
                println!("Resized file {:?}", img_path);
            }
        }
    });
}
