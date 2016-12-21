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

/*
UPDATE Customers
SET City='Hamburg'
WHERE CustomerID=1;
 */

pub fn generate_update(table: &Table) -> String {
    format!("UPDATE {}", table.name)
}

pub fn generate_set(query: String, table: &Table, val: &Value) -> Result<String, Error> {
    let fields_str = String::new();
    common::validate_table_fields(table, val)?;
    // here we know this is an object
    // it would have not passed the previous check if it was not.
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
    let query = generate_update(table);
    let query = generate_set(query, table, &val)?;
    let query = common::generate_where(query, table, queries)?;
    let query = generate_returning(query);
    debug!("query is: {}", query);
    let columns: Vec<String> = table.columns.iter().map(|(s, _)| s.to_string()).collect();
    match conn.query(&*query, &[]) {
        Ok(rows) => Ok(Some(get::collect_ids(rows))),
        Err(e) => Err(Error::InternalError("internal database error".into()))
    }
}
