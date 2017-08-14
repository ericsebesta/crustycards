#[macro_use]
extern crate serde_json;

use serde_json::Value;
use std::io;
use std::io::prelude::*;
use std::fs::File;

struct Card {
    name: String,
    json: Value,
}

fn load_json_from_file(file_relative_path: &str) -> io::Result<String> {
    let mut f = File::open(file_relative_path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse_json_to_values(buffer: &str) -> Result<Value, String> {
    match serde_json::from_str(buffer) {
        Ok(value) => Ok(value),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_values_to_cards(values: &Value) -> Result<Vec<Card>, String> {
    let cards = Vec::new();
    match values {
        Null => println!("found null"),
        Bool => println!("found null"),
        Number => println!("found null"),
        String => println!("found null"),
        Array => println!("found null"),
        Object => println!("found null"),
    }
    Ok(cards)
}

fn main() {
    let json_as_string_result = load_json_from_file("data/cards.collectible.json");
    match json_as_string_result {
        Ok(json_as_string) => {
            let json_as_value_result = parse_json_to_values(&json_as_string);
            match json_as_value_result {
                Ok(json_as_value) => {
                    let pretty_result = serde_json::to_string_pretty(&json_as_value);
                    match pretty_result {
                        Ok(v) => println!("Data is: {}", v),
                        Err(e) => println!("Couldn't pretty print: {}", e),
                    }
                    let cards = parse_values_to_cards(&json_as_value);
                }
                Err(e) => println!("Couldn't parse file to Values: {}", e),
            }
        }
        Err(e) => println!("Error: Coulding load file: {}", e),
    }
}

/////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_json_to_values_empty_string_is_error() {
        let result = parse_json_to_values("");
        assert!(result.is_err());
    }

    #[test]
    fn parse_json_to_values_simple_json_is_valid() {
        let data = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;
        let result = parse_json_to_values(&data);
        assert!(result.is_ok());
    }
}
