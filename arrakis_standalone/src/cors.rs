// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::{future, Future, BoxFuture};
use hyper::server::{Service, Request, Response};
use hyper::{self, Method};
use hyper::header::{AccessControlAllowOrigin, ContentLength, CacheControl,
                    AccessControlAllowHeaders, AccessControlAllowMethods,
                    CacheDirective};
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use unicase::UniCase;

pub struct Cors<S> {
    service: S,
}

impl<S> Cors<S>
    where S: Service<Request=Request, Response=Response, Error=hyper::Error>,
          <S as hyper::client::Service>::Future: Send + 'static {
    pub fn new(service: S) -> Cors<S> {
        Cors {
            service: service,
        }
    }

    fn handle_options(& self) -> BoxFuture<Response, hyper::Error> {
        let response = Response::new()
            .with_header(ContentLength(0))
            .with_header(AccessControlAllowOrigin::Any)
            .with_header(
                AccessControlAllowHeaders(vec![
                    UniCase("content-type".to_owned()),
                ])
            )
            .with_header(
                AccessControlAllowMethods(vec![
                    Method::Get, Method::Put, Method::Post,
                    Method::Delete, Method::Options, Method::Patch
                ])
            )
            .with_header(
                ContentType(
                    Mime(TopLevel::Application, SubLevel::Json, vec![])
                )
            )
            .with_header(
                CacheControl(vec![
                    CacheDirective::NoCache,
                ])
            )
            .with_status(StatusCode::Ok);
        future::ok(response).boxed()
    }

    fn handle_others(&self, req: Request) -> BoxFuture<Response, hyper::Error> {
        self.service.call(req)
            .and_then(move |res| {
                Ok(res.with_header(AccessControlAllowOrigin::Any))
            }).boxed()
    }
}

impl<S> Service for Cors<S>
    where S: Service<Request=Request, Response=Response, Error=hyper::Error>,
          <S as hyper::client::Service>::Future: Send + 'static {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req.method() {
            &Method::Options => self.handle_options(),
            _ => self.handle_others(req),
        }
    }
}
