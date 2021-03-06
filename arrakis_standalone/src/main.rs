// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(slice_patterns)]

extern crate arrakis;
extern crate clap;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_json;
extern crate time as std_time;
extern crate unicase;

use arrakis::Arrakis;
use arrakis::config::Config;
use cors::Cors;
use clap::{App, Arg};
use service::{ArrakisService, Conf};
use hyper::server::Http;
use metrics::Metrics;
use hyper::server::NewService;

mod cors;
mod metrics;
mod response;
mod service;

const DEFAULT_HTTP_ADDR: &'static str = "0.0.0.0:1492";

struct CmdLineArgs {
    pub addr: String,
    pub pq_addr: String,
    pub disable_metrics: bool,
    pub include: Option<String>,
    pub exclude: Option<String>,
    pub with_docs: bool,
}

fn parse_cmdline() -> CmdLineArgs {
    let matches = App::new("arrakis_standalone")
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
        .arg(Arg::with_name("include")
             .long("include")
             .help("specify which tables should be included from the auto generated api")
             .takes_value(true))
        .arg(Arg::with_name("exclude")
             .long("exclude")
             .help("specify which tables should be excluded from the auto generated api")
             .takes_value(true))
        .arg(Arg::with_name("disable-metrics")
             .long("disable-metrics")
             .help("disable metrics logging middleware"))
        .arg(Arg::with_name("with-docs")
             .long("with-docs")
             .help("with builtin docs endpoint"))
        .get_matches();

    CmdLineArgs {
        addr: matches.value_of("addr").unwrap().into(),
        pq_addr: matches.value_of("pq-addr").unwrap().into(),
        disable_metrics: matches.is_present("disable-metrics"),
        include: matches.value_of("include").map_or(None, |s| Some(s.into())),
        exclude: matches.value_of("exclude").map_or(None, |s| Some(s.into())),
        with_docs: matches.is_present("with-docs"),
    }
}

fn split_list(l: Option<&String>) -> Vec<&str> {
    l.map_or(vec![], |ref s| s.split(",").collect::<Vec<&str>>()).iter()
        .filter_map(|s| if s.is_empty() { None } else { Some(*s) }).collect()
}

fn main() {
    let _ = pretty_env_logger::init();
    let args = parse_cmdline();

    let config = Config::builder()
        .timeout(1)
        .excluded(split_list(args.exclude.as_ref()))
        .included(split_list(args.include.as_ref()))
        .build();

    let arrakis = match Arrakis::with_config(&*args.pq_addr, config) {
        Ok(auto) => auto,
        Err(e) => { println!("error: {}", e); return; },
    };

    info!("this instance will manage the following tables: {}",
          arrakis.get_tables().iter().map(|(t, _)| &**t).collect::<Vec<&str>>().join(", "));
    let arrakis_service = ArrakisService::with_conf(arrakis, Conf{with_docs: args.with_docs});
    let cors = move || Ok(Cors::new(arrakis_service.new_service().unwrap()));

    info!("Arrakis listening on http://{}", args.addr);
    if !args.disable_metrics {
        let metrics = move || Ok(Metrics::new(cors().unwrap()));
        Http::new().bind(&(args.addr.parse().unwrap()), metrics).unwrap().run().unwrap();
    } else {
        Http::new().bind(&(args.addr.parse().unwrap()), cors).unwrap().run().unwrap();
    };
}
