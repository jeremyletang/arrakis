// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

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
    Equal(String),
    GreaterThenEqual(String),
    GreaterThan(String),
    LesserThanEqual(String),
    LesserThan(String),
    NotEqual(String),
    Like(String),
    ILike(String),
    In(Vec<String>),
    NotIn(Vec<String>),
    Is(Is),
    IsNot(Is),
    Not(Box<Filter>),
}

impl Filter {
    pub fn new() -> () {}
}
