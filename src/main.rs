use clap::{Parser};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use std::io::BufReader;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg()]
    arg_bookmark: Option<String>,

    #[arg(short, long)]
    add: Option<String>,

    #[arg(short, long)]
    delete: Option<String>,

    #[arg(short, long)]
    keys: bool,

    #[arg(long)]
    check: bool,

    #[arg(long)]
    clean: bool,
}


type StringMap = HashMap<String, String>;

struct BookmarksMap {
    path: PathBuf,
    map: StringMap,
}


fn main() {
    let args = Args::parse();

    let mut bm = BookmarksMap::new();

    if let Some(key) = args.add {
        bm.add(&key);
        bm.write()
    } else if let Some(key) = args.delete {
        bm.remove(&key);
        bm.write()
    } else if args.keys {
        bm.print_keys();
    } else if args.check {
        bm.check();
    } else if args.clean {
        bm.clean();
        bm.write();
    } else if let Some(key) = args.arg_bookmark {
        if let Some(value) = bm.get(&key) {
            println!("{}", value);
        } else {
            eprintln!("Unknown bookmark key: {}", key);
            std::process::exit(1);
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
