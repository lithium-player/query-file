#![feature(test)]

extern crate liquery;
extern crate liquery_file;
extern crate test;

use std::path::Path;

use test::Bencher;
use liquery::Queryable;
use liquery_file::QueryFile;

#[bench]
fn bench_new_file(b: &mut Bencher) {
    b.iter(move || {
        let _ = QueryFile::new(Path::new("tests/file_1b.txt"));
    });
}

#[bench]
fn bench_new_dir(b: &mut Bencher) {
    b.iter(move || {
        let _ = QueryFile::new(Path::new("tests"));
    });
}

macro_rules! bench_query {
    ($name: ident, $query: expr) => {
        bench_query!($name, $query, "tests/file_1b.txt");
    };
    ($name: ident, $query: expr, $path: expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let file = QueryFile::new(Path::new($path)).unwrap();
            b.iter(move || {
                let _ = file.query($query);
            });
        }
    };
}

bench_query!(bench_query_size, "size");
bench_query!(bench_query_extension, "extension");
bench_query!(bench_query_filename, "filename");
bench_query!(bench_query_filetype, "filetype");
bench_query!(bench_query_mimetype, "mimetype");
bench_query!(bench_query_unkown, "");

bench_query!(bench_query_size_dir, "size", "tests");
bench_query!(bench_query_extension_dir, "extension", "tests");
bench_query!(bench_query_filename_dir, "filename", "tests");
bench_query!(bench_query_filetype_dir, "filetype", "tests");
bench_query!(bench_query_mimetype_dir, "mimetype", "tests");
bench_query!(bench_query_unkown_dir, "", "tests");
