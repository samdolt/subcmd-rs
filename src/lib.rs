// Copyright © 2015 - Samuel Dolt <samuel@dolt.ch>
//
// Licensed under the MIT license. This file may not be copied, modified,
// or distributed except according to those terms.
//
// See the COPYRIGHT file at the top-level directory of this distribution.

mod handler;
pub use handler::Handler;

/// Small wrapper arround String.
pub struct CmdName {
    text: String,
}

impl CmdName {
    /// Create a new command name based on the given string
    ///
    /// Note: Command name must contain no space and only
    /// a small subset of ascii. (A-Z a-z)
    pub fn new(name: &str) -> Option<CmdName> {
        None
    }
}

pub trait Command {
    fn name(&self) -> CmdName;
    fn help<'a>(&self) -> &'a str;
    fn description<'a>(&self) -> &'a str;
    fn run(&self, argv: &Vec<String>);
}

#[cfg(test)]
mod tests;
