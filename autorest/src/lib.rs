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
pub mod error;
pub mod get;
pub mod infer_schema;
pub mod method;
pub mod queries;
pub mod schema;

use error::Error;
use infer_schema::infer_schema;
use queries::Queries;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use schema::Table;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error as StdError;

pub use postgres::params::{
    ConnectParams, IntoConnectParams, UserInfo, ConnectTarget};

pub struct AutoRest {
    conn: r2d2::Pool<PostgresConnectionManager>,
    tables: HashMap<String, Table>,
}

impl AutoRest {
    pub fn new<P>(params: P) -> Result<AutoRest, String>
        where P: IntoConnectParams {
        let config = r2d2::Config::default();
        let manager = match PostgresConnectionManager::new(params, TlsMode::None) {
            Ok(m) => m,
            Err(e) => return Err(format!("{}, {}", e.description(), e.cause().unwrap()))
        };
        let pool = r2d2::Pool::new(config, manager).unwrap();
        let tables = infer_schema(&*pool.get().unwrap());
        Ok(AutoRest {
            conn: pool,
            tables: tables,
        })
    }

    pub fn get(&self, model: &str, queries: &Queries) -> Result<Value, Error> {
        if !self.tables.contains_key(model) {
            return Err(Error::UnknowModel(model.into()));
        }
        return get::query(&*(self.conn.get().unwrap()), self.tables.get(model).unwrap(), queries);
    }

    pub fn post(&self, model: &str, queries: &Queries, body: String)
                -> Result<Value, Error> {
        return Ok(Value::Bool(true));
    }

    pub fn put(&self, model: &str, queries: &Queries, body: String)
               -> Result<Value, Error> {
        return Ok(Value::Bool(true));
    }

    pub fn patch(&self, model: &str, queries: &Queries, body: String)
                 -> Result<Value, Error> {
        return Ok(Value::Bool(true));
    }

    pub fn delete(&self, model: &str, queries: &Queries)
                  -> Result<Value, Error> {
        return Ok(Value::Bool(true));
    }

}
