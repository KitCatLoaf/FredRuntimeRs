use mlua::{Lua, MultiValue, Result, Table, Value};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Duration;

pub fn register(lua: &Lua, fred_table: &Table) -> Result<()> {
    let http_table = lua.create_table()?;

    let get_fn = lua.create_function(|lua, (url, headers): (String, Option<HashMap<String, String>>)| {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(mlua::Error::external)?;

        let mut request = client.get(&url);

        if let Some(h) = headers {
            for (k, v) in h {
                request = request.header(&k, &v);
            }
        }

        match request.send() {
            Ok(response) => {
                let status = response.status().as_u16();
                match response.text() {
                    Ok(body) => Ok(MultiValue::from_vec(vec![
                        Value::String(lua.create_string(&body)?),
                        Value::Integer(status as i64),
                    ])),
                    Err(e) => Ok(MultiValue::from_vec(vec![
                        Value::Nil,
                        Value::String(lua.create_string(&e.to_string())?),
                    ])),
                }
            }
            Err(e) => Ok(MultiValue::from_vec(vec![
                Value::Nil,
                Value::String(lua.create_string(&e.to_string())?),
            ])),
        }
    })?;
    http_table.set("get", get_fn)?;
    let post_fn = lua.create_function(|lua, (url, body, headers): (String, String, Option<HashMap<String, String>>)| {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(mlua::Error::external)?;

        let mut request = client.post(&url).body(body);

        if let Some(h) = headers {
            for (k, v) in h {
                request = request.header(&k, &v);
            }
        }

        match request.send() {
            Ok(response) => {
                let status = response.status().as_u16();
                match response.text() {
                    Ok(res_body) => Ok(MultiValue::from_vec(vec![
                        Value::String(lua.create_string(&res_body)?),
                        Value::Integer(status as i64),
                    ])),
                    Err(e) => Ok(MultiValue::from_vec(vec![
                        Value::Nil,
                        Value::String(lua.create_string(&e.to_string())?),
                    ])),
                }
            }
            Err(e) => Ok(MultiValue::from_vec(vec![
                Value::Nil,
                Value::String(lua.create_string(&e.to_string())?),
            ])),
        }
    })?;
    http_table.set("post", post_fn)?;

    fred_table.set("http", http_table)?;

    Ok(())
}