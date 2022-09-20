use image::{imageops::FilterType, open};
use rayon::prelude::*;
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

pub struct Resizer<'a> {
    queue: Vec<PathBuf>,
    config: &'a Config,
}

impl<'a> Resizer<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            queue: Vec::new(),
            config,
        }
    }
    pub fn collect(&mut self, path: &Path) {
        let mut all_dirs: Vec<PathBuf> = Vec::new();
        let mut images = read_dir(path)
            .unwrap_or_else(|_| panic!("couldn't read souce directory {path:?}"))
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
            .collect::<Vec<PathBuf>>();
        self.queue.append(&mut images);
        if self.recursive() {
            for subpath in all_dirs {
                self.collect(&subpath);
            }
        }
    }

    pub fn resize(&mut self) {
        if self.src().is_file() {
            self.resize_file(self.src());
        } else {
            self.collect(&self.src().clone());
            self.resize_all()
        }
    }

    pub fn resize_file(&self, img_path: &Path) {
        let img = open(img_path).unwrap_or_else(|_| panic!("Error opening image {img_path:?}"));
        println!("Checked file {img_path:?}");
        if img.width() == self.width() {
            return;
        };

        // only resize if the desired width is different
        if self.ignore_aspect() {
            img.resize_exact(self.width(), self.height(), FilterType::Lanczos3)
                .save(img_path)
                .unwrap_or_else(|_| {
                    panic!(
                        "Error while saving resized image {:?} (ignoring aspect ratio)",
                        self.src()
                    )
                });
        } else {
            img.resize(self.width(), self.height(), FilterType::Lanczos3)
                .save(img_path)
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
        self.queue.par_iter().for_each(|f| self.resize_file(f));
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
