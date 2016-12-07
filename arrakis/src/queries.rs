// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use filters::Filter;
use ordering::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use error::Error;

pub const SELECT: &'static str = "select";
pub const LIMIT: &'static str = "limit";
pub const OFFSET: &'static str = "offset";
pub const ORDER: &'static str = "order";

pub type Queries<'r> = BTreeMap<&'r str, &'r str>;

pub trait FetchQueries {
    fn select(&self) -> Option<Vec<&str>>;
    fn limit(&self) -> Option<&str>;
    fn offset(&self) -> Option<&str>;
    fn order(&self) -> Option<Vec<Ordering>>;
    // map of column -> filter
    fn filters(&self) -> Result<BTreeMap<&str, Filter>, Error>;
}

pub fn queries_from_hashmap<'r>(m: &'r HashMap<String, Vec<String>>) -> Queries<'r> {
    m.iter().map(|(k, v)| (&**k, &*(v[0]))).collect::<Queries<'r>>()
}

impl<'r> FetchQueries for Queries<'r> {
    // this is really naive
    // do not handle foreign key for now.
    fn select(&self) -> Option<Vec<&str>> {
        match self.get(SELECT) {
            Some(ref val) => {
                Some(val.split(',').collect())
            },
            None => None
        }
    }

    fn limit(&self) -> Option<&str> {
        self.get(LIMIT).map(|val| *val)
    }

    fn offset(&self) -> Option<&str> {
        self.get(OFFSET).map(|val| *val)
    }

    fn order(&self) -> Option<Vec<Ordering>> {
        match self.get(ORDER) {
            Some(ref val) => {
                Some(val.split(',')
                     .collect::<Vec<&str>>().iter()
                     .filter_map(|ref s| Ordering::from_str(s).ok())
                     .collect::<Vec<Ordering>>())
            },
            None => None
        }
    }

    fn filters(&self) -> Result<BTreeMap<&str, Filter>, Error> {
        let mut filters = BTreeMap::new();
        for (k, v) in self.iter() {
            match *k {
                SELECT | LIMIT | OFFSET | ORDER => {},
                _ => {
                    match Filter::new(k, v) {
                        Ok(f) => { filters.insert(*k, f); },
                        Err(e) => return Err(e)
                    }
                }
            }
        }
        return Ok(filters);
    }
}
