// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;
use error::Error;

pub enum Is {
    Null,
    True,
    False,
}

impl FromStr for Is {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "NULL" => Ok(Is::Null),
            "TRUE" => Ok(Is::True),
            "FALSE" => Ok(Is::False),
            _ => Err(()),
        }
    }
}

pub enum Filter {
    Equal(String, String),
    GreaterThanEqual(String, String),
    GreaterThan(String, String),
    LesserThanEqual(String, String),
    LesserThan(String, String),
    NotEqual(String, String),
    Like(String, String),
    ILike(String, String),
    In(String, Vec<String>),
    NotIn(String, Vec<String>),
    Is(String, Is),
    IsNot(String, Is),
    Not(String, Box<Filter>),
}

const INVALID_SYNTAX_ERROR: &'static str =
    "invalid filter syntax, should be a filter and a value at least, separated by a dot";
const EMPTY_VALUE_ERROR: &'static str =
    "invalid filter, value of a filter cannot be empty";

impl Filter {
    pub fn new(name: &str, value: &str) -> Result<Filter, Error> {
        // first find .
        let (filter, value) = match value.find('.') {
            Some(pos) => {
                let (filter, value) = value.split_at(pos);
                (filter, &value[1..])
            },
            None => return Err(Error::InvalidFilterSyntax(INVALID_SYNTAX_ERROR.into()))
        };

        if value.is_empty() {
            return Err(Error::InvalidFilterSyntax(EMPTY_VALUE_ERROR.into()));
        }
        match &*filter.to_lowercase() {
            "eq" => Ok(Filter::Equal(name.to_string(), value.to_string())),
            "gte" => Ok(Filter::GreaterThanEqual(name.to_string(), value.to_string())),
            "gt" => Ok(Filter::GreaterThan(name.to_string(), value.to_string())),
            "lte" => Ok(Filter::LesserThanEqual(name.to_string(), value.to_string())),
            "lt" => Ok(Filter::LesserThan(name.to_string(), value.to_string())),
            "ne" => Ok(Filter::NotEqual(name.to_string(), value.to_string())),
            _ => Err(Error::InvalidFilter(filter.into())),
        }
    }

    pub fn to_string(&self, table: Option<&str>) -> String {
        match self {
            &Filter::Equal(ref n, ref v) => fmt_basic_filter("=", n , v, table),
            &Filter::GreaterThanEqual(ref n, ref v) => fmt_basic_filter(">=", n , v, table),
            &Filter::GreaterThan(ref n, ref v) => fmt_basic_filter(">", n , v, table),
            &Filter::LesserThanEqual(ref n, ref v) => fmt_basic_filter("<=", n , v, table),
            &Filter::LesserThan(ref n, ref v) => fmt_basic_filter("<", n , v, table),
            &Filter::NotEqual(ref n, ref v) => fmt_basic_filter("!=", n , v, table),
            _ => String::new(),
        }
    }
}

fn fmt_basic_filter(filter: &str, col: &str, val: &str, table: Option<&str>) -> String {
    match table {
        Some(t) => format!("{}.{} {} '{}'", t, col, filter, val),
        None =>  format!("{} {} '{}'", col, filter, val),
    }
}
