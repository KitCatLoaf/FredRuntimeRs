use std::env;
use mlua::{Lua, Result, Table};

pub fn register(lua: &Lua, fred_table: &Table) -> Result<()> {
    let version_fn = lua.create_function(|_, ()| {
        Ok(crate::FRED_VERSION)
    })?;
    fred_table.set("version", version_fn)?;

    let platform_fn = lua.create_function(|_, ()| {
        Ok(env::consts::OS)
    })?;
    fred_table.set("platform", platform_fn)?;

    Ok(())
}