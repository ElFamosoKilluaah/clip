extern crate clipboard2;

use clipboard2::{SystemClipboard, Clipboard};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let clipboard = SystemClipboard::new().unwrap();
    let files = fs::read_dir(path_to_search()).unwrap();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: clip <file>");
        return;
    }

    let to_get = args.get(1).unwrap();

    for path in files {
        let value = path.unwrap().path();
        let key = String::from(value.clone().to_str().unwrap());
        println!("Key: {}, arg: {}, contains: {}", &key, &to_get, key.contains(to_get));

        if key.contains(to_get) {
            let mut file = File::open(&value).unwrap();
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer).unwrap();

            clipboard.set_string_contents(String::from_utf8(buffer).map_err(|_| {
                eprintln!("Please verify that this file is encoded in the UTF-8 format !");
            }).unwrap()).unwrap();
            println!("Content set !");
            return;
        }
    }
}

fn path_to_search() -> String {
    let mut to_return = String::from(std::env::current_exe().unwrap().to_str().unwrap());
    let current = env::current_exe().unwrap();
    let to_remove = &current.file_name().unwrap().to_str().unwrap();
    let index = to_return.len() - to_remove.len();
    to_return.split_off(index);

    println!("{}", to_return);
    to_return

}
