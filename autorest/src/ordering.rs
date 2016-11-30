// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

pub enum Ordering {
    Asc(String),
    Desc(String)
}

impl FromStr for Ordering {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split('.').collect::<Vec<&str>>();
        match v.len() {
            1 => Ok(Ordering::Asc(v[0].into())),
            2 => {
                match &*v[1].to_uppercase() {
                    "ASC" => Ok(Ordering::Asc(v[0].into())),
                    "DESC" => Ok(Ordering::Desc(v[0].into())),
                    _ => Err(()),
                }
            },
            _ => Err(()),

        }
    }
}

pub fn to_string(o: &Ordering, prefix: Option<&str>) -> String {
    let s = match o {
        &Ordering::Asc(ref s) => format!("{} {}", s, "ASC"),
        &Ordering::Desc(ref s) => format!("{} {}", s, "DESC"),
    };
    prefix.map_or(s.clone(), |p| format!("{}.{}", p, s))
}
