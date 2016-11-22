// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_json;

pub mod cvt;
pub mod infer_schema;
pub mod schema;

use infer_schema::infer_schema;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use schema::Table;
use std::collections::HashMap;
use std::error::Error;

pub use postgres::params::{
    ConnectParams, IntoConnectParams, UserInfo, ConnectTarget};

pub struct AutoRest {
    conn: r2d2::Pool<PostgresConnectionManager>,
    database: String,
    tables: HashMap<String, Table>,
}

impl AutoRest {
    pub fn new<P, S>(params: P, database: S) -> Result<AutoRest, String>
        where P: IntoConnectParams,
              S: Into<String> {
        let database = database.into();
        let config = r2d2::Config::default();
        let manager = match PostgresConnectionManager::new(params, TlsMode::None) {
            Ok(m) => m,
            Err(e) => return Err(format!("{}, {}", e.description(), e.cause().unwrap()))
        };
        let pool = r2d2::Pool::new(config, manager).unwrap();
        let tables = infer_schema(&*pool.get().unwrap(), &*database);
        Ok(AutoRest {
            conn: pool,
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
