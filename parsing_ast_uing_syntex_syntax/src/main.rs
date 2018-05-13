extern crate syntex_syntax;

use std::path::*;
use syntex_syntax::parse::*;
use syntex_syntax::codemap::*;

fn main() {
    // Create parser and parse.
    let f = file!();
    let p = Path::new(f);
    let m = FilePathMapping::empty();
    let s = ParseSess::new(m);
    let r = parse_crate_from_file(&p, &s);
    println!("{:?}", r);
}


