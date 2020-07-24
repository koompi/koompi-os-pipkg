use crate::schemas::{desc_file::App, store::Store};
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
impl Store {
    pub fn new(apps: Vec<App>) -> Store {
        Store { apps: apps }
    }
    pub fn export(&self) -> Result<()> {
        let output_file = &format!("./dist/{}.json", "store");
        let mut f = File::create(output_file)?;
        f.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;

        f.sync_all()?;
        Ok(())
    }
}
