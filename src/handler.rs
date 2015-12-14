// Copyright © 2015 - Samuel Dolt <samuel@dolt.ch>
//
// Licensed under the MIT license. This file may not be copied, modified,
// or distributed except according to those terms.
//
// See the COPYRIGHT file at the top-level directory of this distribution.

use Command;

pub struct Handler<'a> {
    description: Option<&'a str>,
    subcmd: Vec<Box<Command>>,
}

impl<'a> Handler<'a> {
    pub fn new() -> Handler<'a> {
        Handler {
            description: None,
            subcmd: Vec::new(),
        }
    }

    pub fn set_description(&mut self, descr: &'a str) {
        self.description = Some(descr);
    }

    pub fn add(&mut self, command: Box<Command>) {
        self.subcmd.push(command);
    }

    pub fn run(&self) {}
}
