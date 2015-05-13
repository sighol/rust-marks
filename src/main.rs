extern crate rustc_serialize;
extern crate getopts;


use rustc_serialize::json;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

fn main() {
    let path_sting = get_path();
    let path = Path::new(&path_sting);
    let mut map = read_json(&path);

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let cwd_path = env::current_dir().unwrap();
    let cwd: String = cwd_path.to_str().unwrap().to_string();

    let mut opts = Options::new();
    opts.optopt("a", "add", "Add new bookmark", "NAME");
    opts.optopt("r", "remove", "Remove bookmark", "NAME");
    opts.optflag("k", "keys", "List Keys");


    let matches = opts.parse(&args[1..]).unwrap_or_else(|e| {
        print_usage(&program, opts);
        panic!("{}", e)
    });

    if let Some(a) = matches.opt_str("a") {
        map.insert(a, cwd.clone());
        write_json(&path, &map);
    } else if let Some(bm) = matches.opt_str("r") {
        let bm_str = &bm;
        if map.remove(bm_str).is_some() {
            write_json(&path, &map);
        }
    } else if matches.opt_present("k") {
        for key in get_keys(&map) {
            println!("{}", key);
        }
    } else if !matches.free.is_empty() {
        let key = matches.free[0].clone();
        if let Some(value) = map.get(&key) {
            println!("{}", value);
        } else {
            panic!("Key not found: {}", key);
        }
    } else {
        print_map(&map);
    }
}

fn get_path() -> PathBuf {
    let mut bookmark = env::home_dir().unwrap();
    bookmark.push(".bookmarks");
    bookmark
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn read_json(path: &Path) -> HashMap<String, String> {
    let display = path.display();
    let mut file = File::open(path).unwrap_or_else(|why| {
        panic!("Could not open {}: {}", display, why);
    });

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap_or_else(|why| {
        panic!("Couldn't read {}: {}", display, why);
    });

    let map: HashMap<String,String> = json::decode(&s).unwrap_or_else(|why| {
        panic!("Could not Decode JSON: {}", why);
    });
    map
}

fn write_json(path: &Path, map: &HashMap<String, String>) {
    let output = json::encode(map).unwrap();

    let mut f = File::create(path).unwrap_or_else(|why| {
        panic!("Could not create file {}: {}", path.display(), why);
    });
    f.write_all(output.as_bytes()).unwrap_or_else(|why| {
        panic!("Could not write to file: {}", why);
    });
    f.sync_all().unwrap_or_else(|why| {
        panic!("Could not sync file: {}", why)
    });
}

fn get_keys(map: &HashMap<String, String>) -> Vec<&String> {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    keys
}

fn print_map(map: &HashMap<String, String>) {
    let mut max_len = 0;
    for (key, _) in map {
        if key.len() > max_len {
            max_len = key.len();
        }
    }
    max_len += 2;

    let keys = get_keys(map);
    for key in keys {
        let mut bfr = "".to_string();
        let len = key.len();
        for _ in 0..(max_len-len) {
            bfr.push(' ');
        }
        println!("{}{} : {}", bfr, key, map.get(key).unwrap());
    }
}
