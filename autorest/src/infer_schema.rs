// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use postgres::Connection;
use postgres::rows::Row;
use schema::{Table, Column};
use postgres::types::Type;
use std::collections::{HashMap, BTreeMap};

const INFER_SCHEMA_QUERY: &'static str =
    "select table_name, column_name, column_default, is_nullable, udt_name, \
     character_maximum_length, is_updatable \
     FROM INFORMATION_SCHEMA.COLUMNS \
     WHERE table_schema NOT IN ('information_schema', 'pg_catalog')";

fn as_bool(s: String) -> bool {
    match &*s {
        "YES" => true,
        "NO" => false,
        _ => unreachable!(),
    }
}

fn get_column(row: &Row, ty: Type) -> Column {
    Column {
        name: row.get(1),
        default: row.get(2),
        is_nullable: as_bool(row.get(3)),
        data_type: ty,
        character_maximum_length: row.get(5),
        is_updatable: as_bool(row.get(6)),
    }
}

pub fn infer_schema(conn: &Connection) -> HashMap<String, Table> {
    let mut tables: HashMap<String, Table> = HashMap::new();
    for row in &conn.query(&*INFER_SCHEMA_QUERY, &[]).unwrap() {
        let table_name: String = row.get(0);
        if tables.get(&table_name).is_none() {
            tables.insert(table_name.clone(),
                          Table{name: table_name.clone(), columns: BTreeMap::new()});
        }
        let column_name: String = row.get(1);
        let ty_query = format!("SELECT atttypid FROM pg_attribute \
                                WHERE attrelid = '{}'::regclass and attname = '{}'",
                               table_name, column_name);
        let ty = Type::from_oid(conn.query(&*ty_query, &[]).unwrap().get(0).get(0)).unwrap();
        tables.get_mut(&table_name).unwrap().columns.insert(row.get(1), get_column(&row, ty));
    }
    return tables;
}
