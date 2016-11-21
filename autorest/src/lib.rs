// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate postgres;
extern crate serde;
extern crate serde_json;

pub mod infer_schema;
pub mod schema;

use infer_schema::infer_schema;
use postgres::{Connection, TlsMode};
use schema::Table;
use std::collections::HashMap;
use std::error::Error;

pub use postgres::params::{
    ConnectParams, IntoConnectParams, UserInfo, ConnectTarget};

pub struct AutoRest {
    conn: Connection,
    database: String,
    tables: HashMap<String, Table>,
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
        for (k, v) in &tables {
            println!("{:?} => {:?}\n", k, v);
        }
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
