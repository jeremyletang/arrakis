// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use postgres::types::{Oid, Type};

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub default: Option<String>,
    pub is_nullable: bool,
    pub data_type: Type,
    pub character_maximum_length: Option<i32>,
    pub is_updatable: bool,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: HashMap<String, Column>,
}
