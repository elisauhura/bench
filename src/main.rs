use std::io::Read;

extern crate serde_json;

mod stat;

enum Action {
    Build,
    Check,
    Run,
    Export
}

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
export:
    generate CSV from the results
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
    let cmd: Action;

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
        //implement more stuff here... build, run and export mainly, later generate a bench_run_set script
        "check" => {
            cmd = Action::Check;
        }
        "export" => {
            cmd = Action::Export;
        }
        "build" => {
            cmd = Action::Build;
        }
        "run" => {
            cmd = Action::Run;
        }
        _ => {
            panic!("Err: Unknown command to bench");
        }
    }

    seek = String::from(path.as_str()) + file.as_str();
    //println!("Looking for {}", seek);

    let mut f = std::fs::File::open(&seek).expect("Err: file not found");
    let mut raw_text= String::new();

    f.read_to_string(&mut raw_text).expect("Err: could not read file");

    // TODO: Implement a real parse to check if the file is a valid bench file
    let mut v: serde_json::Value = serde_json::from_str(raw_text.as_str()).expect("Err: could not parse file");

    check_folders(&path);
    //println!("Folders are OK!");

    match cmd {
        Action::Build => {
            let lock: bool;
            v["build"] = serde_json::Value::Bool(false);
            clear_build(&path);
            //By now it will just invoke a build command, it shall be improved in the future
            match v["build_cmd"] {
                serde_json::Value::String(ref q) => {
                    match v["build_arg"] {
                        serde_json::Value::String(ref v) => { lock = std::process::Command::new(q).arg(v).current_dir(&path).status().expect("Err: could not build benchmark").success(); }
                        _ => { panic!("Err: build_arg is not a string"); }
                    }
                }
                _ => { panic!("Err: build_cmd is not a string"); }
            }
            if lock { v["build"] = serde_json::Value::Bool(lock); }
        }
        Action::Check => {
            if v["build"] == true {
                std::process::exit(0);
            }
            std::process::exit(1);
        }
        Action::Run => {
            if v["build"] != true {
                std::process::exit(1);
            }
            match v["run_cmd"] {
                serde_json::Value::String(ref q) => {
                    match v["run_arg"] {
                        serde_json::Value::String(ref v) => { std::process::Command::new(q).arg(v).current_dir(&path).status().expect("Err: could not run benchmark").success(); }
                        _ => { panic!("Err: run_arg is not a string"); }
                    }
                }
                _ => { panic!("Err: run_cmd is not a string"); }
            }
        }
        Action::Export => {
            let mut f = std::fs::File::open(String::from(path.as_str()) + "log/out").expect("Err: file not found");
            let mut raw_text= String::new();

            f.read_to_string(&mut raw_text).expect("Err: could not read file");
            println!("{}", raw_text);
        }
    }

    raw_text = serde_json::to_string_pretty(&v).expect("Err: could not parse object to json");

    std::fs::write(seek, raw_text).expect("Err: could not write to bench file");
}

fn check_folders(offset: &String) {
    check_folder(String::from(offset.as_str()) + "bin");
    check_folder(String::from(offset.as_str()) + "src");
    check_folder(String::from(offset.as_str()) + "log");
    check_folder(String::from(offset.as_str()) + "input");
    check_folder(String::from(offset.as_str()) + "output");
}

fn check_folder(path: String) {
    let path = std::path::Path::new(path.as_str());
    if !path.exists() {
        std::fs::create_dir(path).expect("Err: could not create folder");
    }
}

fn clear_build(offset: &String) {
    std::fs::remove_dir_all(String::from(offset.as_str()) + "bin").expect("Err: could not clean bin directory");
    check_folder(String::from(offset.as_str()) + "bin");
}