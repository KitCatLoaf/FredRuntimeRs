mod library;

use mlua::Lua;
use std::env;
use std::fs;

pub const FRED_VERSION: &str = "2.0-ALPHA";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Fred Runtime v{}", FRED_VERSION);
        println!("Run 'fred -h' for usage.");
        return;
    }

    let command = args[1].to_lowercase();
    match command.as_str() {
        "-v" | "--version" => {
            println!("Fred Runtime v{}", FRED_VERSION);
        }
        "-h" | "--help" => {
            println!("USAGE:");
            println!("  fred <file.frd>   Run Fred script");
            println!("  fred -v           Show Version");
        }
        _ => {
            let filename = &args[1];
            match fs::read_to_string(filename) {
                Ok(code) => {
                    let lua = Lua::new();

                    if let Err(err) = library::setup(&lua) {
                        eprintln!("Failed to setup Fred Runtime Globals: {}", err);
                    } else if let Err(err) = lua.load(&code).exec() {
                        eprintln!("Fred Runtime Error: {}", err);
                    }
                }
                Err(err) => {
                    eprintln!("COULD NOT READ '{}': {}", filename, err);
                }
            }
        }
    }
}