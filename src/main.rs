extern crate docopt;

extern crate rustc_serialize;
extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use std::io::BufReader;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_bookmark: Option<String>,
    flag_add: Option<String>,
    flag_delete: Option<String>,
    flag_keys: bool,
    flag_check: bool,
    flag_clean: bool,
    flag_version: bool,
}

type StringMap = HashMap<String, String>;

struct BookmarksMap {
    path: PathBuf,
    map: StringMap,
}

static USAGE: &'static str = "
Usage:
    marks
    marks <bookmark>
    marks --add=<bookmark>
    marks --delete=<bookmark>
    marks --keys
    marks --check
    marks --clean
    marks --version
    marks --help

Options:
    -k, --keys               Show keys.
    -a, --add=<bookmark>     Add bookmark for current directory
    -d, --delete=<bookmark>  Delete bookmark.
    -h, --help               Show this message.
    --check                  Check for tags pointing to non.
    --clean                  Delete non existing bookmarks.
    --version                Print version information.
";

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .map(|a| a.help(true))
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut bm = BookmarksMap::new();

    if let Some(key) = args.flag_add {
        bm.add(&key);
        bm.write()
    } else if let Some(key) = args.flag_delete {
        bm.remove(&key);
        bm.write()
    } else if args.flag_keys {
        bm.print_keys();
    } else if args.flag_check {
        bm.check();
    } else if args.flag_clean {
        bm.clean();
        bm.write();
    } else if args.flag_version {
        println!("{}", VERSION);
    } else if let Some(key) = args.arg_bookmark {
        if let Some(value) = bm.get(&key) {
            println!("{}", value);
        } else {
            panic!("Key not found {}", key);
        }
    } else {
        bm.print();
    }
}

impl BookmarksMap {
    fn new() -> BookmarksMap {
        let path_buf = BookmarksMap::get_path();

        BookmarksMap {
            map: BookmarksMap::read(&path_buf),
            path: path_buf,
        }
    }

    fn get_path() -> PathBuf {
        let mut bookmark = env::home_dir().unwrap();
        bookmark.push(".bookmarks");
        bookmark
    }

    fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).map(|v| v.clone())
    }

    fn add(&mut self, key: &str) {
        let cwd_path = env::current_dir().unwrap();
        let cwd = cwd_path.to_str().unwrap().to_string();
        self.map.insert(key.to_string(), cwd);
    }

    fn remove(&mut self, key: &str) {
        if self.map.remove(key).is_none() {
            println!("Key '{}' not found", key);
        }
    }

    fn print_keys(&self) {
        for (key, _) in &self.map {
            println!("{}", key);
        }
    }

    fn print(&self) {
        let mut max_len = 0;
        for (key, _) in &self.map {
            if key.len() > max_len {
                max_len = key.len();
            }
        }
        max_len += 1;

        for key in self.get_keys() {
            let mut bfr = "".to_string();
            let len = key.len();
            for _ in 0..(max_len - len) {
                bfr.push(' ');
            }
            println!("{}{}: {}", bfr, key, self.map.get(key).unwrap());
        }
    }

    fn check(&self) {
        let keys = self.get_bad_keys();
        for key in &keys {
            println!("Bad key: {:10} -> {}", key, self.get(key).unwrap());
        }
    }

    fn clean(&mut self) {
        let keys = self.get_bad_keys();
        for key in &keys {
            println!("Removing {} ...", key);
            self.map.remove(key);
        }
    }

    fn get_bad_keys(&self) -> Vec<String> {
        let mut remove_keys = Vec::new();
        for (key, value) in &self.map {
            if std::fs::metadata(value).is_err() {
                remove_keys.push(key.to_string());
            }
        }
        remove_keys.sort();
        remove_keys
    }

    fn get_keys(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = self.map.keys().collect();
        keys.sort();
        keys
    }

    fn read(path: &Path) -> StringMap {
        match File::open(path) {
            Err(_) => HashMap::new(),
            Ok(file) => {
                let mut reader = BufReader::new(file);
                serde_json::from_reader(&mut reader).expect("Could not decode JSON")
            }
        }
    }

    fn write(&self) {
        let mut f = File::create(&self.path).expect("Could not create file");
        serde_json::to_writer_pretty(&mut f, &self.map).expect("Could not serialize");
        f.sync_all().expect("Could not sync");
    }
}
