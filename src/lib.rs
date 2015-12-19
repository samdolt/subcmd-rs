// Copyright © 2015 - Samuel Dolt <samuel@dolt.ch>
//
// Licensed under the MIT license. This file may not be copied, modified,
// or distributed except according to those terms.
//
// See the COPYRIGHT file at the top-level directory of this distribution.

#![warn(missing_docs)]

//! Cargo style subcommand
//!
//! This library help to build an app that use a similar command line interface
//! as Cargo or Git.
//!
//! # Example
//!
//! The following example show a simple program with two subcommand:
//!
//! - `cargo build`
//! - `cargo clean`
//!
//! ```
//! extern crate subcmd;
//! use subcmd::CmdHandler;
//! use subcmd::Command;
//!
//! struct CmdBuild;
//!
//! impl Command for CmdBuild {
//!     fn name<'a>(&self) -> &'a str {"build"}
//!     fn help<'a>(&self) -> &'a str {"Usage: cargo build [options]"}
//!     fn description<'a>(&self) -> &'a str { "Compile the current project" }
//!     fn run(&self, argv: &Vec<String>) {
//!         println!("I'm building your files");
//!     }
//! }
//!
//! struct CmdClean;
//!
//! impl Command for CmdClean {
//!     fn name<'a>(&self) -> &'a str {"clean"}
//!     fn help<'a>(&self) -> &'a str {"Usage: cargo clean [options]"}
//!     fn description<'a>(&self) -> &'a str { "Remove the target directory" }
//!     fn run(&self, argv: &Vec<String>) {
//!         println!("I'm cleaning your files");
//!     }
//! }
//!
//! fn main() {
//!     let mut handler = CmdHandler::new();
//!     handler.add(Box::new(CmdBuild));
//!     handler.add(Box::new(CmdClean));
//!     handler.run();
//! }
//! ```

extern crate getopts;
extern crate tabwriter;
extern crate strsim;
extern crate ansi_term;

mod cmd_handler;
pub use cmd_handler::CmdHandler;


/// This trait must be implemented for each subcommand
pub trait Command {
    /// This fonction must return the command line, without space. Like
    /// `build`, `clean`, ...
    fn name<'a>(&self) -> &'a str;

    /// Return the help message of a subcommand. Used for `bin help subcommand`
    fn help<'a>(&self) -> &'a str;

    /// Return a one line description. Used for the program help `bin -h`
    fn description<'a>(&self) -> &'a str;

    /// Main entry point. argv contains all argument passed to the binary,
    /// with the program name in argv[0]
    fn run(&self, argv: &Vec<String>);
}

#[cfg(test)]
mod tests;
