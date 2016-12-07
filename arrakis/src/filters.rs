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
    Not(Box<Filter>),
}

const EQ: &'static str = "EQ";
const GTE: &'static str = "GTE";
const GT: &'static str = "GT";
const LTE: &'static str = "LTE";
const LT: &'static str = "LT";
const NE: &'static str = "NE";
const LIKE: &'static str = "LIKE";
const ILIKE: &'static str = "ILIKE";
const IN: &'static str = "IN";
const NOT_IN: &'static str = "NOTIN";
const IS: &'static str = "IS";
const IS_NOT: &'static str = "ISNOT";
const NOT: &'static str = "NOT";

const EQ_SYM: &'static str = "=";
const GTE_SYM: &'static str = ">=";
const GT_SYM: &'static str = ">";
const LTE_SYM: &'static str = "<=";
const LT_SYM: &'static str = "<";
const NE_SYM: &'static str = "!=";
const LIKE_SYM: &'static str = "LIKE";
const ILIKE_SYM: &'static str = "ILIKE";
const IN_SYM: &'static str = "IN";
const NOT_IN_SYM: &'static str = "NOT IN";
const IS_SYM: &'static str = "IS";
const IS_NOT_SYM: &'static str = "IS NOT";
const NOT_SYM: &'static str = "NOT";

const INVALID_SYNTAX_ERROR: &'static str =
    "invalid filter syntax, should be a filter and a value at least, separated by a dot";
const EMPTY_VALUE_ERROR: &'static str =
    "invalid filter, value of a filter cannot be empty";

impl Filter {
    pub fn new(name: &str, value: &str) -> Result<Filter, Error> {
        use self::Filter::*;
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
        match &*filter.to_uppercase() {
            EQ => Ok(Equal(name.to_string(), value.to_string())),
            GTE => Ok(GreaterThanEqual(name.to_string(), value.to_string())),
            GT => Ok(GreaterThan(name.to_string(), value.to_string())),
            LTE => Ok(LesserThanEqual(name.to_string(), value.to_string())),
            LT => Ok(LesserThan(name.to_string(), value.to_string())),
            NE => Ok(NotEqual(name.to_string(), value.to_string())),
            LIKE => Ok(Like(name.to_string(), value.replace('*', "%"))),
            ILIKE => Ok(Like(name.to_string(), value.replace('*', "%"))),
            IN => Ok(In(
                name.to_string(),
                value.split(',').map(|s| s.into()).collect::<Vec<String>>())
            ),
            NOT_IN => Ok(NotIn(
                name.to_string(),
                value.split(',').map(|s| s.into()).collect::<Vec<String>>())
            ),
            NOT => match Filter::new(name, value) {
                Ok(f) => Ok(Not(Box::new(f))),
                Err(e) => Err(e),
            },
            _ => Err(Error::InvalidFilter(filter.into())),
        }
    }

    pub fn to_string(&self, table: Option<&str>) -> String {
        use self::Filter::*;
        match self {
            &Equal(ref n, ref v) => fmt_basic_filter(EQ_SYM, n , v, table),
            &GreaterThanEqual(ref n, ref v) => fmt_basic_filter(GTE_SYM, n , v, table),
            &GreaterThan(ref n, ref v) => fmt_basic_filter(GT_SYM, n , v, table),
            &LesserThanEqual(ref n, ref v) => fmt_basic_filter(LTE_SYM, n , v, table),
            &LesserThan(ref n, ref v) => fmt_basic_filter(LT_SYM, n , v, table),
            &NotEqual(ref n, ref v) => fmt_basic_filter(NE_SYM, n , v, table),
            &Like(ref n, ref patt) => fmt_like_filter(LIKE_SYM, n, patt, table),
            &ILike(ref n, ref patt) => fmt_like_filter(ILIKE_SYM, n, patt, table),
            &In(ref n, ref v) => fmt_in_filter(n, v, table),
            &NotIn(ref n, ref v) => fmt_notin_filter(n, v, table),
            &Not(ref f) => fmt_not_filter(f, table),
            _ => String::new(),
        }
    }
}

fn fmt_like_filter(f: &str, name: &str, patt: &str, table: Option<&str>) -> String {
    match table {
        Some(t) => format!("{}.{} {} '{}'", t, name, f, patt),
        None => format!("{} {} '{}'", name, f, patt),
    }
}

fn fmt_in_filter(name: &str, val: &Vec<String>, table: Option<&str>) -> String {
    let l = val.iter().map(|s| format!("'{}'", s)).collect::<Vec<String>>().join(", ");
    match table {
        Some(t) => format!("{}.{} {} ({})", t, name, IN_SYM, l),
        None => format!("{} {} ({})", name, IN_SYM, l),
    }
}

fn fmt_notin_filter(name: &str, val: &Vec<String>, table: Option<&str>) -> String {
    let l = val.iter().map(|s| format!("'{}'", s)).collect::<Vec<String>>().join(", ");
    match table {
        Some(t) => format!("{}.{} {} ({})", t, name, NOT_IN_SYM, l),
        None => format!("{} {} ({})", name, NOT_IN_SYM, l),
    }
}

fn fmt_not_filter(f: &Filter, table: Option<&str>) -> String {
    format!("{} ({})", NOT_SYM, f.to_string(table))
}

fn fmt_basic_filter(filter: &str, col: &str, val: &str, table: Option<&str>) -> String {
    match table {
        Some(t) => format!("{}.{} {} '{}'", t, col, filter, val),
        None =>  format!("{} {} '{}'", col, filter, val),
    }
}
