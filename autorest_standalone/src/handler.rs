// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::server::{Handler, Request, Response};
use hyper::header::ContentLength;
use std::io::Write;
use hyper::uri::RequestUri;
use std::collections::HashMap;

pub struct AutoRestHandler;

impl AutoRestHandler {
    pub fn new() -> AutoRestHandler {
        return AutoRestHandler;
    }
}

impl Handler for AutoRestHandler {
    fn handle(&self, req: Request, mut res: Response) {
        match req.uri {
            RequestUri::AbsolutePath(s) => {
                let q = parse_queries(&*s);
                println!("{:?}", q);
            },
            _ => {}
        };


        let body = b"Hello World!";
        res.headers_mut().set(ContentLength(body.len() as u64));
        let mut res = res.start().unwrap();
        res.write_all(body).unwrap();
    }
}


fn parse_queries(path: &str) -> HashMap<&str, &str> {
    match path.find('?') {
        Some(pos) => {
            let (_, end) = path.split_at(pos+1);
            match end.len() {
                0 => Default::default(),
                _ => {
                    end.split('&').collect::<Vec<&str>>().iter().map(|&s| {
                        match s.find('=') {
                            Some(pos) => {
                                let (b, e) = s.split_at(pos+1);
                                (&(b[..b.len()-1]), e)
                            },
                            None => (s, "")
                        }
                    }).collect::<HashMap<&str, &str>>()
                }
            }
        },
        None => Default::default()
    }
}
