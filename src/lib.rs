//! # liquery-file
//! A library to make queryable files for the liquery `Queryable`
//! trait.

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
                Some(os_str) => match os_str.to_str() {
                    Some(s) => Some(s.to_owned()),
                    None => None,
                },
                None => None,
            },
            "filename" => match self.path.file_name() {
                Some(os_str) => match os_str.to_str() {
                    Some(s) => Some(s.to_owned()),
                    None => None,
                },
                None => None,
            },
            "mimetype" => Some(format!("{}", mime_guess::guess_mime_type(self.path))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate liquery;
    use super::*;
    use liquery::Queryable;
    use std::path::Path;

    macro_rules! query_test {
        ($name: ident, $field: expr, $out: expr) => {
            query_test!($name, $field, $out, "tests/file_1b.txt");
        };
        ($name: ident, $field: expr, $out: expr, $path: expr) => {
            #[test]
            fn $name() {
                let path = Path::new($path);
                let queryable = QueryFile::new(path).unwrap();

                assert_eq!($out, &queryable.query($field).unwrap());
            }
        };
    }

    macro_rules! query_test_no_result {
        ($name: ident, $field: expr) => {
            query_test_no_result!($name, $field, "tests/file_1b.txt");
        };
        ($name: ident, $field: expr, $path: expr) => {
            #[test]
            fn $name() {
                let path = Path::new($path);
                let queryable = QueryFile::new(path).unwrap();

                assert_eq!(None, queryable.query($field));
            }
        };
    }

    query_test!(size_of_file, "size", "1");

    query_test!(file_type_file, "filetype", "file");
    query_test!(file_type_dir, "filetype", "directory", "tests");

    query_test!(file_name, "filename", "file_1b.txt");
    query_test!(file_name_dir, "filename", "tests", "tests");

    query_test!(file_extension, "extension", "txt");
    query_test_no_result!(file_extension_dir, "extension", "tests");

    query_test!(file_mime, "mimetype", "text/plain");
    query_test!(file_mime_rust, "mimetype", "text/x-rust", "src/lib.rs");

    query_test_no_result!(unkown_feild, "unkown");
    query_test_no_result!(empty_feild, "");
}
