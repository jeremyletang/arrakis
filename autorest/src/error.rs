// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    InvalidFilter(String),
    InvalidType(String, String, String),
    UnknowModel(String),
    InternalError(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound(..) => "table not found",
            Error::InvalidFilter(..) => "invalid or unknown filter",
            Error::InvalidType(..) => "invalid type for column",
            Error::UnknowModel(..) => "unknow model",
            Error::InternalError(..) => "internal error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound(ref s) => write!(fmt, "model not found, {}", s),
            Error::InvalidFilter(ref s) => write!(fmt, "invalid or unknown filter '{}'", s),
            Error::InvalidType(ref col, ref expected, ref found) => {
                write!(fmt, "invalid type for column {}, expected {} found {}",
                       col, expected, found)
            },
            Error::UnknowModel(ref s) => write!(fmt, "table '{}' do not exist", s),
            Error::InternalError(ref s) => write!(fmt, "internal error, {}", s),
        }
    }
}
