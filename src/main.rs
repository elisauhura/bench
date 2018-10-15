use std::fs::File;
use std::io::prelude::*;

extern crate serde_json;

mod stat;

fn main() {
    let file_key = "BENCH_STDFILE";
    let path_key = "BENCH_STDPATH";
    let deafult_stdfile = String::from("bench.json");
    let deafult_stdpath = String::from("");
    let file: String;
    let path: String;
    let seek: String;
    match std::env::var(file_key) {
        Ok(val) => file = val,
        Err(_e) => file = deafult_stdfile
    }
    match std::env::var(path_key) {
        Ok(val) => path = val,
        Err(_e) => path = deafult_stdpath
    }
    seek = path + file.as_str();
    println!("Looking for {}", seek);

    let mut f = File::open(seek).expect("Err: file not found");
    let mut raw_text= String::new();

    f.read_to_string(&mut raw_text).expect("Err: could not read file");

    // TODO: Implement a real parse to check if the file is a valid bench file
    let v: serde_json::Value = serde_json::from_str(raw_text.as_str()).expect("Err: could not parse file");
}

