pub const FRED_VERSION: &str = "2.3-ALPHA";

use mlua::Lua;
use std::env;
use std::fs;
use std::process::exit;

mod library;

fn main() -> mlua::Result<()> {
    let args: Vec<String> = env::args().collect();
    let lua = Lua::new();

    library::setup(&lua)?;

    if args.len() > 1 {
        match args[1].as_str() {
            "--fred" => {
                lua.load(r#"
                    print(fred.color(fred.style("IT IS ME MY CHILD.", "bold"), "red"))
                "#).exec()?;
                exit(0);
            }
            "--pleh" => {
                lua.load(r#"
                    local lines = {
                        "",
                        fred.version(),
                        "",
                        "Usage:",
                        "  fred <file.frd>            Run Fred (.frd) file",
                        "  fred compile <file.lua>    Compile Lua to .frd",
                        "  fred -h, --help            Show this help menu",
                        "  fred -v, --version         Show version",
                        "  fred -l, --log             View the latest update log",
                        "",
                    }
                    for i = #lines, 1, -1 do
                        print(string.reverse(lines[i]))
                    end
                "#).exec()?;
                exit(0);
            }
            "-v" | "--version" => {
                lua.load(r#"print("\n" .. fred.version())"#).exec()?;
                exit(0);
            }
            "-l" | "--log" => {
                lua.load(r#"
                    print("\nUpdate Log:\n\n- Allow folder support for FileSystem lib\n- New VS Code helper extension")
                "#).exec()?;
                exit(0);
            }
            "-h" | "--help" => {
                println!("\nUsage:");
                println!("  fred <file.frd>            Run Fred (.frd) file");
                println!("  fred compile <file.lua>    Compile Lua to .frd");
                println!("  fred -h, --help            Show this help menu");
                println!("  fred -v, --version         Show version");
                println!("  fred -l, --log             View the latest update log\n");
                exit(0);
            }
            "compile" => {
                if args.len() > 2 {
                    let lua_file = &args[2];
                    let frd_file = lua_file.replace(".lua", ".frd");
                    if let Ok(content) = fs::read_to_string(lua_file) {
                        let _ = fs::write(&frd_file, content);
                        println!("Compiled {} -> {}", lua_file, frd_file);
                    } else {
                        eprintln!("Error: Could not read file '{}'", lua_file);
                    }
                } else {
                    eprintln!("Usage: fred compile <file.lua>");
                }
                exit(0);
            }
            path => {
                if let Err(e) = lua.load(fs::read_to_string(path).unwrap_or_default()).exec() {
                    eprintln!("Fred Runtime Error: {}", e);
                }
                exit(0);
            }
        }
    }

    println!("Fred Runtime {}. Use 'fred -h' for options.", FRED_VERSION);
    Ok(())
}