use image::{imageops::FilterType, open, DynamicImage};
use rayon::prelude::*;
use std::{fmt::Debug, fs::read_dir, path::PathBuf};

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

pub struct Resizer<'a> {
    queue: Vec<DynamicImage>,
    config: &'a Config,
}

impl<'a> Resizer<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            queue: Vec::new(),
            config,
        }
    }
    pub fn collect(&mut self) {
        let mut all_dirs: Vec<PathBuf> = Vec::new();
        self.queue = read_dir(self.src())
            .unwrap_or_else(|_| panic!("couldn't read souce directory {:?}", self.src()))
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|f| {
                if let Some(extension) = f.extension() {
                    if let Some(ext) = extension.to_str() {
                        return EXTENSIONS.contains(&ext);
                    }
                };
                if f.is_dir() && self.recursive() {
                    all_dirs.push(f.to_path_buf());
                }
                false
            })
            .map(|f| open(f.as_path()).unwrap_or_else(|_| panic!("Error opening image {:?}", f)))
            .collect::<Vec<DynamicImage>>();
    }

    pub fn resize(&mut self) {
        if self.config.src.is_file() {
            let img =
                open(self.src()).unwrap_or_else(|_| panic!("Error opening image {:?}", self.src()));
            self.resize_file(&img);
        } else {
            self.collect();
            self.resize_all()
        }
    }

    pub fn resize_file(&self, img: &DynamicImage) {
        if img.width() == self.width() && img.height() == self.height() {
            return;
        };

        // only resize if the desired width is different
        if self.ignore_aspect() {
            img.resize_exact(self.width(), self.height(), FilterType::Lanczos3)
                .save(self.src())
                .unwrap_or_else(|_| {
                    panic!(
                        "Error while saving resized image {:?} (ignoring aspect ratio)",
                        self.src()
                    )
                });
        } else {
            img.resize(self.width(), self.height(), FilterType::Lanczos3)
                .save(&self.src())
                .unwrap_or_else(|_| {
                    panic!(
                        "Error while saving resized image {:?} (keeping aspect ratio)",
                        self.src()
                    )
                });
        }
        println!("Resized file {:?}", self.src());
    }

    pub fn resize_all(&self) {
        self.queue.par_iter().for_each(|f| {
            self.resize_file(f);
        });
    }

    const fn src(&self) -> &PathBuf {
        &self.config.src
    }

    const fn width(&self) -> u32 {
        self.config.width
    }
    const fn height(&self) -> u32 {
        self.config.height
    }
    const fn ignore_aspect(&self) -> bool {
        self.config.ignore_aspect
    }
    const fn recursive(&self) -> bool {
        self.config.recursive
    }
}
