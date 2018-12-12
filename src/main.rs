use std::io::Read;
use std::io::Write;

extern crate serde_json;
extern crate statistical;

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
    let deafult_stdpath = String::from("./");
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
    requires `neofetch` to get hardware information
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
            print!("{}", help_msg);
            return;
        }
    }

    seek = String::from(path.as_str()) + file.as_str();

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
                        serde_json::Value::String(ref v) => {
                            let out: String;
                            out = String::from(std::str::from_utf8(&std::process::Command::new(q).arg(v).arg("-s").current_dir(&path).output().expect("Err: could not run benchmark").stdout).expect("Err: could not read output"));
                            let mut q: serde_json::Value = serde_json::from_str(out.as_str()).expect("Err: parsing benchmark output");
                            match q["out"] {
                                serde_json::Value::Array(ref mut r) => {
                                    for i in r {
                                        match i {
                                            serde_json::Value::Object(ref mut s) => {
                                                s["id"] = serde_json::Value::String(id.clone());
                                            }
                                            _ => { panic!("Err: parsing benchmark output"); }
                                        }
                                    }
                                }
                                _ => { panic!("Err: parsing benchmark output"); }
                            }
                            std::fs::File::write_all(&mut force_write(String::from(path.as_str()) + "/log/out.json"), q.to_string().as_bytes()).expect("Err: could not write bench out");
                        }
                        _ => { panic!("Err: run_arg is not a string"); }
                    }
                }
                _ => { panic!("Err: run_cmd is not a string"); }
            }
        }
        Action::Export => {
            //Read json, print as CSV
            let mut f = std::fs::File::open(String::from(path.as_str()) + "/log/out.json").expect("Err: file not found");
            let mut raw_text= String::new();
            let mut hardware = String::from("not available");
            let out = std::process::Command::new("neofetch").arg("--stdout").output();
            match out {
                Ok(s) => {hardware = String::from(std::str::from_utf8(&s.stdout).expect("Err: could not get neofetch out")).replace("\n", " ").replace("\"","\"\"").replace(",", ";")}
                Err(_) => {}
            }

            f.read_to_string(&mut raw_text).expect("Err: could not read file");
            let log: serde_json::Value = serde_json::from_str(raw_text.as_str()).expect("Err: could not parse out.json file");
            match log["out"] {
                serde_json::Value::Array(ref r) => {
                    for i in r {
                        match i {
                            serde_json::Value::Object(ref s) => {
                                match s["tasks"] {
                                    serde_json::Value::Array(ref tasks) => {
                                        let mut values = Vec::new();
                                        for num in tasks {
                                            match num {
                                                serde_json::Value::Number(n) => {
                                                    let n = match n.as_f64() {
                                                        Some(q) => { q }
                                                        None => {0.0}
                                                    };
                                                    if n != 0.0 {
                                                        values.push(n);
                                                    }
                                                }
                                                _ => { }
                                            }
                                        }
                                        /*Generate statistics*/
                                        if values.len() > 0 {
                                            let mean = statistical::mean(&values);
                                            let median = statistical::median(&values);
                                            let dev = statistical::standard_deviation(&values, Some(mean));
                                            println!("{},{},{},{},{},\"{}\",{},{},{}", s["bench"], s["id"], s["args"], s["mode"], hardware, s["time"], mean, median, dev);
                                        } else {
                                            println!("{},{},{},{},{},{},_,_,_", s["bench"], s["id"], s["args"], s["mode"], hardware, s["time"]);
                                        }
                                    }
                                    _ => {
                                        println!("{},{},{},{},{},{},_,_,_", s["bench"], s["id"], s["args"], s["mode"], hardware, s["time"]);
                                    }
                                }
                            }
                            _ => { panic!("Err: parsing benchmark output"); }
                        }
                    }
                }
                _ => { panic!("Err: parsing benchmark output"); }
            }
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

fn force_write(path: String) -> std::fs::File {
    match std::fs::remove_file(&path) {
        Ok(_) => std::fs::File::create(&path).expect("Err: could not create file"),
        Err(_) => std::fs::File::create(&path).expect("Err: could not create file")
    }
}