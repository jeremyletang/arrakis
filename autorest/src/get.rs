// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cvt;
use error::Error;
use queries::{FetchQueries, Queries};
use ordering;
use postgres::Connection;
use postgres::rows::Rows;
use schema::Table;
use serde_json::Value as JsonValue;
use serde_json::Map as JsonMap;

pub fn generate_select(mut query: String, table: &Table, queries: &Queries)
                       -> Result<(String, Vec<String>), Error> {
    query += "SELECT ".into();
    let columns: Vec<String> = match queries.select() {
        Some(columns) => columns.iter().map(|v| v.to_string()).collect(),
        None => table.columns.iter().map(|(k, _)| k.clone()).collect()
    };

    query += &*columns.iter()
        .map(|ref s| format!("{}.{}", &*table.name, s))
        .collect::<Vec<String>>()
        .join(", ");

    // ensure that possible user specified select column exists
    if let Some(e) = validate_columns(table, &columns) {
        return Err(e);
    }

    return Ok((query, columns));
}

pub fn generate_from(query: String, table_name: &str) -> String {
    format!("{} FROM {}", query, table_name)
}

pub fn generate_where(mut query: String, table: &Table, queries: &Queries)
                      -> Result<String, Error> {
    let filters = queries.filters()?;
    if !filters.is_empty() {
        query = format!("{} WHERE ", query);
    }
    let mut filters_str = vec![];
    for (col, filter) in filters {
        if !table.columns.contains_key(col) {
            let estr = format!("column {} do not exist for table {}", col, table.name);
            return Err(Error::InvalidFilterSyntax(estr));
        }
        filters_str.push(filter.to_string(Some(&table.name)));
    }
    query += &*filters_str.iter().map(|s| &**s)
        .collect::<Vec<&str>>()
        .join("AND ");
    Ok(query)
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

pub fn generate_limit(query: String, queries: &Queries) -> Result<String, Error> {
    match queries.limit() {
        Some(limit) => {
            match limit.trim().parse::<u32>() {
                Ok(i) => Ok(format!("{} LIMIT {}", query, i)),
                Err(_) => Err(Error::InvalidFilterType("limit".into(), "u32".into()))
            }
        }
        None => Ok(query)
    }
}

pub fn generate_offset(query: String, queries: &Queries) -> Result<String, Error> {
    match queries.offset() {
        Some(offset) => {
            match offset.trim().parse::<u32>() {
                Ok(i) => Ok(format!("{} OFFSET {}", query, i)),
                Err(_) => Err(Error::InvalidFilterType("offset".into(), "u32".into()))
            }
        }
        None => Ok(query)
    }
}

pub fn generate_order(mut query: String, table: &Table, queries: &Queries) -> String {
    match queries.order() {
        Some(orders) => {
            if orders.len() > 0 { query = format!("{} {}", query, "ORDER BY "); }
            query += &*orders.iter()
                .map(|ref o| format!("{}", ordering::to_string(&o, Some(&*table.name))))
                .collect::<Vec<String>>()
                .join(", ");
            return query;
        },
        None => query,
    }
}

pub fn query(conn: &Connection, table: &Table, queries: &Queries)
             -> Result<JsonValue, Error> {
    let query = String::new();
    let (query, columns) = generate_select(query, table, queries)?;
    let query = generate_from(query, &*table.name);
    let query = generate_where(query, table, queries)?;
    let query = generate_order(query, table, queries);
    let query = generate_limit(query, queries)?;
    let query = generate_offset(query, queries)?;
    debug!("query is: {}", query);
    match conn.query(&*query, &[]) {
        Ok(rows) => Ok(collect_row_to_json(columns, table, rows)),
        Err(e) => Err(Error::InternalError("internal database error".into()))
    }
}
