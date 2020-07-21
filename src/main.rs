#![allow(non_snake_case, unused_imports, unused_assignments, unused_variables)]

mod interfaces;
mod schemas;

use rayon::prelude::*;
use schemas::desc_file::DescFile;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // Generate json description file
    // let mut package_info: DescFile = DescFile::new();
    // package_info.import("desc").export().unwrap_or(());
    let mut all_file: Vec<String> = Vec::new();
    let start = Instant::now();
    let paths = fs::read_dir("/home/brilliant/Documents/test/pacman/sync/").unwrap();
    for path in paths {
        let file_path: String = format!("{}/desc", path.unwrap().path().display());
        all_file.push(file_path.clone());
    }

    all_file.par_iter().for_each(|n| {
        let mut package_info: DescFile = DescFile::new();
        package_info.import(n).export().unwrap_or(());
        // true
    });

    // for path in paths {
    //     let file_path = &format!("{}/desc", path.unwrap().path().display());
    //     let mut package_info: DescFile = DescFile::new();
    //     package_info.import(file_path).export().unwrap_or(());
    // }
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
