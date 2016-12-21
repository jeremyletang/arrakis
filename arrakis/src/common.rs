// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use error::Error;
use queries::{FetchQueries, Queries};
use schema::Table;
use serde_json::Value;

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

pub fn validate_table_fields(table: &Table, val: &Value) -> Result<(), Error> {
    match val {
        &Value::Object(ref m) => {
            // TODO(JEREMY): ensure the types match with the db types
            for (k, _) in m {
                if !table.columns.contains_key(k) {
                    return Err(Error::UnknowColumn(k.clone(), table.name.clone()));
                }
            }
            Ok(())
        },
        _ => Err(Error::InvalidInputError("expect json object as top level value".into()))
    }
}
