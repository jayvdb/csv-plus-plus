//! # Csv
//!
//! Functions for writing to CSV files
//!
use csv;
use std::path::PathBuf;

use crate::{Options, Result, Template};
use super::CompilerTarget;

pub struct Csv {
    path: PathBuf,
    builder: csv::WriterBuilder,
}

impl CompilerTarget for Csv {
    fn write(&self, options: &Options, template: &Template) -> Result<()> {
        // let mut builder = csv::WriterBuilder::new();
        // let mut csv_target = Csv { builder };
        // TODO

        Ok(())
    }
}

impl Csv {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            builder: csv::WriterBuilder::new(),
        }
    }

    pub fn supports_extension(os_str: &std::ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("csv")
    }
}
