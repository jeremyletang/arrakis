// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::{Future, BoxFuture};
use hyper;
use hyper::server::{Service, Request, Response};
use std_time::precise_time_ns;

pub struct Metrics<S> {
    service: S,
}

impl<S> Metrics<S> {
    pub fn new(service: S) -> Metrics<S> {
        Metrics {
            service: service,
        }
    }
}

impl<S> Service for Metrics<S>
    where S: Service<Request=Request, Response=Response, Error=hyper::Error>,
          <S as hyper::client::Service>::Future: Send + 'static {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let before = precise_time_ns();
        let addr = req.remote_addr().clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        self.service.call(req)
            .and_then(move |res| {
                let delta = precise_time_ns() - before;
                info!("request from {} to {} {} in {} ms",
                      addr,
                      method,
                      uri,
                      (delta as f64) / 1000000.0);
                Ok(res)
            }).boxed()
    }
}
