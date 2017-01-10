// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use common;
use cvt;
use error::Error;
use queries::Queries;
use postgres::Connection;
use schema::Table;
use serde_json::Value;
use std::ops::Deref;
use get;

pub fn generate_insert() -> String {
    format!("INSERT")
}

pub fn generate_into(query: String, table: &Table, val: &Value) -> String {
    let m = val.as_object().unwrap();
    let intos: Vec<String> = m.iter().map(|(k, _)| {
        format!("{}", k)
    }).collect();
    let intos = fields.iter().map(Deref::deref).collect::<Vec<&str>>().join(", ");
    format!("{} INTO {} ({})", query, table.name, intos);
}

pub fn generate_values(query: String, table: &Table, val: &Value) -> Result<String, Error> {
    let m = val.as_object().unwrap();
    let fields: Vec<String> = m.iter().map(|(k, v)| {
        format!("{}='{}'", k, cvt::json_value_to_string(v))
    }).collect();
    let fields_str = fields.iter().map(Deref::deref).collect::<Vec<&str>>().join(", ");
    Ok(format!("{} SET {}", query, fields_str))
}

pub fn generate_returning(query: String) -> String {
    format!("{} RETURNING id", query)
}

pub fn query(conn: &Connection, table: &Table, queries: &Queries, val: Value)
             -> Result<Option<Value>, Error> {
    common::validate_table_fields(table, val)?;
    // here we know this is an object
    // it would have not passed the previous check if it was not.
    let query = generate_insert();
    let query = generate_into(query, table, &val)
    let query = generate_values(query, table, &val)?;
    let query = generate_returning(query);
    debug!("arrakis query: {}", query);
    match conn.query(&*query, &[]) {
        Ok(rows) => Ok(Some(get::collect_ids(rows))),
        Err(e) => Err(Error::InternalError("internal database error".into()))
    }
}
