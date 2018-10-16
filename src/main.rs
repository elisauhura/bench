use std::io::Read;

extern crate serde_json;

mod stat;

fn main() {
    let file_key = "BENCH_STDFILE";
    let path_key = "BENCH_STDPATH";
    let id_key = "BENCH_STDID";
    let deafult_stdfile = String::from("bench.json");
    let deafult_stdpath = String::from("");
    let deafult_stdid = String::from("ANON");
    let help_msg = String::from(r#"
bench, a benchmark automation tool

syntax: bench <cmd>

available commands are:

build:
    build you project
check:
    check is the project is ready to be runned
help:
    display this message
run:
    run the project and measure the results

offset path is done via the BENCH_STDPATH, eg:
BENCH_STDPATH=../foo/ bench build

author id is set via the BENCH_STDID, otherwise, ANON will be used.

    "#);
    let file: String;
    let path: String;
    let seek: String;
    let id: String;
    let mut mark = 0;
    let mut command: String = String::from("");

    for argument in std::env::args() {
        mark += 1;
        if mark == 2 {
            command = argument.clone();
            break;
        }
    }

    if command == String::from("") {
        command = String::from("help");
    }

    match std::env::var(file_key) {
        Ok(val) => file = val,
        Err(_e) => file = deafult_stdfile
    }
    match std::env::var(path_key) {
        Ok(val) => path = val,
        Err(_e) => path = deafult_stdpath
    }
    match std::env::var(id_key) {
        Ok(val) => id = val,
        Err(_e) => id = deafult_stdid
    }

    match command.as_ref() {
        "help" => {
            print!("{}", help_msg);
            return;
        }
        _ => {
            panic!("Err: Unknown command to bench");
        }
    }

    seek = String::from(path.as_str()) + file.as_str();
    println!("Looking for {}", seek);

    let mut f = std::fs::File::open(seek).expect("Err: file not found");
    let mut raw_text= String::new();

    f.read_to_string(&mut raw_text).expect("Err: could not read file");

    // TODO: Implement a real parse to check if the file is a valid bench file
    let v: serde_json::Value = serde_json::from_str(raw_text.as_str()).expect("Err: could not parse file");

    check_folders(&path);
    println!("Folders are OK!");
}

fn check_folders(offset: &String) {
    check_folder(String::from(offset.as_str()) + "bin");
    check_folder(String::from(offset.as_str()) + "src");
    check_folder(String::from(offset.as_str()) + "log");
}

fn check_folder(path: String) {
    let path = std::path::Path::new(path.as_str());
    if !path.exists() {
        std::fs::create_dir(path).expect("Err: could not create folder");
    }
}