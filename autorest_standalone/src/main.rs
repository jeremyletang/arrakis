// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(plugin)]
#![plugin(log)]

extern crate autorest;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate time as std_time;

use autorest::AutoRest;
use handler::AutoRestHandler;
use hyper::Server;
use metrics::Metrics;

mod handler;
mod response;
mod metrics;

fn main() {
    let _ = env_logger::init();
    let ar = AutoRest::new("postgresql://root:root@192.168.99.100:6432/giistr", "giistr").unwrap();
    let arh = Metrics::new(AutoRestHandler::new(ar));
    info!("starting autorest server at 0.0.0.0:1492");
    Server::http("0.0.0.0:1492").unwrap().handle(arh).unwrap();
}
