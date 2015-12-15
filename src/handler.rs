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
    program: String,
}

impl<'a> Handler<'a> {
    /// Create a new `Handler`
    pub fn new() -> Handler<'a> {
        Handler {
            description: None,
            subcmd: Vec::new(),
            program: String::with_capacity(30),
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
    pub fn run(&mut self) {
        let args: Vec<String> = env::args().collect();
        self.run_with_args(&args)
    }

    fn short_usage(&self) -> String {
        // Here we should have a program name
        debug_assert!(self.program.len() > 0);

        let mut usage = String::with_capacity(150);
        usage.push_str("Usage:\n");
        usage.push_str(&format!("\t{} <command> [<args>...]\n", self.program));
        usage.push_str(&format!("\t{} [options]", self.program));

        usage
    }

    fn print_usage(&self, opts: &Options) {
        let mut brief = String::with_capacity(250);

        match self.description {
            Some(descr) => brief.push_str(&format!("{}\n\n", descr)),
            None        => {},
        }

        brief.push_str(&self.short_usage());

        println!("{}", opts.usage(&brief));

        println!("Commands are:");

        let cmd_name_max_len = {
            let mut max_len = 0;

            for cmd in self.subcmd.iter() {
                if cmd.name().len() > max_len {
                    max_len = cmd.name().len();
                }
            }

            max_len
        };

        for cmd in self.subcmd.iter() {
            let mut name = cmd.name().to_string();

            // Alignement
            while name.len() < (cmd_name_max_len + 6) {
                name.push(' ');
            }

            println!("    {}{}",name, cmd.description());
        }

        print!("\nSee '{} help <command>' for more information ", self.program);
        println!("on a specific command.");
    }

    fn bad_usage(&self) {
        println!("Invalid arguments.\n");
        println!("{}", self.short_usage());
    }

    /// Run the main logic without auto retrieving of argv
    pub fn run_with_args(&mut self, args: &Vec<String>) {
        self.program = args[0].clone();

        let mut opts = Options::new();

        // We don't want to parse option after the subcommand
        opts.parsing_style(ParsingStyle::StopAtFirstFree);

        opts.optflag("h", "help", "print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                self.bad_usage();
                return;
            },
        };

        // Catch a -h/--help request
        if matches.opt_present("h") {
            // -h/--help don't allow other options/args
            if matches.free.len() != 0 {
                self.bad_usage();
                return;
            }
            self.print_usage(&opts);
            return;
        }

        // Catch the command
        let command = if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            self.bad_usage();
            return;
        };

        // Run the command
        for cmd in self.subcmd.iter() {
            if cmd.name() == command {
                cmd.run(&args);
                return;
            }
        }

        // Check built-in command
        if (command == "help") && (matches.free.len() == 2) {
            self.help_for_command(&matches.free[1]);
            return;
        }

        self.bad_usage();
    }


    fn help_for_command(&self, name: &str) {
        for cmd in self.subcmd.iter() {
            if cmd.name() == name {
                println!("{}", cmd.help());
                return;
            };
        };
    }
}
