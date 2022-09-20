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
    /// Creates a new [`Resizer`].
    pub fn new(config: &'a Config) -> Self {
        Self {
            queue: Vec::new(),
            config,
        }
    }

    /// collects all image paths into a vec.
    ///
    /// # Panics
    ///
    /// Panics if a path couldn't be read
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
        if !images.is_empty() {
            self.queue.append(&mut images);
        }

        if self.recursive() {
            for subpath in all_dirs {
                self.collect(&subpath);
            }
        }
    }

    /// Resizes the source image or all images of a directory
    pub fn resize(&mut self) {
        if self.src().is_file() {
            self.resize_file(self.src());
        } else {
            self.collect(&self.src().clone());
            self.resize_all()
        }
    }

    /// resizes a single file respecting given options.
    ///
    /// # Panics
    ///
    /// Panics if image couldn't be opended or resized.
    pub fn resize_file(&self, img_path: &Path) {
        let img = open(img_path).unwrap_or_else(|_| panic!("Error opening image {img_path:?}"));
        if img.width() == self.width() {
            return;
        };

        // only resize if the desired width is different
        if self.ignore_aspect() {
            println!("Resized image {img_path:?}");
            img.resize_exact(self.width(), self.height(), FilterType::Lanczos3)
                .save(img_path)
                .unwrap_or_else(|_| {
                    panic!("Error while saving resized image {img_path:?} (ignoring aspect ratio)")
                });
        } else {
            img.resize(self.width(), self.height(), FilterType::Lanczos3)
                .save(img_path)
                .unwrap_or_else(|_| {
                    panic!("Error while saving resized image {img_path:?} (keeping aspect ratio)",)
                });
        }
        println!("Resized image {img_path:?}");
    }

    /// Resizes all images in the queue of the [`Resizer`]
    pub fn resize_all(&self) {
        self.queue.par_iter().for_each(|f| self.resize_file(f));
    }

    /// Returns a reference to the src of the [`Resizer`].
    const fn src(&self) -> &PathBuf {
        &self.config.src
    }

    /// Returns the desired width of the [`Resizer`].
    const fn width(&self) -> u32 {
        self.config.width
    }

    /// Returns the desired height of the [`Resizer`].
    const fn height(&self) -> u32 {
        self.config.height
    }

    /// Returns the ignore aspect option of the [`Resizer`].
    const fn ignore_aspect(&self) -> bool {
        self.config.ignore_aspect
    }

    /// Returns the recursive option of this [`Resizer`].
    const fn recursive(&self) -> bool {
        self.config.recursive
    }
}
