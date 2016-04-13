//! # liquery-file
//!

extern crate liquery;
extern crate mime_guess;

use std::path::Path;
use std::fs::{File, Metadata};

use liquery::Queryable;

struct QueryFile {
    metadata: Metadata,
}

impl QueryFile {
    pub fn new(path: &Path) -> std::io::Result<Self> {
        let file = try!(File::open(path));
        Ok(QueryFile { metadata: try!(file.metadata()) })
    }
}

impl Queryable for QueryFile {
    fn query(&self, key: &str) -> Option<String> {
        match key {
            "size" => Some(format!("{}", self.metadata.len())),
            _ => None,
        }
    }
}
