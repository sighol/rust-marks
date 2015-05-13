
extern crate rustc_serialize;
extern crate getopts;


use rustc_serialize::json;
use getopts::Options;
use std::env;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::collections::BTreeMap;

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

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&program, opts);
            panic!("{}", f)
        }
    };

    if let Some(a) = matches.opt_str("a") {
        map.insert(a, cwd.clone());
        write_json(&path, &map);
    } else if let Some(bm) = matches.opt_str("r") {
        let bm_str = &bm;
        if map.contains_key(bm_str) {
            map.remove(bm_str);
            write_json(&path, &map);
        }
    } else if matches.opt_present("k") {
        let keys = get_keys(&map);
        for key in keys {
            println!("{}", key);
        }
    } else if !matches.free.is_empty() {
        let key = matches.free[0].clone();
        if map.contains_key(&key) {
            println!("{}", map.get(&key).unwrap());
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
    let mut file = match File::open(path) {
    	Err(why) => panic!("Could not open {}: {}", path.display(), Error::description(&why)),
    	Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
    	Err(why) => panic!("Couldn't read {}: {}", path.display(), Error::description(&why)),
    	Ok(_) => 0,
    };

    let map: HashMap<String,String> = json::decode(&s).unwrap();
    map
}

fn write_json(path: &Path, map: &HashMap<String, String>) {
	let output = json::encode(map).unwrap();

	let mut f = File::create(path).unwrap();
	f.write_all(output.as_bytes());
	f.sync_all();
}

fn get_keys(map: &HashMap<String, String>) -> Vec<&String> {
	let mut keys: Vec<&String> = map.keys().collect();
	keys.sort();
	keys
}

fn print_map(map: &HashMap<String, String>) {
	let mut max_len = 0;
	for (key, value) in map {
		if key.len() > max_len {
			max_len = key.len();
		}
	}
	max_len += 2;

	let keys = get_keys(map);

	for key in keys {
		let mut bfr = "".to_string();
		let len = key.len();
		for i in 0..(max_len-len) {
			bfr.insert(i, ' ');
		}
		println!("{}{} : {}", bfr, key, map.get(key).unwrap());
	}
}
