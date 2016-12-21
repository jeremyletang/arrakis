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
    InvalidFilterSyntax(String),
    InvalidFilterType(String, String),
    InvalidColumnType(String, String, String),
    UnknowModel(String),
    UnknowColumn(String, String),
    InvalidInputError(String),
    InternalError(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound(..) => "table not found",
            Error::InvalidFilter(..) => "invalid or unknown filter",
            Error::InvalidFilterSyntax(..) => "invalid filter syntax",
            Error::InvalidFilterType(..) => "invalid type for column",
            Error::InvalidColumnType(..) => "invalid type for column",
            Error::UnknowModel(..) => "unknow model",
            Error::UnknowColumn(..) => "unknow column",
            Error::InvalidInputError(..) => "invalid input",
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
            Error::NotFound(ref s) => write!(fmt, "table not found '{}'", s),
            Error::InvalidFilter(ref s) => write!(fmt, "invalid or unknown filter '{}'", s),
            Error::InvalidFilterSyntax(ref s) => write!(fmt, "{}", s),
            Error::InvalidFilterType(ref col, ref expected) =>
                write!(fmt, "invalid type for filter '{}, expected '{}'", col, expected),
            Error::InvalidColumnType(ref col, ref expected, ref found) => {
                write!(fmt, "invalid type for column '{}', expected '{}' found '{}'",
                       col, expected, found)
            },
            Error::UnknowModel(ref s) => write!(fmt, "table '{}' do not exist", s),
            Error::UnknowColumn(ref c, ref m) =>
                write!(fmt, "column '{}' do not exist for table '{}'", c, m),
            Error::InvalidInputError(ref s) => write!(fmt, "invalid input: {}", s),
            Error::InternalError(ref s) => write!(fmt, "internal error, {}", s),
        }
    }
}
