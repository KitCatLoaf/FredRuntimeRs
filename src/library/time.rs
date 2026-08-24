use chrono::Local;
use mlua::{Lua, Result, Table, Value};
use std::thread;
use std::time::{Duration};

pub fn register(lua: &Lua, fred_table: &Table) -> Result<()> {
    let time_fn = lua.create_function(|_, ()| {
        let now = Local::now().timestamp_millis() as f64 / 1000.0;
        Ok(now)
    })?;
    fred_table.set("time", time_fn)?;

    let sleep_fn = lua.create_function(|_, ms: u64| {
        thread::sleep(Duration::from_millis(ms));
        Ok(())
    })?;
    fred_table.set("sleep", sleep_fn)?;

    let date_fn = lua.create_function(|lua, mode: Option<String>| {
            let now = Local::now();

            let year = now.format("%Y").to_string().parse::<i32>().unwrap_or(0);
            let month = now.format("%m").to_string().parse::<u32>().unwrap_or(0);
            let day = now.format("%d").to_string().parse::<u32>().unwrap_or(0);
            let hour = now.format("%H").to_string().parse::<u32>().unwrap_or(0);
            let min = now.format("%M").to_string().parse::<u32>().unwrap_or(0);
            let sec = now.format("%S").to_string().parse::<u32>().unwrap_or(0);

            match mode.as_deref() {
                Some("*t") | Some("table") | None => {
                    let date_table = lua.create_table()?;
                    date_table.set("year", year)?;
                    date_table.set("month", month)?;
                    date_table.set("day", day)?;
                    date_table.set("hour", hour)?;
                    date_table.set("min", min)?;
                    date_table.set("sec", sec)?;
                    Ok(Value::Table(date_table))
                }
                Some(fmt) => {
                    let formatted = fmt
                        .replace("%Y", &format!("{:04}", year))
                        .replace("%m", &format!("{:02}", month))
                        .replace("%d", &format!("{:02}", day))
                        .replace("%H", &format!("{:02}", hour))
                        .replace("%M", &format!("{:02}", min))
                        .replace("%S", &format!("{:02}", sec))
                        .replace("%h", &hour.to_string());
                    Ok(Value::String(lua.create_string(&formatted)?))
                }
            }
        })?;
        fred_table.set("date", date_fn)?;

        Ok(())
    }