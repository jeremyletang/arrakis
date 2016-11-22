// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate hyper;
extern crate autorest;

use handler::AutoRestHandler;
use hyper::Server;
use autorest::AutoRest;

mod handler;

fn main() {
    let ar = match AutoRest::new("postgresql://root:root@192.168.99.100:6432/giistr", "giistr") {
        Ok(ar) => ar,
        Err(e) => {
            println!("AutoRest error: {}", e);
            return
        }
    };
    let arh = AutoRestHandler::new(ar);
    Server::http("0.0.0.0:1492").unwrap().handle(arh).unwrap();
}
