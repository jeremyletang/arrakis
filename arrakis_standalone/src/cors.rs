// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::server::{Handler, Request, Response};
use hyper::method::Method;
use hyper::header::{AccessControlAllowOrigin, ContentLength, CacheControl,
                    AccessControlAllowHeaders, AccessControlAllowMethods,
                    CacheDirective};
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use unicase::UniCase;

pub struct Cors<H> {
    handler: H,
}

impl<H> Cors<H>
    where H: Handler {
    pub fn new(handler: H) -> Cors<H> {
        Cors {
            handler: handler,
        }
    }

    fn handle_options<'h, 'a>(&'h self, _: Request<'h, 'a>, mut res: Response<'h>) {
        res.headers_mut().set(ContentLength(0));
        res.headers_mut().set(AccessControlAllowOrigin::Any);
        res.headers_mut().set(
            AccessControlAllowHeaders(vec![
                UniCase("content-type".to_owned()),
            ])
        );
        res.headers_mut().set(
            AccessControlAllowMethods(vec![
                Method::Get, Method::Put, Method::Post,
                Method::Delete, Method::Options, Method::Patch
            ])
        );
        res.headers_mut().set(
            ContentType(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            )
        );
        res.headers_mut().set(
            CacheControl(vec![
                CacheDirective::NoCache,
            ])
        );
        *res.status_mut() = StatusCode::Ok;
    }

    fn handle_others<'h, 'a>(&'h self, req: Request<'h, 'a>, mut res: Response<'h>) {
        res.headers_mut().set(AccessControlAllowOrigin::Any);
        res.headers_mut().set(
            ContentType(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            )
        );
        self.handler.handle(req, res);
    }

}

impl<H> Handler for Cors<H>
    where H: Handler {
    fn handle<'h, 'a>(&'h self, req: Request<'h, 'a>, res: Response<'h>) {
        match req.method {
            Method::Options => self.handle_options(req, res),
            _ => self.handle_others(req, res),
        }
    }
}
