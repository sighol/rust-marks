extern crate rustc_serialize;
extern crate docopt;


use rustc_serialize::json;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

static USAGE: &'static str = "
Usage:
    marks
    marks <tag>
    marks --add=TAG
    marks --remove=TAG
    marks --keys
    marks --check
    marks --clean
    marks --help

Options:
    -k, --keys          Show keys.
    -a, --add=TAG       Add new tag.
    -r, --remove=TAG    Remove tag.
    -h, --help          Show this message.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_tag: Option<String>,
    flag_add: Option<String>,
    flag_remove: Option<String>,
    flag_keys: bool,
    flag_check: bool,
    flag_clean: bool,
}

fn main() {

    let args: Args = docopt::Docopt::new(USAGE)
                                    .map(|a| a.help(true))
                                    .and_then(|d| d.decode())
                                    .unwrap_or_else(|e| e.exit());
    let path_string = get_path();
    let path = Path::new(&path_string);

    if std::fs::metadata(&path_string).is_err() {
        let empty_map: HashMap<String, String> = HashMap::new();
        write_json(&path, &empty_map);
    }

    let mut map = read_json(&path);

    let cwd_path = env::current_dir().unwrap();
    let cwd: String = cwd_path.to_str().unwrap().to_string();

    if let Some(a) = args.flag_add {
        map.insert(a, cwd.clone());
        write_json(&path, &map);
    } else if let Some(bm) = args.flag_remove {
        let bm_str = &bm;
        if map.remove(bm_str).is_some() {
            write_json(&path, &map);
        }
    } else if args.flag_keys {
        for key in get_keys(&map) {
            println!("{}", key);
        }
    } else if args.flag_check {
        for (key, value) in &map {
            if std::fs::metadata(&value).is_err()
            {
                println!("{}, {} does not exist", key, value);
            }
        }
    } else if args.flag_clean {
        clean(&mut map, &path);
    } else if let Some(key) = args.arg_tag {
        if let Some(value) = map.get(&key) {
            println!("{}", value);
        } else {
            panic!("Key not found: {}", key);
        }
    } else {
        print_map(&map);
    }
}

fn clean(map: &mut HashMap<String, String>, path: &Path) {
    let keys = get_bad_keys(map);
    for key in &keys {
        println!("Removing {} ...", &key);
        let my_key: String = key.clone();
        map.remove(&my_key);
    }

    write_json(&path, &map);
}

fn get_bad_keys(map: &mut HashMap<String, String>) -> Vec<String> {
    let mut remove_keys = Vec::new();
    for (key, value) in map {
        if std::fs::metadata(&value).is_err()
        {
            remove_keys.push(key.to_string());
        }
    }
    remove_keys
}

fn get_path() -> PathBuf {
    let mut bookmark = env::home_dir().unwrap();
    bookmark.push(".bookmarks");
    bookmark
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
    max_len += 1;

    let keys = get_keys(map);
    for key in keys {
        let mut bfr = "".to_string();
        let len = key.len();
        for _ in 0..(max_len-len) {
            bfr.push(' ');
        }
        println!("{}{}: {}", bfr, key, map.get(key).unwrap());
    }
}
