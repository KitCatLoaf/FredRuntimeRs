pub mod core;

use mlua::{Lua, Result};

pub fn setup(lua: &Lua) -> Result<()> {
    let fred_table = lua.create_table()?;
    core::register(lua, &fred_table)?;
    lua.globals().set("fred", fred_table)?;
    Ok(())
}