use image::{imageops::FilterType, open};
use rayon::prelude::*;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub mod config;
use config::Config;

const EXTENSIONS: [&str; 2] = ["png", "jpg"];

pub struct Resizer<'a> {
    queue: Vec<PathBuf>,
    config: &'a Config,
}

impl<'a> Resizer<'a> {
    /// Creates a new [`Resizer`].
    #[must_use]
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
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|f| {
                if let Some(extension) = f.extension() {
                    if let Some(ext) = extension.to_str() {
                        return EXTENSIONS.contains(&ext);
                    }
                };
                if f.is_dir() && self.recursive() {
                    all_dirs.push(f.clone());
                }
                false
            })
            .collect::<Vec<PathBuf>>();
        if !images.is_empty() {
            self.queue.append(&mut images);
        }

        self.collect_recursive(all_dirs);
    }

    fn collect_recursive(&mut self, all_dirs: Vec<PathBuf>) {
        if self.recursive() {
            for subpath in all_dirs {
                self.collect(&subpath);
            }
        }
    }

    /// Resizes the source image or all images of a directory
    pub fn resize(&mut self) {
        let src = self.src();
        if src.is_file() {
            self.resize_file(src);
        } else {
            self.collect(&src.clone());
            self.resize_all();
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
            self.resize_exact(&img, img_path);
        } else {
            self.resize_ratio(&img, img_path);
        }
    }

    #[doc(hidden)]
    fn resize_ratio(&self, img: &image::DynamicImage, img_path: &Path) {
        img.resize(self.width(), self.height(), FilterType::Lanczos3)
            .save(img_path)
            .unwrap_or_else(|_| {
                panic!("Error while saving resized image {img_path:?} (keeping aspect ratio)",)
            });
        println!("Resized image {img_path:?}");
    }

    #[doc(hidden)]
    fn resize_exact(&self, img: &image::DynamicImage, img_path: &Path) {
        img.resize_exact(self.width(), self.height(), FilterType::Lanczos3)
            .save(img_path)
            .unwrap_or_else(|_| {
                panic!("Error while saving resized image {img_path:?} (ignoring aspect ratio)")
            });
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
