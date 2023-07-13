extern crate cc;

use std::env;

fn main() {
    cc::Build::new().cpp(true).file("src/num.cpp").compile("libnum.dylib");
}