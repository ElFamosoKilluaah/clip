extern crate clipboard2;

use clipboard2::{SystemClipboard, Clipboard};
use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let clipboard = SystemClipboard::new().unwrap();
    let files = fs::read_dir("./").unwrap();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: clip <file>");
        return;
    }

    let to_get = args.get(1).unwrap();

    for path in files {
        let value = path.unwrap().path();
        let key = String::from(value.clone().to_str().unwrap());

        if key.contains(to_get) {
            let mut file = File::open(&value).unwrap();
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer).unwrap();

            clipboard.set_string_contents(String::from_utf8(buffer).map_err(|_| {
                eprintln!("Please verify that this file is encoded in the UTF-8 format !");
            }).unwrap()).unwrap();
            println!("Content set !");
        }
    }


}
