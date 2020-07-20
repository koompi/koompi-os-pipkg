#![allow(non_snake_case)]

mod interfaces;
mod schemas;

use schemas::desc_file::DescFile;

fn main() {
    let mut package_info: DescFile = DescFile::new();
    package_info.import("desc");
    package_info.export().unwrap_or(());
}
