use mlua::{Lua, MultiValue, Result, Table, Value};
use std::fs;
use std::path::Path;

pub fn register(lua: &Lua, fred_table: &Table) -> Result<()> {
    let fs_table = lua.create_table()?;

    let read_fn = lua.create_function(|lua, path: String| {
        match fs::read_to_string(&path) {
            Ok(content) => Ok(MultiValue::from_vec(vec![Value::String(
                lua.create_string(&content)?,
            )])),
            Err(e) => Ok(MultiValue::from_vec(vec![
                Value::Nil,
                Value::String(lua.create_string(&e.to_string())?),
            ])),
        }
    })?;
    fs_table.set("read", read_fn)?;

    let write_fn = lua.create_function(|lua, (path, content): (String, String)| {
        if let Some(parent) = Path::new(&path).parent() {
            let _ = fs::create_dir_all(parent);
        }

        match fs::write(&path, content) {
            Ok(_) => Ok(MultiValue::from_vec(vec![Value::Boolean(true)])),
            Err(e) => Ok(MultiValue::from_vec(vec![
                Value::Boolean(false),
                Value::String(lua.create_string(&e.to_string())?),
            ])),
        }
    })?;
    fs_table.set("write", write_fn)?;

    let exists_fn = lua.create_function(|_, path: String| {
        Ok(Path::new(&path).exists())
    })?;
    fs_table.set("exists", exists_fn)?;

    let copy_fn = lua.create_function(|lua, (src, dest): (String, String)| {
        if let Some(parent) = Path::new(&dest).parent() {
            let _ = fs::create_dir_all(parent);
        }

        match fs::copy(&src, &dest) {
            Ok(_) => Ok(MultiValue::from_vec(vec![Value::Boolean(true)])),
            Err(e) => Ok(MultiValue::from_vec(vec![
                Value::Boolean(false),
                Value::String(lua.create_string(&e.to_string())?),
            ])),
        }
    })?;
    fs_table.set("copy", copy_fn)?;

    let make_fn = lua.create_function(|_, path: String| {
        let result = fs::create_dir_all(&path).is_ok();
        Ok(result)
    })?;
    fs_table.set("make", make_fn)?;

    fred_table.set("fs", fs_table)?;

    Ok(())
}