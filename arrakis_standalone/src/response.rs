// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use arrakis::error::Error as ArrakisError;
use hyper::header::ContentLength;
use hyper::status::StatusCode;
use hyper::server::Response as HyperResponse;
use hyper::Body;
use serde_json::Value as JsonValue;
use serde_json::Map as JsonMap;

pub fn arrakis_error_to_status_code(ar_err: ArrakisError) -> StatusCode {
    match ar_err {
        ArrakisError::InternalError(..) => StatusCode::InternalServerError,
        ArrakisError::NotFound(..) => StatusCode::NotFound,
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

pub fn make_arrakis_response(ar_res: Result<Option<JsonValue>, ArrakisError>) -> (Vec<u8>, StatusCode) {
    match ar_res {
        Ok(jv) => match jv {
            Some(v) => (make_success_response(v), StatusCode::Ok),
            None => (vec![], StatusCode::NoContent),
        },
        Err(e) => (make_error_response(&*format!("{}", e)), arrakis_error_to_status_code(e))
    }
}

pub fn write_arrakis_response(ar_res: Result<Option<JsonValue>, ArrakisError>)
                              -> HyperResponse {
    let (body, code) = make_arrakis_response(ar_res);
    let len = body.len();
    write_response(body, len as u64, code)
}

pub fn write_error_response(estr: &str, code: StatusCode)
                            -> HyperResponse {
    let body = make_error_response(estr);
    let len = body.len();
    write_response(body, len as u64, code)
}

fn write_response<T: Into<Body>>(body: T, len: u64, code: StatusCode)
                  -> HyperResponse {
    HyperResponse::new().with_header(ContentLength(len))
        .with_status(code)
        .with_body(body.into())
}
