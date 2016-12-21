// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use arrakis::error::Error as ArError;
use hyper::header::ContentLength;
use hyper::status::StatusCode;
use hyper::server::Response as HttpResponse;
use serde_json::Value as JsonValue;
use serde_json::Map as JsonMap;
use std::io::Write;

pub fn ar_error_to_status_code(ar_err: ArError) -> StatusCode {
    match ar_err {
        ArError::InternalError(..) => StatusCode::InternalServerError,
        ArError::NotFound(..) => StatusCode::NotFound,
        _ => StatusCode::BadRequest,
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

pub fn make_ar_response(ar_res: Result<Option<JsonValue>, ArError>) -> (Vec<u8>, StatusCode) {
    match ar_res {
        Ok(jv) => match jv {
            Some(v) => (make_success_response(v), StatusCode::Ok),
            None => (vec![], StatusCode::NoContent),
        },
        Err(e) => (make_error_response(&*format!("{}", e)), ar_error_to_status_code(e))
    }
}

pub fn write_ar_response(res: HttpResponse, ar_res: Result<Option<JsonValue>, ArError>) {
    let (body, code) = make_ar_response(ar_res);
    write_response(res, &*body, code);
}

pub fn write_error_response(res: HttpResponse, estr: &str, code: StatusCode) {
    let body = make_error_response(estr);
    write_response(res, &*body, code);
}

fn write_response(mut res: HttpResponse, body: &[u8], code: StatusCode) {
    res.headers_mut().set(ContentLength(body.len() as u64));
    *res.status_mut() = code;
    let mut res = res.start().unwrap();
    res.write_all(body).unwrap();
}
