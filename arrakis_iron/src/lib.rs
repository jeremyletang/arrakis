// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate arrakis;
extern crate iron;
extern crate urlencoded;
extern crate serde;
extern crate serde_json;

mod response;

use arrakis::Arrakis;
use arrakis::method::Method as ArrakisMethod;
use arrakis::queries::queries_from_hashmap;
use iron::{Handler, IronResult, Request, Response, Url, Plugin};
use iron::request::Body;
use iron::method::Method;
use iron::status::Status;
use response::{write_arrakis_response, write_error_response};
use std::io::Read;
use urlencoded::UrlEncodedQuery;
use std::collections::HashMap;

pub use arrakis::config::Config;

pub struct ArrakisHandler {
    ar: Arrakis
}

impl ArrakisHandler {
    pub fn new(pq_addr: &str) -> Result<ArrakisHandler, String> {
        match Arrakis::new(pq_addr) {
            Ok(a) => Ok(ArrakisHandler {ar: a}),
            Err(e) => Err(format!("{}", e))
        }
    }

    pub fn with_config(pq_addr: &str, config: Config) -> Result<ArrakisHandler, String> {
        match Arrakis::with_config(pq_addr, config) {
            Ok(a) => Ok(ArrakisHandler {ar: a}),
            Err(e) => Err(format!("{}", e))
        }
    }
}

impl Handler for ArrakisHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let body = read_body(&mut req.body);
        let model = extract_model_from_url(&req.url);

        let qmap = match req.get::<UrlEncodedQuery>() {
            Ok(hashmap) => hashmap,
            Err(_) =>
                // return Ok(response::write_error_response(
                // &format!("unable to read queries: {}", e),
                //    Status::InternalServerError)),
                HashMap::new(),
        };
        let queries = queries_from_hashmap(&qmap);

        match arrakis_of_iron_method(&req.method) {
            Some(m) => Ok(write_arrakis_response(self.ar.any(&m, &*model, &queries, body))),
            None => {
                let estr = format!("method not allowed {}", &req.method);
                Ok(write_error_response(&*estr, Status::MethodNotAllowed))
            }
        }
    }
}

fn arrakis_of_iron_method(m: &Method) -> Option<ArrakisMethod> {
    match m {
        &Method::Get => Some(ArrakisMethod::Get),
        &Method::Post => Some(ArrakisMethod::Post),
        &Method::Put => Some(ArrakisMethod::Put),
        &Method::Patch => Some(ArrakisMethod::Patch),
        &Method::Delete => Some(ArrakisMethod::Delete),
        _ => None,
    }
}

fn extract_model_from_url(url: &Url) -> String {
    return url.path().last().unwrap().to_string();
}

fn read_body<'a, 'b>(body: &mut Body<'a, 'b>) -> String {
    let mut buf = String::new();
    let _ = body.read_to_string(&mut buf);
    return buf;
}
