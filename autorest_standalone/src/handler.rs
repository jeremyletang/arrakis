// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use autorest::AutoRest;
use autorest::queries::Queries;
use autorest::method::Method as ArMethod;
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::server::{Handler, Request, Response};
use hyper::uri::RequestUri;
use std::io::Read;
use response::{write_ar_response, write_error_response};

pub struct AutoRestHandler {
    ar: AutoRest,
}

impl AutoRestHandler {
    pub fn new(ar: AutoRest) -> AutoRestHandler {
        return AutoRestHandler {
            ar: ar,
        }
    }
}

impl Handler for AutoRestHandler {
    fn handle(&self, mut req: Request, res: Response) {
        let body = read_body(&mut req);
        match req.uri {
            RequestUri::AbsolutePath(s) => {
                let (model, queries) = parse_queries(&*s);
                let model = model.trim_matches('/');
                match hyper_method_to_autorest_method(&req.method) {
                    Some(m) => {
                        let ar_res = match m {
                            ArMethod::Get => self.ar.get(model, &queries),
                            ArMethod::Post => self.ar.post(model, &queries, body),
                            ArMethod::Put => self.ar.put(model, &queries, body),
                            ArMethod::Patch => self.ar.patch(model, &queries, body),
                            ArMethod::Delete => self.ar.delete(model, &queries),
                        };
                        write_ar_response(res, ar_res);
                    },
                    None => {
                        let estr = format!("method not allowed {}", &req.method);
                        write_error_response(res, &*estr, StatusCode::MethodNotAllowed);
                    }
                };
            },
            _ => write_error_response(res, "unable to parse url", StatusCode::BadRequest)
        };
    }
}

fn parse_queries(path: &str) -> (&str, Queries) {
    match path.find('?') {
        Some(pos) => {
            let (begin, end) = path.split_at(pos+1);
            let path = &(begin[..begin.len()-1]);
            match end.len() {
                0 => (path, Default::default()),
                _ => {
                    (path, end.split('&').collect::<Vec<&str>>().iter().map(|&s| {
                        match s.find('=') {
                            Some(pos) => {
                                let (b, e) = s.split_at(pos+1);
                                (&(b[..b.len()-1]), e)
                            },
                            None => (s, "")
                        }
                    }).collect::<Queries>())
                }
            }
        },
        None => (path, Default::default())
    }
}

fn hyper_method_to_autorest_method(m: &Method) -> Option<ArMethod> {
    match m {
        &Method::Get => Some(ArMethod::Get),
        &Method::Post => Some(ArMethod::Post),
        &Method::Put => Some(ArMethod::Put),
        &Method::Patch => Some(ArMethod::Patch),
        &Method::Delete => Some(ArMethod::Delete),
        _ => None,
    }
}

fn read_body(req: &mut Request) -> String {
    let mut buf = String::new();
    let _ = req.read_to_string(&mut buf);
    return buf;
}
