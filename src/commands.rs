use pokedex::Pokedex;
use std::collections::HashMap;

use crate::pokeapi::ResDestructure;

pub struct CLIcommand {
    name: String,
    description: String,
    callback: fn(),
}

impl CLIcommand {
    fn new(name: &str, description: &str, callback: fn()) -> Self {
        CLIcommand {
            name: name.to_string(),
            description: description.to_string(),
            callback,
        }
    }

    pub fn getCommands() -> HashMap<String, CLIcommand> {
        let mut map: HashMap<String, CLIcommand> = HashMap::new();
        map.insert(
            "help".to_string(),
            CLIcommand::new("help", "prints this help message", CLIcommand::help),
        );
        map.insert(
            "exit".to_string(),
            CLIcommand::new("exit", "exits the program", CLIcommand::exit),
        );
        map.insert(
            "map".to_string(),
            CLIcommand::new("map", "prints the map", CLIcommand::mapf),
        );
        map.insert(
            "mapb".to_string(),
            CLIcommand::new("mapb", "prints the map", CLIcommand::mapb),
        );

        map
    }

    pub fn help() {
        println!("help");
    }

    pub fn exit() {
        println!("exited program");
        std::process::exit(0);
    }

    pub fn mapf(cfg: ResDestructure) {}
}
