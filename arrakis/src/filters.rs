// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use error::Error;
use std::str::FromStr;
use std::string::ToString;

pub enum IsKind {
    Null,
    True,
    False,
}

const NULL: &'static str = "NULL";
const TRUE: &'static str = "TRUE";
const FALSE: &'static str = "FALSE";

impl FromStr for IsKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::IsKind::*;
        match &*s.to_uppercase() {
            NULL => Ok(Null),
            TRUE => Ok(True),
            FALSE => Ok(False),
            _ => Err(()),
        }
    }
}

impl ToString for IsKind {
    fn to_string(&self) -> String {
        use self::IsKind::*;
        match *self {
            Null => NULL.into(),
            True => TRUE.into(),
            False => FALSE.into(),
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
    Is(String, IsKind),
    IsNot(String, IsKind),
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
const UNALLOWED_IS_FILTER_VALUE: &'static str =
    "invalid filter is / is not, allowed value are true, false, null.";

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
            ILIKE => Ok(ILike(name.to_string(), value.replace('*', "%"))),
            IN => Ok(In(
                name.to_string(),
                value.split(',').map(|s| s.into()).collect::<Vec<String>>())
            ),
            NOT_IN => Ok(NotIn(
                name.to_string(),
                value.split(',').map(|s| s.into()).collect::<Vec<String>>())
            ),
            IS => match IsKind::from_str(value) {
                Ok(k) => Ok(Is(name.to_string(), k)),
                Err(e) => Err(Error::InvalidFilterSyntax(UNALLOWED_IS_FILTER_VALUE.to_string())),
            },
            IS_NOT => match IsKind::from_str(value) {
                Ok(k) => Ok(IsNot(name.to_string(), k)),
                Err(e) => Err(Error::InvalidFilterSyntax(UNALLOWED_IS_FILTER_VALUE.to_string())),
            },
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
            &In(ref n, ref v) => fmt_in_filter(IN_SYM, n, v, table),
            &NotIn(ref n, ref v) => fmt_in_filter(NOT_IN_SYM, n, v, table),
            &Is(ref n, ref k) => fmt_is_filter(IS_SYM, n, k, table),
            &IsNot(ref n, ref k) => fmt_is_filter(IS_NOT_SYM, n, k, table),
            &Not(ref f) => fmt_not_filter(f, table),
        }
    }
}

fn fmt_is_filter(f: &str, name: &str, k: &IsKind, table: Option<&str>) -> String {
    match table {
        Some(t) => format!("{}.{} {} {}", t, name, f, k.to_string()),
        None => format!("{} {} {}", name, f, k.to_string()),
    }
}

fn fmt_like_filter(f: &str, name: &str, patt: &str, table: Option<&str>) -> String {
    match table {
        Some(t) => format!("{}.{} {} '{}'", t, name, f, patt),
        None => format!("{} {} '{}'", name, f, patt),
    }
}

fn fmt_in_filter(f: &str, name: &str, val: &Vec<String>, table: Option<&str>) -> String {
    let l = val.iter().map(|s| format!("'{}'", s)).collect::<Vec<String>>().join(", ");
    match table {
        Some(t) => format!("{}.{} {} ({})", t, name, f, l),
        None => format!("{} {} ({})", name, f, l),
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
