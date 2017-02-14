// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use arrakis::error::Error as ArError;
use iron::headers::ContentLength;
use iron::modifiers::Header;
use iron::status::Status;
use iron::prelude::Set;
use iron::Response;
use serde_json::Value as JsonValue;
use serde_json::Map as JsonMap;

pub fn arrakis_error_to_status(ar_err: ArError) -> Status {
    match ar_err {
        ArError::InvalidFilter(..) => Status::BadRequest,
        ArError::InvalidFilterType(..) => Status::BadRequest,
        ArError::InvalidFilterSyntax(..) => Status::BadRequest,
        ArError::InvalidColumnType(..) => Status::BadRequest,
        ArError::NotFound(..) => Status::NotFound,
        ArError::UnknowModel(..) => Status::BadRequest,
        ArError::UnknowColumn(..) => Status::BadRequest,
        ArError::InternalError(..) => Status::InternalServerError,
        _ => Status::BadRequest,
    }
}

pub fn make_success_response(value: JsonValue) -> Vec<u8> {
    let mut map = JsonMap::new();
    map.insert(String::from("data"), value);
    let value = JsonValue::Object(map);
    ::serde_json::ser::to_vec(&value).unwrap()
}

pub fn make_error_response(estr: &str) -> Vec<u8> {
    let mut map = JsonMap::new();
    map.insert("error".into(), JsonValue::String(estr.into()));
    let value = JsonValue::Object(map);
    ::serde_json::ser::to_vec(&value).unwrap()
}

pub fn make_arrakis_response(ar_res: Result<Option<JsonValue>, ArError>) -> (Vec<u8>, Status) {
    match ar_res {
        Ok(jv) => match jv {
            Some(v) => (make_success_response(v), Status::Ok),
            None => (vec![], Status::NoContent),
        },
        Err(e) => (make_error_response(&*format!("{}", e)), arrakis_error_to_status(e))
    }
}

pub fn write_arrakis_response(ar_res: Result<Option<JsonValue>, ArError>) -> Response {
    let (body, code) = make_arrakis_response(ar_res);
    return write_response(&*body, code)
}

pub fn write_error_response(estr: &str, code: Status) -> Response {
    let body = make_error_response(estr);
    return write_response(&*body, code);
}

fn write_response(body: &[u8], code: Status) -> Response {
    let mut res = Response::with((code, body));
    res = res.set(Header(ContentLength(body.len() as u64)));
    return res;
}
