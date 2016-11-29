// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::time::Duration;

const DEFAULT_CONNECTION_TIMEOUT: u64 = 5;

#[derive(Debug, Clone)]
pub struct Config<'r> {
    timeout_: u64,
    excluded_: Vec<&'r str>,
    included_: Vec<&'r str>,
}

impl<'r> Config<'r> {
    pub fn builder() -> Builder<'r> {
        Builder {
            config: Config::default(),
        }
    }

    pub fn timeout(&self) -> u64 {
        self.timeout_
    }

    pub fn excluded(& self) -> &[&str] {
        &*self.excluded_
    }

    pub fn included(&self) -> &[&str] {
        &*self.included_
    }
}

impl<'r> Default for Config<'r> {
    fn default() -> Config<'r> {
        Config {
            timeout_: DEFAULT_CONNECTION_TIMEOUT,
            excluded_: vec![],
            included_: vec![],
        }
    }
}

pub struct Builder<'r> {
    config: Config<'r>,
}

impl<'r> Builder<'r> {
    pub fn build(self) -> Config<'r> {
        self.config
    }

    pub fn timeout(mut self, timeout: u64) -> Builder<'r> {
        self.config.timeout_ = timeout;
        self
    }

    pub fn excluded(mut self, excluded: Vec<&'r str>) -> Builder<'r> {
        self.config.excluded_ = excluded;
        self
    }

    pub fn included(mut self, included: Vec<&'r str>) -> Builder<'r> {
        self.config.included_ = included;
        self
    }
}
