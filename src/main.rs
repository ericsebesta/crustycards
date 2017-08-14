extern crate serde_json;

use serde_json::{Value, Error};
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn load_json(file_relative_path: &str) -> io::Result<Value> {
    let mut f = File::open(file_relative_path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let v: Value = serde_json::from_str(&buffer)?;
    Ok(v)
}

fn main() {
    let result = load_json("data/cards.collectible.json");
    match result {
        Ok(value) => {
            let pretty_result = serde_json::to_string_pretty(&value);
            match pretty_result {
                Ok(v) => println!("Data is: {}", v),
                Err(e) => println!("Couldn't pretty print: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
