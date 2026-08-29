use std::fs;
use std::path::Path;
use mlua::{Lua, Result, Table, Value};
use walkdir::WalkDir;

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}

pub fn register(lua: &Lua, fred_table: &Table) -> Result<()> {
    let fs_table = lua.create_table()?;

    let read_fn = lua.create_function(|lua, path_str: String| {
        let path = Path::new(&path_str);

        if path.is_dir() {
            let err_str = format!("Cannot read '{}': path is a directory", path_str);
            return Ok((Value::Nil, Value::String(lua.create_string(&err_str)?)));
        }

        match fs::read_to_string(path) {
            Ok(content) => Ok((Value::String(lua.create_string(&content)?), Value::Nil)),
            Err(e) => Ok((Value::Nil, Value::String(lua.create_string(&e.to_string())?))),
        }
    })?;
    fs_table.set("read", read_fn)?;

    let write_fn = lua.create_function(|lua, (path_str, content): (String, String)| {
        let path = Path::new(&path_str);

        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Ok((Value::Boolean(false), Value::String(lua.create_string(&e.to_string())?)));
            }
        }

        match fs::write(path, content) {
            Ok(_) => Ok((Value::Boolean(true), Value::Nil)),
            Err(e) => Ok((Value::Boolean(false), Value::String(lua.create_string(&e.to_string())?))),
        }
    })?;
    fs_table.set("write", write_fn)?;

    let exists_fn = lua.create_function(|_, path_str: String| {
        Ok(Path::new(&path_str).exists())
    })?;
    fs_table.set("exists", exists_fn)?;

    let copy_fn = lua.create_function(|lua, (src_str, dest_str): (String, String)| {
        let src = Path::new(&src_str);
        let dest = Path::new(&dest_str);

        if !src.exists() {
            let err_str = format!("Source path does not exist: {}", src_str);
            return Ok((Value::Boolean(false), Value::String(lua.create_string(&err_str)?)));
        }

        let res = if src.is_dir() {
            copy_dir_all(src, dest)
        } else {
            if let Some(parent) = dest.parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::copy(src, dest).map(|_| ())
        };

        match res {
            Ok(_) => Ok((Value::Boolean(true), Value::Nil)),
            Err(e) => Ok((Value::Boolean(false), Value::String(lua.create_string(&e.to_string())?))),
        }
    })?;
    fs_table.set("copy", copy_fn)?;

    let make_fn = lua.create_function(|lua, path_str: String| {
        match fs::create_dir_all(Path::new(&path_str)) {
            Ok(_) => Ok((Value::Boolean(true), Value::Nil)),
            Err(e) => Ok((Value::Boolean(false), Value::String(lua.create_string(&e.to_string())?))),
        }
    })?;
    fs_table.set("make", make_fn)?;

    let remove_fn = lua.create_function(|lua, (path_str, recursive): (String, Option<bool>)| {
        let path = Path::new(&path_str);

        if !path.exists() {
            let err_str = format!("Path does not exist: {}", path_str);
            return Ok((Value::Boolean(false), Value::String(lua.create_string(&err_str)?)));
        }

        let is_recursive = recursive.unwrap_or(true);
        let res = if path.is_dir() {
            if is_recursive {
                fs::remove_dir_all(path)
            } else {
                fs::remove_dir(path)
            }
        } else {
            fs::remove_file(path)
        };

        match res {
            Ok(_) => Ok((Value::Boolean(true), Value::Nil)),
            Err(e) => Ok((Value::Boolean(false), Value::String(lua.create_string(&e.to_string())?))),
        }
    })?;
    fs_table.set("remove", remove_fn)?;

    let list_fn = lua.create_function(|lua, (path_str, recursive): (String, Option<bool>)| {
        let path = Path::new(&path_str);

        if !path.is_dir() {
            let err_str = format!("Path is not a directory: {}", path_str);
            return Ok((Value::Nil, Value::String(lua.create_string(&err_str)?)));
        }

        let is_recursive = recursive.unwrap_or(false);
        let result_table = lua.create_table()?;
        let mut idx = 1;

        if is_recursive {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.path() == path {
                    continue;
                }

                let entry_table = lua.create_table()?;
                entry_table.set("path", entry.path().to_string_lossy().to_string())?;
                entry_table.set("name", entry.file_name().to_string_lossy().to_string())?;
                entry_table.set("is_dir", entry.file_type().is_dir())?;

                result_table.set(idx, entry_table)?;
                idx += 1;
            }
        } else {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let entry_table = lua.create_table()?;
                    entry_table.set("path", entry.path().to_string_lossy().to_string())?;
                    entry_table.set("name", entry.file_name().to_string_lossy().to_string())?;
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    entry_table.set("is_dir", is_dir)?;

                    result_table.set(idx, entry_table)?;
                    idx += 1;
                }
            }
        }

        Ok((Value::Table(result_table), Value::Nil))
    })?;
    fs_table.set("list", list_fn)?;

    fred_table.set("fs", fs_table)?;

    Ok(())
}