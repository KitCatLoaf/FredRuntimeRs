use mlua::{Lua, Result, Table};
use std::io::{self, Write};

pub fn register(lua: &Lua, fred_table: &Table) -> Result<()> {
    let color_fn = lua.create_function(|_, (text, color): (String, Option<String>)| {
        let color_name = color.unwrap_or_else(|| "white".to_string());
        
        let code = match color_name.to_lowercase().as_str() {
            "red" => "\x1b[31m",
            "green" => "\x1b[32m",
            "yellow" => "\x1b[33m",
            "blue" => "\x1b[34m",
            "magenta" | "purple" => "\x1b[35m",
            "cyan" => "\x1b[36m",
            "white" => "\x1b[37m",
            _ => "",
        };

        let result = format!("{}{}\x1b[0m", code, text);
        Ok(result)
    })?;
    fred_table.set("color", color_fn)?;

    let style_fn = lua.create_function(|_, (text, style): (String, Option<String>)| {
        let style_name = style.unwrap_or_else(|| "none".to_string());

        let code = match style_name.to_lowercase().as_str() {
            "bold" => "\x1b[1m",
            "dim" => "\x1b[2m",
            "italic" => "\x1b[3m",
            "underline" => "\x1b[4m",
            _ => "",
        };

        let result = format!("{}{}\x1b[0m", code, text);
        Ok(result)
    })?;
    fred_table.set("style", style_fn)?;

    let read_fn = lua.create_function(|_, prompt: Option<String> | {
        if let Some(p) = prompt {
            print!("{}", p);
            let _ = io::stdout().flush();
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_default();
        Ok(input.trim_end().to_string())
    })?;
    fred_table.set("read", read_fn)?;

    Ok(())
}