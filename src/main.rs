#![allow(non_snake_case)]

use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Info {
    FILENAME: String,
    NAME: String,
    BASE: String,
    VERSION: String,
    DESC: String,
    CSIZE: String,
    ISIZE: String,
    MD5SUM: String,
    SHA256SUM: String,
    PGPSIG: String,
    URL: String,
    LICENSE: String,
    ARCH: String,
    BUILDDATE: String,
    PACKAGER: String,
    PROVIDES: Vec<String>,
    DEPENDS: Vec<String>,
}

fn main() {
    let mut package_info: Info = Info {
        FILENAME: String::from(""),
        NAME: String::from(""),
        BASE: String::from(""),
        VERSION: String::from(""),
        DESC: String::from(""),
        CSIZE: String::from(""),
        ISIZE: String::from(""),
        MD5SUM: String::from(""),
        SHA256SUM: String::from(""),
        PGPSIG: String::from(""),
        URL: String::from(""),
        LICENSE: String::from(""),
        ARCH: String::from(""),
        BUILDDATE: String::from(""),
        PACKAGER: String::from(""),
        PROVIDES: vec![String::from("")],
        DEPENDS: vec![String::from("")],
    };
    let data = file_reader("desc");
    // split file key
    let mut lines: Vec<&str> = data.split("\n\n").collect();
    // split each column
    for line in lines.iter_mut() {
        let splitted: Vec<&str> = line.split("%").collect();
        for (i, c) in splitted.iter().enumerate() {
            let info: Vec<&str> = c.split("\n").collect();
            if i > 0 {
                if info.len() == 1 {
                    match splitted[i] {
                        "FILENAME" => {
                            let FILENAME = splitted[i + 1].to_string();
                            let data: Vec<&str> = FILENAME.split("\n").collect();
                            package_info.FILENAME = data[1].to_string();
                        }
                        "NAME" => {
                            let NAME = splitted[i + 1].to_string();
                            let data: Vec<&str> = NAME.split("\n").collect();
                            package_info.NAME = data[1].to_string();
                        }
                        "BASE" => {
                            let BASE = splitted[i + 1].to_string();
                            let data: Vec<&str> = BASE.split("\n").collect();
                            package_info.BASE = data[1].to_string();
                        }
                        "VERSION" => {
                            let VERSION = splitted[i + 1].to_string();
                            let data: Vec<&str> = VERSION.split("\n").collect();
                            package_info.VERSION = data[1].to_string();
                        }
                        "DESC" => {
                            let DESC = splitted[i + 1].to_string();
                            let data: Vec<&str> = DESC.split("\n").collect();
                            package_info.DESC = data[1].to_string();
                        }
                        "CSIZE" => {
                            let CSIZE = splitted[i + 1].to_string();
                            let data: Vec<&str> = CSIZE.split("\n").collect();
                            package_info.CSIZE = data[1].to_string();
                        }
                        "ISIZE" => {
                            let ISIZE = splitted[i + 1].to_string();
                            let data: Vec<&str> = ISIZE.split("\n").collect();
                            package_info.ISIZE = data[1].to_string();
                        }
                        "MD5SUM" => {
                            let MD5SUM = splitted[i + 1].to_string();
                            let data: Vec<&str> = MD5SUM.split("\n").collect();
                            package_info.MD5SUM = data[1].to_string();
                        }
                        "SHA256SUM" => {
                            let SHA256SUM = splitted[i + 1].to_string();
                            let data: Vec<&str> = SHA256SUM.split("\n").collect();
                            package_info.SHA256SUM = data[1].to_string();
                        }
                        "PGPSIG" => {
                            let PGPSIG = splitted[i + 1].to_string();
                            let data: Vec<&str> = PGPSIG.split("\n").collect();
                            package_info.PGPSIG = data[1].to_string();
                        }
                        "URL" => {
                            let URL = splitted[i + 1].to_string();
                            let data: Vec<&str> = URL.split("\n").collect();
                            package_info.URL = data[1].to_string();
                        }
                        "LICENSE" => {
                            let LICENSE = splitted[i + 1].to_string();
                            let data: Vec<&str> = LICENSE.split("\n").collect();
                            package_info.LICENSE = data[1].to_string();
                        }
                        "ARCH" => {
                            let ARCH = splitted[i + 1].to_string();
                            let data: Vec<&str> = ARCH.split("\n").collect();
                            package_info.ARCH = data[1].to_string();
                        }
                        "BUILDDATE" => {
                            let BUILDDATE = splitted[i + 1].to_string();
                            let data: Vec<&str> = BUILDDATE.split("\n").collect();
                            package_info.BUILDDATE = data[1].to_string();
                        }
                        "PACKAGER" => {
                            let PACKAGER = splitted[i + 1].to_string();
                            let data: Vec<&str> = PACKAGER.split("\n").collect();
                            package_info.PACKAGER = data[1].to_string();
                        }
                        "PROVIDES" => {
                            let PROVIDES = vec![splitted[i + 1].to_string()];
                            let data: Vec<&str> = PROVIDES[0].split("\n").collect();
                            let mut new_data: Vec<String> = Vec::new();
                            for i in data.iter() {
                                if i.to_string() != "" {
                                    new_data.push(i.to_string())
                                }
                            }
                            package_info.PROVIDES = new_data;
                        }
                        "DEPENDS" => {
                            let DEPENDS = vec![splitted[i + 1].to_string()];
                            let data: Vec<&str> = DEPENDS[0].split("\n").collect();
                            let mut new_data: Vec<String> = Vec::new();
                            for i in data.iter() {
                                if i.to_string() != "" {
                                    new_data.push(i.to_string())
                                }
                            }
                            package_info.DEPENDS = new_data;
                        }
                        &_ => continue,
                    }
                }
            }
        }
    }
    file_writer(
        package_info.clone(),
        &format!("{}.json", &package_info.NAME),
    )
    .unwrap_or(());
}

pub fn file_reader(_path: &str) -> String {
    let file = File::open(_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents
}

fn file_writer(data_src: Info, _path: &str) -> Result<()> {
    let mut f = File::create(_path)?;
    f.write_all(serde_json::to_string_pretty(&data_src)?.as_bytes())?;

    f.sync_all()?;
    Ok(())
}
