use commands::{getCommands, CLIcommand};
use pokeapi::Pokeapi;
use std::io::{self, Write};

fn main() {
    loop {
        println!("Welcome to the Pokedex! ");
        print!("Pokedex > ");
        io::stdout().flush().expect("failed to flush");
        for map in map.values() {
            println!("Name: {}", map.name);
            println!("Description: {}", map.description);
            (map.callback)();
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let command_name = input.trim();
    }

    // println!("Commands:");
    // for map in map.values() {
    //     println!("Name: {}", map.name);
    //     println!("Description: {}", map.description);
    //     (map.callback)();
    // }
}
