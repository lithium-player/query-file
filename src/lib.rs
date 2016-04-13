//! # liquery-file
//!

extern crate liquery;
extern crate mime_guess;

use std::path::Path;
use std::fs::Metadata;

use liquery::Queryable;

pub struct QueryFile<'a> {
    metadata: Metadata,
    path: &'a Path,
}

impl <'a> QueryFile<'a> {
    pub fn new(path: &'a Path) -> std::io::Result<Self> {
        Ok(QueryFile {
            metadata: try!(path.metadata()),
            path: path,
        })
    }
}

impl <'a>  Queryable for QueryFile<'a> {
    fn query(&self, key: &str) -> Option<String> {
        match key {
            "size" => Some(format!("{}", self.metadata.len())),
            "filetype" => {
                let filetype = self.metadata.file_type();
                if filetype.is_symlink() {
                    Some("symlink".to_owned())
                } else if filetype.is_dir() {
                    Some("directory".to_owned())
                } else {
                    Some("file".to_owned())
                }
            },
            "extension" => match self.path.extension() {
                Some(ostr) => match ostr.to_str() {
                    Some(pstr) => Some(pstr.to_owned()),
                    None => None,
                },
                None => None,
            },
            "filename" => match self.path.file_name() {
                Some(ostr) => match ostr.to_str() {
                    Some(pstr) => Some(pstr.to_owned()),
                    None => None,
                },
                None => None,
            },
            "mimetype" => Some(format!("{}", mime_guess::guess_mime_type(self.path))),
            _ => None,
        }
    }
}
