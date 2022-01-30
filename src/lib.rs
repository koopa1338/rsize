use image::{imageops::FilterType, open};
use rayon::prelude::*;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

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
    #[clap(long, short)]
    recursive: bool,
}

fn resize_file(
    path: impl AsRef<Path> + std::fmt::Debug,
    width: u32,
    height: u32,
    ignore_aspect: bool,
) {
    let img = open(&path).unwrap_or_else(|_| panic!("Error while saving resized image {:?}", path));
    let (dim_w, _) = img.to_rgb16().dimensions();

    // only resize if the desired width is different
    if dim_w != width {
        if ignore_aspect {
            img.resize_exact(width, height, FilterType::Lanczos3)
                .save(&path)
                .unwrap_or_else(|_| panic!("Error while saving resized image {:?}", path));
        } else {
            img.resize(width, height, FilterType::Lanczos3)
                .save(&path)
                .unwrap_or_else(|_| panic!("Error while saving resized image {:?}", path));
        }
        println!("Resized file {:?}", path);
    }
}

pub fn resize(config: Config) {
    if config.src.is_file() {
        let filepath = config.src.as_path();
        resize_file(filepath, config.width, config.height, config.ignore_aspect);
    } else {
        resize_all(
            config.src,
            config.width,
            config.height,
            config.ignore_aspect,
            config.recursive,
        );
    }
}

fn resize_all(
    filepath: impl AsRef<Path> + std::fmt::Debug,
    width: u32,
    height: u32,
    ignore_aspect: bool,
    recursive: bool,
) {
    //get all files as PathBuf in a vec
    let all_files = read_dir(&filepath)
        .unwrap_or_else(|_| panic!("couldn't read souce directory {:?}", filepath))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|f| {
            if let Some(extension) = f.as_path().extension() {
                if let Some(ext) = extension.to_str() {
                    return EXTENSIONS.contains(&ext);
                }
            }
            // do not filter directories as we might resize recursively
            f.is_dir()
        })
        .collect::<Vec<PathBuf>>();

    all_files.into_par_iter().for_each(|p| {
        if p.is_file() {
            resize_file(p.as_path(), width, height, ignore_aspect);
        } else if recursive {
            resize_all(p.as_path(), width, height, ignore_aspect, recursive);
        }
    });
}
