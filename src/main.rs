extern crate rustc_serialize;
extern crate getopts;


use rustc_serialize::json;
use getopts::Options;
use std::env;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

struct CommandlineOptions {
	add_bookmark: bool,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn read_json() -> HashMap<String,String> {
    let path = Path::new(".bookmarks");

    let mut file = match File::open(&path) {
    	Err(why) => panic!("Could not open {}: {}", path.display(), Error::description(&why)),
    	Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
    	Err(why) => panic!("Couldn't read {}: {}", path.display(), Error::description(&why)),
    	Ok(_) => 0,
    };

    let mut map: HashMap<String,String> = json::decode(&s).unwrap();
    map
}

fn main() {
	let map = read_json();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let cwd_path = env::current_dir().unwrap();
    let cwd: String = cwd_path.to_str().unwrap().to_string();

    let mut opts = Options::new();
    opts.optopt("a", "", "Add new bookmark", "NAME");

    let matches = opts.parse(&args[1..]).unwrap();

    match matches.opt_str("a") {
    	Some(a: String) => { map[a] = cwd },
    	None => {},
    };

}
