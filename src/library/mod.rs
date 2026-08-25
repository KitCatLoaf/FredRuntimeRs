pub mod core;
pub mod time;
pub mod io;
pub mod fs;

use mlua::{Lua, Result};

pub fn setup(lua: &Lua) -> Result<()> {
    let fred_table = lua.create_table()?;
    core::register(lua, &fred_table)?;
    time::register(lua, &fred_table)?;
    io::register(lua, &fred_table)?;
    fs::register(lua, &fred_table)?;
    lua.globals().set("fred", fred_table)?;
    Ok(())
}