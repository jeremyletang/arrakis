// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::server::{Handler, Request, Response};
use std_time::precise_time_ns;

pub struct Metrics<H> {
    handler: H,
}

impl<H> Metrics<H>
    where H: Handler {
    pub fn new(handler: H) -> Metrics<H> {
        Metrics {
            handler: handler,
        }
    }
}

impl<H> Handler for Metrics<H>
    where H: Handler {
    fn handle<'h, 'a>(&'h self, req: Request<'h, 'a>, res: Response<'h>) {
        let before = precise_time_ns();
        let addr = req.remote_addr.clone();
        let method = req.method.clone();
        let uri = req.uri.clone();
        self.handler.handle(req, res);
        let delta = precise_time_ns() - before;
        info!("request from {} to {} {} in {} ms",
              addr,
              method,
              uri,
              (delta as f64) / 1000000.0);
    }
}
