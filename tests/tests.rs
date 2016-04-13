extern crate liquery;
extern crate liquery_file;

use liquery_file::QueryFile;
use liquery::Queryable;
use std::path::Path;

// TODO: refactor with macro

#[test]
fn size_of_file() {
    let path = Path::new("tests/file_1b.txt");
    let queryable = QueryFile::new(path).unwrap();

    assert_eq!("1", &queryable.query("size").unwrap());
}

#[test]
fn file_type_file() {
    let path = Path::new("tests/file_1b.txt");
    let queryable = QueryFile::new(path).unwrap();

    assert_eq!("file", &queryable.query("filetype").unwrap());
}

#[test]
fn file_type_dir() {
    let path = Path::new("tests");
    let queryable = QueryFile::new(path).unwrap();

    assert_eq!("directory", &queryable.query("filetype").unwrap());
}

#[test]
fn file_name_dir() {
    let path = Path::new("tests");
    let queryable = QueryFile::new(path).unwrap();

    assert_eq!("tests", &queryable.query("filename").unwrap());
}

#[test]
fn file_name() {
    let path = Path::new("tests/file_1b.txt");
    let queryable = QueryFile::new(path).unwrap();

    assert_eq!("file_1b.txt", &queryable.query("filename").unwrap());
}

#[test]
fn file_extension() {
    let path = Path::new("tests/file_1b.txt");
    let queryable = QueryFile::new(path).unwrap();

    assert_eq!("txt", &queryable.query("extension").unwrap());
}

#[test]
fn file_extension_dir() {
    let path = Path::new("tests");
    let queryable = QueryFile::new(path).unwrap();

    assert_eq!(None, queryable.query("extension"));
}
