// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use arrakis::Arrakis;
use arrakis::queries::Queries;
use arrakis::method::Method as ArrakisMethod;
use futures::{Stream, Future};
use futures::future::BoxFuture;
use hyper::status::StatusCode;
use hyper::{self, Method};
use hyper::server::{NewService, Service, Request, Response};
use response::{write_arrakis_response, write_error_response};

#[derive(Clone)]
pub struct ArrakisHandler {
    ar: Arrakis,
}

impl ArrakisHandler {
    pub fn new(ar: Arrakis) -> ArrakisHandler {
        return ArrakisHandler {
            ar: ar,
        }
    }
}

impl Service for ArrakisHandler {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        // read body
        let arrakis = self.ar.clone();
        let (method, uri, _, _, _body) = req.deconstruct();
        _body.fold(vec![], move |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
        }).and_then(move |v| {
            let body: String = unsafe { String::from_utf8_unchecked(v.clone()) };
            let queries = parse_queries(uri.query().unwrap_or(""));
            let model = uri.path().trim_matches('/');
            match arrakis_of_hyper_method(&method) {
                Some(m) => Ok(write_arrakis_response(arrakis.any(&m, model, &queries, body))),
                None => {
                    let estr = format!("method not allowed {}", &method);
                    Ok(write_error_response(&*estr, StatusCode::MethodNotAllowed))
                }
            }
        }).boxed()
    }
}

impl NewService for ArrakisHandler {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = ArrakisHandler;

    fn new_service(&self) -> ::std::io::Result<ArrakisHandler> {
        Ok(self.clone())
    }
}

fn parse_queries(queries: &str) -> Queries {
    match queries.len() {
        0 => Default::default(),
        _ => {
            queries.split('&').collect::<Vec<&str>>().iter().map(|&s| {
                match s.find('=') {
                    Some(pos) => {
                        let (b, e) = s.split_at(pos+1);
                        (&(b[..b.len()-1]), e)
                    },
                    None => (s, "")
                }
            }).collect::<Queries>()
        }
    }
}

fn arrakis_of_hyper_method(m: &Method) -> Option<ArrakisMethod> {
    match m {
        &Method::Get => Some(ArrakisMethod::Get),
        &Method::Post => Some(ArrakisMethod::Post),
        &Method::Put => Some(ArrakisMethod::Put),
        &Method::Patch => Some(ArrakisMethod::Patch),
        &Method::Delete => Some(ArrakisMethod::Delete),
        _ => None,
    }
}
