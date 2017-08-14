extern crate serde_json;

use serde_json::Value;
use std::io;
use std::io::prelude::*;
use std::fs::File;

enum ClassEnum {
    Neutral,
    Druid,
    Hunter,
    Mage,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
    Unknown,
}

enum RarityEnum {
    Common,
    Rare,
    Epic,
    Legendary,
    Free,
    Unknown,
}

enum TypeEnum {
    Minion,
    Hero,
    Weapon,
    Spell,
    Unknown,
}

enum FactionEnum {
    Alliance,
    Horde,
    Unknown,
}

struct Card {
    name: String,
    id: String,
    playerClass: ClassEnum,
    cardClass: ClassEnum,
    rarity: RarityEnum,
    theType: TypeEnum,
    faction: FactionEnum,
    artist: String,
    text: String,
    set: String,
    flavor: String,
    howToEarn: String,
    howToEarnGolden: String,
    targetingArrowText: String,
    race: String,

    attack: i64,
    cost: i64,
    dbfId: i64,
    health: i64,
    overload: i64,
    spellDamage: i64,
    durability: i64,

    collectible: bool,
    elite: bool,

    json: String,
}

impl Card {
    fn internalize_json(&mut self, map: &serde_json::Map<String, Value>) {
        //ignored fields: collectionText (looks like a dupe of text which is rarely defined)
        for (key, value) in map {
            match key.as_ref() {
                "name" => {
                    match *value {
                        Value::String(ref s) => {
                            self.name = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "id" => {
                    match *value {
                        Value::String(ref s) => {
                            self.id = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "playerClass" => {
                    match *value {
                        Value::String(ref s) => {
                            match s.as_ref() {
                                "NEUTRAL" => self.playerClass = ClassEnum::Neutral,
                                "DRUID" => self.playerClass = ClassEnum::Druid,
                                "HUNTER" => self.playerClass = ClassEnum::Hunter,
                                "MAGE" => self.playerClass = ClassEnum::Mage,
                                "PALADIN" => self.playerClass = ClassEnum::Paladin,
                                "PRIEST" => self.playerClass = ClassEnum::Priest,
                                "ROGUE" => self.playerClass = ClassEnum::Rogue,
                                "SHAMAN" => self.playerClass = ClassEnum::Shaman,
                                "WARLOCK" => self.playerClass = ClassEnum::Warlock,
                                "WARRIOR" => self.playerClass = ClassEnum::Warrior,
                                _ => println!("Unknown playerClass found!: {}", s),
                            }
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "cardClass" => {
                    match *value {
                        Value::String(ref s) => {
                            match s.as_ref() {
                                "NEUTRAL" => self.cardClass = ClassEnum::Neutral,
                                "DRUID" => self.cardClass = ClassEnum::Druid,
                                "HUNTER" => self.cardClass = ClassEnum::Hunter,
                                "MAGE" => self.cardClass = ClassEnum::Mage,
                                "PALADIN" => self.cardClass = ClassEnum::Paladin,
                                "PRIEST" => self.cardClass = ClassEnum::Priest,
                                "ROGUE" => self.cardClass = ClassEnum::Rogue,
                                "SHAMAN" => self.cardClass = ClassEnum::Shaman,
                                "WARLOCK" => self.cardClass = ClassEnum::Warlock,
                                "WARRIOR" => self.cardClass = ClassEnum::Warrior,
                                _ => println!("Unknown cardClass found!: {}", s),
                            }
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "rarity" => {
                    match *value {
                        Value::String(ref s) => {
                            match s.as_ref() {
                                "COMMON" => self.rarity = RarityEnum::Common,
                                "RARE" => self.rarity = RarityEnum::Rare,
                                "EPIC" => self.rarity = RarityEnum::Epic,
                                "LEGENDARY" => self.rarity = RarityEnum::Legendary,
                                "FREE" => self.rarity = RarityEnum::Free,
                                _ => println!("Unknown rarity found!: {}", s),
                            }
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "type" => {
                    match *value {
                        Value::String(ref s) => {
                            match s.as_ref() {
                                "MINION" => self.theType = TypeEnum::Minion,
                                "WEAPON" => self.theType = TypeEnum::Weapon,
                                "SPELL" => self.theType = TypeEnum::Spell,
                                "HERO" => self.theType = TypeEnum::Hero,
                                _ => println!("Unknown type found!: {}", s),
                            }
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "faction" => {
                    match *value {
                        Value::String(ref s) => {
                            match s.as_ref() {
                                "ALLIANCE" => self.faction = FactionEnum::Alliance,
                                "HORDE" => self.faction = FactionEnum::Horde,
                                _ => println!("Unknown faction found!: {}", s),
                            }
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "artist" => {
                    match *value {
                        Value::String(ref s) => {
                            self.artist = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "text" => {
                    match *value {
                        Value::String(ref s) => {
                            self.text = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "set" => {
                    match *value {
                        Value::String(ref s) => {
                            self.set = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "artist" => {
                    match *value {
                        Value::String(ref s) => {
                            self.artist = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "flavor" => {
                    match *value {
                        Value::String(ref s) => {
                            self.flavor = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "howToEarn" => {
                    match *value {
                        Value::String(ref s) => {
                            self.howToEarn = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "howToEarnGolden" => {
                    match *value {
                        Value::String(ref s) => {
                            self.howToEarnGolden = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "targetingArrowText" => {
                    match *value {
                        Value::String(ref s) => {
                            self.targetingArrowText = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "race" => {
                    match *value {
                        Value::String(ref s) => {
                            self.race = s.clone();
                        }
                        _ => println!("Should be string but isn't"),
                    }
                }
                "attack" => {
                    match *value {
                        Value::Number(ref n) => {
                            match n.as_i64() {
                                Some(num) => self.attack = num,
                                None => println!("Should be an i64 but isn't"),
                            }
                        }
                        _ => println!("Should be a number but isn't"),
                    }
                }
                "cost" => {
                    match *value {
                        Value::Number(ref n) => {
                            match n.as_i64() {
                                Some(num) => self.cost = num,
                                None => println!("Should be an i64 but isn't"),
                            }
                        }
                        _ => println!("Should be a number but isn't"),
                    }
                }
                "dbfId" => {
                    match *value {
                        Value::Number(ref n) => {
                            match n.as_i64() {
                                Some(num) => self.dbfId = num,
                                None => println!("Should be an i64 but isn't"),
                            }
                        }
                        _ => println!("Should be a number but isn't"),
                    }
                }
                "health" => {
                    match *value {
                        Value::Number(ref n) => {
                            match n.as_i64() {
                                Some(num) => self.health = num,
                                None => println!("Should be an i64 but isn't"),
                            }
                        }
                        _ => println!("Should be a number but isn't"),
                    }
                }
                "overload" => {
                    match *value {
                        Value::Number(ref n) => {
                            match n.as_i64() {
                                Some(num) => self.overload = num,
                                None => println!("Should be an i64 but isn't"),
                            }
                        }
                        _ => println!("Should be a number but isn't"),
                    }
                }
                "spellDamage" => {
                    match *value {
                        Value::Number(ref n) => {
                            match n.as_i64() {
                                Some(num) => self.spellDamage = num,
                                None => println!("Should be an i64 but isn't"),
                            }
                        }
                        _ => println!("Should be a number but isn't"),
                    }
                }
                "durability" => {
                    match *value {
                        Value::Number(ref n) => {
                            match n.as_i64() {
                                Some(num) => self.durability = num,
                                None => println!("Should be an i64 but isn't"),
                            }
                        }
                        _ => println!("Should be a number but isn't"),
                    }
                }
                "collectible" => {
                    match *value {
                        Value::Bool(b) => self.collectible = b,
                        _ => println!("Should be string but isn't"),
                    }
                }
                "elite" => {
                    match *value {
                        Value::Bool(b) => self.elite = b,
                        _ => println!("Should be string but isn't"),
                    }
                }
                _ => println!("found unknown key: {}", key),
            }
        }
    }
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

fn parse_values_to_cards(value: &Value) -> Result<Vec<Card>, String> {
    let mut cards = Vec::new();
    match *value {
        //outermost value must be an array
        Value::Array(ref a) => {
            for val in a {
                let result = parse_card(val);
                match result {
                    Ok(card) => cards.push(card),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        _ => println!("Expected an array"),
    }
    Ok(cards)
}

fn parse_card(value: &Value) -> Result<Card, String> {
    let mut card = Card {
        name: "".into(),
        id: "".into(),
        json: "".into(),
        playerClass: ClassEnum::Unknown,
        cardClass: ClassEnum::Unknown,
        rarity: RarityEnum::Unknown,
        theType: TypeEnum::Unknown,
        faction: FactionEnum::Unknown,
        artist: "".into(),
        text: "".into(),
        set: "".into(),
        flavor: "".into(),
        howToEarnGolden: "".into(),
        targetingArrowText: "".into(),
        howToEarn: "".into(),
        race: "".into(),
        attack: 0,
        cost: 0,
        dbfId: 0,
        health: 0,
        overload: 0,
        spellDamage: 0,
        durability: 0,
        collectible: false,
        elite: false,
    };
    match *value {
        Value::Object(ref o) => {
            let pretty_text_result = serde_json::to_string_pretty(&o);
            match pretty_text_result {
                Ok(pretty_text) => {
                    card.internalize_json(o);
                    card.json = pretty_text;
                }
                Err(e) => println!("Couldn't pretty print card text: {}", e),
            }
        }
        _ => println!("Expected an object"),
    }
    Ok(card)
}

fn main() {
    let json_as_string_result = load_json_from_file("data/cards.collectible.json");
    match json_as_string_result {
        Ok(json_as_string) => {
            let json_as_value_result = parse_json_to_values(&json_as_string);
            match json_as_value_result {
                Ok(json_as_value) => {
                    let cards_result = parse_values_to_cards(&json_as_value);
                    match cards_result {
                        Ok(cards) => {
                            println!("Loaded {} cards", cards.len());
                            let card = &cards[0];
                            println!("Sample card: {} {}", card.name, card.json);
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(e) => println!("Couldn't parse file to Values: {}", e),
            }
        }
        Err(e) => println!("Error: Couldn't load file: {}", e),
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



/*
fn parse_values_to_cards(value: &Value) -> Result<Vec<Card>, String> {
    let cards = Vec::new();
    match *value {
        Value::Bool(ref b) => println!("found bool: {}", b),
        Value::Number(ref n) => {
            match n.as_i64() {
                Some(num) => println!("found i64: {}", num),
                None => match value.as_u64() {
                    Some(num) => println!("found u64: {}", num),
                    None => match value.as_f64() {
                        Some(num) => println!("found f64: {}", num),
                        None => println!("should be a number but isn't"),
                    }
                }
            }
        },
        Value::String(ref s) => {
            println!("found string: {}", s);
        },
        Value::Array(ref a) => {
            println!("found array");
            for (_, value) in a.iter().enumerate() {
                println!("found array element, recuring");
                parse_values_to_cards(value);
            }
        },
        Value::Object(ref o) => {
            println!("found array");
            for (name, value) in o {
                println!("found object: \"{}\", recursing", name);
                parse_values_to_cards(value);
            }
        },
        Value::Null => println!("found null"),
    }
    Ok(cards)
}
*/
