use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DescFile {
    pub FILENAME: String,
    pub NAME: String,
    pub BASE: String,
    pub VERSION: String,
    pub DESC: String,
    pub CSIZE: String,
    pub ISIZE: String,
    pub MD5SUM: String,
    pub SHA256SUM: String,
    pub PGPSIG: String,
    pub URL: String,
    pub LICENSE: String,
    pub ARCH: String,
    pub BUILDDATE: String,
    pub PACKAGER: String,
    pub PROVIDES: Vec<String>,
    pub DEPENDS: Vec<String>,
}
