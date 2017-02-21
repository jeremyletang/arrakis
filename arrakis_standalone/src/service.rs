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
use hyper::header::{ContentType, ContentLength};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::{self, Method};
use hyper::status::StatusCode;
use hyper::server::{NewService, Service, Request, Response};
use response::{write_arrakis_response, write_error_response};

#[derive(Debug, Default, Clone)]
pub struct Conf {
    pub with_docs: bool,
}

#[derive(Debug, Clone)]
pub struct ArrakisService {
    ar: Arrakis,
    conf: Conf,
}

impl ArrakisService {
    pub fn new(arrakis: Arrakis) -> ArrakisService {
        return ArrakisService {
            ar: arrakis,
            conf: Default::default(),
        }
    }

    pub fn with_conf(arrakis: Arrakis, conf: Conf) -> ArrakisService {
        return ArrakisService {
            ar: arrakis,
            conf: conf,
        }
    }
}

impl Service for ArrakisService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        // read body
        let arrakis = self.ar.clone();
        let conf = self.conf.clone();
        let (method, uri, _, _, _body) = req.deconstruct();
        _body.fold(vec![], move |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
        }).and_then(move |v| {
            let body: String = unsafe { String::from_utf8_unchecked(v.clone()) };
            let queries = parse_queries(uri.query().unwrap_or(""));
            let path = uri.path().trim_matches('/').split("/").collect::<Vec<&str>>();
            match *path {
                ["builtins", builtin] => Ok(serve_builtins(builtin, arrakis, conf)),
                ["api", model] => match arrakis_of_hyper_method(&method) {
                    Some(m) => Ok(write_arrakis_response(arrakis.any(&m, model, &queries, body))),
                    None => {
                        let estr = format!("method not allowed {}", &method);
                        Ok(write_error_response(&*estr, StatusCode::MethodNotAllowed))
                    }
                },
                _ => Ok(write_error_response("not found", StatusCode::NotFound))
            }
        }).boxed()
    }
}

impl NewService for ArrakisService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = ArrakisService;

    fn new_service(&self) -> ::std::io::Result<ArrakisService> {
        Ok(self.clone())
    }
}

fn execute_docs_builtins(arrakis: Arrakis) -> Response {
    let response_body = arrakis.make_doc();
    let len = response_body.len() as u64;
    Response::new()
        .with_header(ContentLength(len))
        .with_header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])))
        .with_status(StatusCode::Ok)
        .with_body(response_body)

}

fn serve_builtins(builtin: &str, arrakis: Arrakis, conf: Conf) -> Response {
    match builtin {
        "docs" => execute_docs_builtins(arrakis),
        _ => {
            let estr = format!("unknown builtin {}", builtin);
            write_error_response(&*estr, StatusCode::BadRequest)
        }
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
