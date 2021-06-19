use image::{imageops::FilterType, open};
use std::{ffi::OsStr, fs::read_dir, path::PathBuf};
use rayon::prelude::*;

pub fn resize(filepath: PathBuf, width: u32, height: u32, ignore_aspect: bool) {
    if filepath.is_file() {
        let img = open(filepath.as_path()).unwrap();
        let resized_img = img.resize(width, height, FilterType::Lanczos3);
        resized_img.save(filepath.as_path()).unwrap();
        println!("Resized file {:?}", filepath);
    } else {
        resize_all(filepath, width, height, ignore_aspect);
    }
}

fn resize_all(filepath: PathBuf, width: u32, height: u32, ignore_aspect: bool) {
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
