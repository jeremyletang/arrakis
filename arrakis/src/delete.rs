// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use common;
use error::Error;
use queries::Queries;
use postgres::Connection;
use schema::Table;

pub fn generate_delete() -> String {
    return "DELETE".into();
}

pub fn query(conn: &Connection, table: &Table, queries: &Queries)
             -> Result<(), Error> {
    let query = generate_delete();
    let query = common::generate_from(query, &*table.name);
    let query = common::generate_where(query, table, queries)?;
    debug!("query is: {}", query);
    match conn.query(&*query, &[]) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::InternalError("internal database error".into()))
    }
}
