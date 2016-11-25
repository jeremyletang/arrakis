// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(plugin)]
#![plugin(log)]

extern crate autorest;
extern crate clap;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate time as std_time;

use autorest::AutoRest;
use clap::{App, Arg};
use handler::AutoRestHandler;
use hyper::Server;
use metrics::Metrics;

mod handler;
mod response;
mod metrics;

const DEFAULT_HTTP_ADDR: &'static str = "0.0.0.0:1492";

struct CmdLineArgs {
    pub addr: String,
    pub pq_addr: String,
    pub disable_metrics: bool,
}

fn parse_cmdline() -> CmdLineArgs {
    let matches = App::new("autorest_standalone")
        .version("v0.1.0")
        .global_setting(clap::AppSettings::ColoredHelp)
        .about("automatic generation of your rest api from your database schema")
        .arg(Arg::with_name("addr")
             .long("addr")
             .help("http address of the server")
             .default_value(DEFAULT_HTTP_ADDR))
        .arg(Arg::with_name("pq-addr")
             .long("pq-addr")
             .help("postgres server address")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("disable-metrics")
             .long("disable-metrics")
             .help("disable metrics logging middleware"))
        .get_matches();

    CmdLineArgs {
        addr: matches.value_of("addr").unwrap().into(),
        pq_addr: matches.value_of("pq-addr").unwrap().into(),
        disable_metrics: matches.is_present("disable-metrics"),
    }
}

fn main() {
    let _ = env_logger::init();
    let args = parse_cmdline();
    info!("starting autorest server at {}", &*args.addr);
    let auto = AutoRest::new(&*args.pq_addr).unwrap();
    let handler = AutoRestHandler::new(auto);
    if !args.disable_metrics {
        let handler = Metrics::new(handler);
        Server::http(&*args.addr).unwrap().handle(handler).unwrap();
    } else {
        Server::http(&*args.addr).unwrap().handle(handler).unwrap();
    }
}
