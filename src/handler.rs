// Copyright © 2015 - Samuel Dolt <samuel@dolt.ch>
//
// Licensed under the MIT license. This file may not be copied, modified,
// or distributed except according to those terms.
//
// See the COPYRIGHT file at the top-level directory of this distribution.


use Command;
use std::env;
use getopts::Options;
use getopts::ParsingStyle;

/// Command line parser and subcommand runner
///
/// # Example
///
/// ```ignore
/// let mut handler = Handler::new();
///
/// // Add your custom command
/// handler.add(Box::new(MyCommand));
/// handler.add(Box::new(AnotherCommand));
///
/// handler.run(); // Run main logic
/// ```
pub struct Handler<'a> {
    description: Option<&'a str>,
    subcmd: Vec<Box<Command>>,
}

impl<'a> Handler<'a> {
    /// Create a new `Handler`
    pub fn new() -> Handler<'a> {
        Handler {
            description: None,
            subcmd: Vec::new(),
        }
    }

    /// Set a one line description, used in `bin --help`
    pub fn set_description(&mut self, descr: &'a str) {
        self.description = Some(descr);
    }

    /// Register a new subcommand
    pub fn add(&mut self, command: Box<Command>) {
        self.subcmd.push(command);
    }

    /// Main logic
    ///
    /// This function retrieve argv, parse-it and run the corresponding
    /// subcommand
    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.run_with_args(&args)
    }

    fn print_usage(&self, program: &str, opts: &Options) {
        let brief = format!("Usage: {} [options] <command>", program);
        print!("{}", opts.usage(&brief));
    }

    /// Run the main logic without auto retrieving of argv
    pub fn run_with_args(&self, args: &Vec<String>) {
        let program = args[0].clone();
        let mut opts = Options::new();

        // We don't want to parse option after the subcommand
        opts.parsing_style(ParsingStyle::StopAtFirstFree);

        opts.optflag("h", "help", "print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };

        if matches.opt_present("h") {
            self.print_usage(&program, &opts);
        }

        let command = if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            self.print_usage(&program, &opts);
            return;
        };

        for cmd in self.subcmd.iter() {
            if cmd.name() == command {
                cmd.run(&args);
                return;
            }
        }

        self.print_usage(&program, &opts);
    }
}
