// Copyright © 2015-2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Command;
use Message;
use CmdWrapper;
use CmdResult;

use std::env;
use std::io::Write;
use getopts::Options;
use getopts::ParsingStyle;
use tabwriter::TabWriter;
use strsim::damerau_levenshtein;

/// Command line parser and subcommand runner
///
/// # Example
///
/// ```
/// use subcmd::CmdHandler;
/// CmdHandler::new()
/// // Add your custom command here
///          //.add(Box::new(MyCommand));
///          // .add(Box::new(AnotherCommand));
///             .run(); // Run main logic
/// ```
pub struct CmdHandler<'a> {
    description: Option<&'a str>,
    commands: Vec<Box<Command>>,
    program_name: String,
    args: Vec<String>,
}

impl<'a> CmdHandler<'a> {
    /// Create a new `CmdHandler`
    pub fn new() -> CmdHandler<'a> {
        let args: Vec<String> = env::args().collect();
        let program_name = args[0].clone();

        CmdHandler {
            description: None,
            commands: Vec::new(),
            program_name: program_name,
            args: args,
        }
    }

    /// Set a one line description, used in `bin --help`
    pub fn set_description<'b>(mut self, descr: &'a str) -> CmdHandler<'a> {
        self.description = Some(descr);
        self
    }

    /// Override default args
    pub fn override_args(mut self, args: Vec<String>) -> CmdHandler<'a> {
        self.args = args;
        self
    }

    /// Register a new subcommand
    pub fn add<'b>(mut self, command: Box<Command>) -> CmdHandler<'a> {
        self.commands.push(command);
        self
    }

    fn short_usage(&self) -> String {
        let mut usage = String::with_capacity(150);
        usage.push_str("Usage:\n");
        usage.push_str(&format!("\t{} <command> [<args>...]\n", self.program_name));
        usage.push_str(&format!("\t{} [options]", self.program_name));

        usage
    }

    fn help(&self, opts: &Options) -> CmdResult {
        let mut brief = String::with_capacity(250);
        let mut msg = Message::new();

        match self.description {
            Some(descr) => brief.push_str(&format!("{}\n\n", descr)),
            None => {}
        }

        brief.push_str(&self.short_usage());


        msg.add_line(&opts.usage(&brief));
        msg.add_line("Commands are:");

        let mut tw = TabWriter::new(Vec::new());
        for cmd in self.commands.iter() {
            write!(&mut tw, "    {}\t{}\n", cmd.name(), cmd.description()).unwrap();
        }
        tw.flush().unwrap();

        msg.add_line(&String::from_utf8(tw.unwrap()).unwrap());

        msg.add_line(&format!("\nSee '{} help <command>' for more information ",
                              self.program_name));
        msg.add_line("on a specific command.");

        CmdResult::Help(msg)
    }

    fn bad_usage(&self) -> CmdResult {
        let mut msg = Message::new();
        msg.set_error(true);

        msg.add_line("Invalid arguments.");
        msg.add_line(&self.short_usage());
        CmdResult::BadUsage(msg)
    }


    /// Run the main logic
    pub fn parse(mut self) -> CmdResult {
        let mut opts = Options::new();

        // We don't want to parse option after the subcommand
        opts.parsing_style(ParsingStyle::StopAtFirstFree);

        opts.optflag("h", "help", "print this help menu");

        // args[0] is the program name
        let matches = match opts.parse(&self.args[1..]) {
            Ok(m) => m,
            Err(_) => {
                // Raise on a unknow flag like `--unknow`
                return self.bad_usage();
            }
        };

        // Catch a -h/--help request
        if matches.opt_present("h") {
            // -h/--help don't allow other options/args
            if matches.free.len() != 0 {
                return self.bad_usage();
            }
            return self.help(&opts);
        }

        // Catch the command
        let command = if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            return self.bad_usage();
        };

        // Try to find the command
        for index in 0..self.commands.len() {
            if self.commands[index].name() == command {
                let wrap = CmdWrapper::new(self.commands.remove(index), self.args);
                return CmdResult::Cmd(wrap);
            }
        }

        // Check built-in command
        if (command == "help") && (matches.free.len() == 2) {
            return self.help_for_command(&matches.free[1]);
        }


        // No command found, check for similariy
        let mut sim_cmd: Option<&Box<Command>> = None;
        // We only want command with a similarity lowest than 3
        let mut lowest_sim: usize = 3;
        for cmd in self.commands.iter() {
            let new_sim = damerau_levenshtein(cmd.name(), &command);
            if new_sim < lowest_sim {
                lowest_sim = new_sim;
                sim_cmd = Some(cmd);
            }
        }

        match sim_cmd {
            Some(cmd) => {
                let mut msg = Message::new();
                msg.set_error(true);
                msg.add_line("No such subcommand\n");
                msg.add_line(&format!("    Did you mean `{}`?", cmd.name()));
                return CmdResult::BadUsage(msg);
            }
            None => {}
        };

        let mut msg = Message::new();
        msg.set_error(true);
        msg.add_line("No such subcommand");

        CmdResult::UnknowCmd(msg)
    }

    /// Parse and run the requested command
    pub fn run(self) {
        match self.parse() {
            CmdResult::Help(msg) => msg.print(),
            CmdResult::HelpForCmd(cmd) => cmd.print_help(),
            CmdResult::BadUsage(msg) => msg.print(),
            CmdResult::UnknowCmd(msg) => msg.print(),
            CmdResult::Cmd(cmd) => cmd.run(),
        }
    }

    fn help_for_command(&mut self, name: &str) -> CmdResult {
        for index in 0..self.commands.len() {
            if self.commands[index].name() == name {
                let wrap = CmdWrapper::new(self.commands.remove(index), self.args.clone());
                return CmdResult::HelpForCmd(wrap);
            };
        }

        self.bad_usage()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Command;
    use CmdResult;

    struct CmdA;

    impl Command for CmdA {
        fn name<'a>(&self) -> &'a str {
            "cmd-a"
        }
        fn help<'a>(&self) -> &'a str {
            "HELP"
        }
        fn description<'a>(&self) -> &'a str {
            "DESCR"
        }
        fn run(&self, argv: &Vec<String>) {
            // DO NOTHING
        }
    }

    struct AnotherCmd;

    impl Command for AnotherCmd {
        fn name<'a>(&self) -> &'a str {
            "another-cmd"
        }
        fn help<'a>(&self) -> &'a str {
            "HELP another"
        }
        fn description<'a>(&self) -> &'a str {
            "DESCR another"
        }
        fn run(&self, argv: &Vec<String>) {
            // DO NOTHING
        }
    }

    #[test]
    fn test_usage() {
        let args: Vec<String> = vec!["bin".to_string(), "-h".to_string()];

        match CmdHandler::new().override_args(args).parse() {
            CmdResult::Help(msg) => {
                assert!(msg.get().contains("Usage"));
                assert!(msg.get().contains("Commands are:"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_bad_usage() {
        let args: Vec<String> = vec!["bin".to_string(), "--unknow".to_string()];

        match CmdHandler::new().override_args(args).parse() {
            CmdResult::BadUsage(msg) => {
                assert!(msg.get().contains("Invalid argument"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_bad_command() {
        let args: Vec<String> = vec!["bin".to_string(), "cmd-b".to_string()];

        let handler = CmdHandler::new()
            .override_args(args)
            .add(Box::new(CmdA))
            .add(Box::new(AnotherCmd));

        match handler.parse() {
            CmdResult::BadUsage(msg) => {
                assert!(msg.get().contains("cmd-a"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_unknow_cmd() {
        let args: Vec<String> = vec!["bin".to_string(), "bbbbbbbbbbb".to_string()];

        let handler = CmdHandler::new()
            .override_args(args)
            .add(Box::new(CmdA))
            .add(Box::new(AnotherCmd));

        match handler.parse() {
            CmdResult::UnknowCmd(msg) => assert!(msg.get().contains("No such subcommand")),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_cmd() {
        let args: Vec<String> = vec!["bin".to_string(), "cmd-a".to_string()];

        let handler = CmdHandler::new()
            .override_args(args)
            .add(Box::new(CmdA))
            .add(Box::new(AnotherCmd));

        match handler.parse() {
            CmdResult::Cmd(cmd) => assert_eq!(cmd.name(), "cmd-a"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_help_for_cmd() {
        let args: Vec<String> = vec!["bin".to_string(), "help".to_string(), "cmd-a".to_string()];

        let handler = CmdHandler::new()
            .override_args(args)
            .add(Box::new(CmdA))
            .add(Box::new(AnotherCmd));

        match handler.parse() {
            CmdResult::HelpForCmd(cmd) => assert_eq!(cmd.name(), "cmd-a"),
            _ => unreachable!(),
        }
    }

}
