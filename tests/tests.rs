extern crate liquery;
extern crate liquery_file;

use std::collections::HashMap;

use liquery_file::QueryFile;
use liquery::{Query, EvalFunc};
use std::path::Path;

macro_rules! query_test {
    ($name: ident, $query: expr, $out: expr) => {
        query_test!($name, $query, $out, "tests/file_1b.txt");
    };
    ($name: ident, $query: expr, $out: expr, $path: expr) => {
        #[test]
        fn $name() {
            let path = Path::new($path);
            let queryable = QueryFile::new(path).unwrap();

            let func = HashMap::<String, Box<EvalFunc>>::new();

            let result = Query::parse($query.to_owned())
                .unwrap()
                .eval(&queryable, &func)
                .unwrap();

            assert_eq!($out, result);
        }
    };
}

query_test!(filename_query, "%filename%", "file_1b.txt");
