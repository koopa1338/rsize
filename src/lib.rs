use image::{imageops::FilterType, open};
// use rayon::prelude::*;
use std::{
    fmt::Debug,
    fs::read_dir,
    path::{Path, PathBuf},
};

use clap::Parser;

const EXTENSIONS: [&str; 2] = ["png", "jpg"];

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

fn resize_file(path: impl AsRef<Path> + Debug, width: u32, height: u32, ignore_aspect: bool) {
    println!("check file: {path:?}");
    let img = open(&path).unwrap_or_else(|_| panic!("Error opening image {path:?}"));
    let dim_w = img.width();

    // only resize if the desired width is different
    if dim_w != width {
        if ignore_aspect {
            img.resize_exact(width, height, FilterType::Lanczos3)
                .save(&path)
                .unwrap_or_else(|_| {
                    panic!("Error while saving resized image {path:?} (ignoring aspect ratio)")
                });
        } else {
            img.resize(width, height, FilterType::Lanczos3)
                .save(&path)
                .unwrap_or_else(|_| {
                    panic!("Error while saving resized image {path:?} (keeping aspect ratio)")
                });
        }
        println!("Resized file {path:?}");
    }
}

pub fn resize(config: Config) {
    if config.src.is_file() {
        resize_file(
            &config.src,
            config.width,
            config.height,
            config.ignore_aspect,
        );
    }
    if config.src.is_dir() {
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
    filepath: impl AsRef<Path> + Debug,
    width: u32,
    height: u32,
    ignore_aspect: bool,
    recursive: bool,
) {
    //get all files as PathBuf in a vec
    let mut all_dirs: Vec<PathBuf> = Vec::new();
    let all_files: Vec<PathBuf> = read_dir(&filepath)
        .unwrap_or_else(|_| panic!("couldn't read souce directory {filepath:?}"))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|f| {
            if let Some(extension) = f.extension() {
                if let Some(ext) = extension.to_str() {
                    return EXTENSIONS.contains(&ext);
                }
            };
            if f.is_dir() && recursive {
                all_dirs.push(f.to_path_buf());
            }
            false
        })
        .collect();

    // TODO: debug why rayons par_iter doesn't work here. It gets stuck after spawning 10 to 13
    // threads.
    all_files.iter().for_each(|p| {
        if p.is_file() {
            resize_file(p, width, height, ignore_aspect);
        }
    });
    if recursive {
        // TODO: if par iter works try to do this also in parallel if possible
        for filepath in all_dirs {
            resize_all(filepath, width, height, ignore_aspect, recursive);
        }
    }
}
