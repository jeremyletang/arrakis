// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate postgres;

use postgres::{Connection, TlsMode};
use std::error::Error;
use std::collections::HashMap;
use postgres::rows::Row;

pub use postgres::params::{ConnectParams, IntoConnectParams, UserInfo,
                           ConnectTarget};

#[derive(Debug)]
pub struct Column {
    name: String,
    default: Option<String>,
    is_nullable: bool,
    data_type: String,
    character_maximum_length: Option<i32>,
    is_updatable: bool,
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: HashMap<String, Column>,
}

pub struct AutoRest {
    conn: Connection,
    database: String,
    tables: HashMap<String, Table>,
}

const infer_schema_query: &'static str =
    "select table_name, column_name, column_default, is_nullable, udt_name, \
     character_maximum_length, is_updatable
     FROM INFORMATION_SCHEMA.COLUMNS \
     WHERE table_schema NOT IN ('information_schema', 'pg_catalog')";

fn as_bool(s: String) -> bool {
    match &*s {
        "YES" => true,
        "NO" => false,
        _ => unreachable!(),
    }
}

fn get_column(row: &Row) -> Column {
    Column {
        name: row.get(1),
        default: row.get(2),
        is_nullable: as_bool(row.get(3)),
        data_type: row.get(4),
        character_maximum_length: row.get(5),
        is_updatable: as_bool(row.get(6)),
    }
}

fn infer_schema(conn: &Connection, database: &str) -> HashMap<String, Table> {
    let mut tables: HashMap<String, Table> = HashMap::new();
    for row in &conn.query(&*infer_schema_query, &[]).unwrap() {
        let table_name: String = row.get(0);
        if tables.get(&table_name).is_none() {
            tables.insert(table_name.clone(),
                          Table{name: table_name.clone(), columns: HashMap::new()});
        }
        tables.get_mut(&table_name).unwrap().columns.insert(row.get(1), get_column(&row));
    }
    return tables;
}

impl AutoRest {
    pub fn new<P, S>(params: P, database: S) -> Result<AutoRest, String>
        where P: IntoConnectParams,
              S: Into<String> {
        let database = database.into();
        let conn = match Connection::connect(params, TlsMode::None) {
            Ok(c) => c,
            Err(e) => return Err(format!("{}, {}", e.description(), e.cause().unwrap()))
        };
        let tables = infer_schema(&conn, &*database);
        Ok(AutoRest {
            conn: conn,
            database: database,
            tables: tables,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
