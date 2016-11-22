// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cvt;
use error::Error;
use queries::{QueriesFilter, Queries};
use postgres::Connection;
use postgres::rows::Rows;
use schema::Table;
use serde_json::Value as JsonValue;
use serde_json::Map as JsonMap;

pub fn generate_select(table: &Table, queries: &Queries) -> (String, Vec<String>) {
    let mut partial: String = "SELECT".into();
    let columns: Vec<String> = match queries.select() {
        Some(columns) => columns.iter().map(|v| v.to_string()).collect(),
        None => table.columns.iter().map(|(k, _)| k.clone()).collect()
    };
    let mut first = true;
    for v in &columns {
        if first {
            partial += &*format!(" {}", v);
            first = false;
        } else {
            partial += &*format!(", {}", v);
        }
    }

    return (partial, columns);
}

pub fn generate_from(table_name: &str) -> String {
    format!("FROM {}", table_name)
}

pub fn collect_row_to_json<'stmt>(columns: Vec<String>, table: &Table, rows: Rows<'stmt>)
                                  -> JsonValue {
    let mut arr = vec![];
    for r in &rows {
        let mut map = JsonMap::new();
        let mut i = 0;
        while i != columns.len() {
            let col = table.columns.get(&columns[i]).unwrap();
            let val = cvt::row_field_to_json_value(&r, i, col.is_nullable, col.data_type.clone());
            map.insert(columns[i].clone(), val);
            i += 1;
        }
        let val = JsonValue::Object(map);
        arr.push(val);
    }

    return JsonValue::Array(arr);
}

fn validate_columns(table: &Table, columns: &Vec<String>) -> Option<Error> {
    for c in columns {
        if !table.columns.contains_key(c) {
            return Some(Error::UnknowColumn(c.to_string(), table.name.clone()));
        }
    }
    return None;
}

pub fn query(conn: &Connection, table: &Table, queries: &Queries)
             -> Result<JsonValue, Error> {
    let (select_partial, columns) = generate_select(table, queries);
    // ensure that possible user specified select column exists
    if let Some(e) = validate_columns(table, &columns) {
        return Err(e);
    }
    let from_partial = generate_from(&*table.name);
    let query = format!("{} {}", select_partial, from_partial);
    println!("query is: {}", query);
    match conn.query(&*query, &[]) {
        Ok(rows) => Ok(collect_row_to_json(columns, table, rows)),
        Err(e) => Err(Error::InternalError("internal database error".into()))
    }
}
