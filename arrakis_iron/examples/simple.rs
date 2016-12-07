// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate arrakis_iron;
extern crate iron;

use arrakis_iron::ArrakisHandler;
use iron::Iron;

fn main() {
    let pq_addr = ::std::env::args().nth(1).unwrap();
    Iron::new(
        ArrakisHandler::new(&pq_addr).unwrap()
    ).http("localhost:1492").unwrap();
}
