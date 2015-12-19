// Copyright © 2015 - Samuel Dolt <samuel@dolt.ch>
//
// Licensed under the MIT license. This file may not be copied, modified,
// or distributed except according to those terms.
//
// See the COPYRIGHT file at the top-level directory of this distribution.


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
/// let mut handler = CmdHandler::new();
///
/// // Add your custom command here
/// // handler.add(Box::new(MyCommand));
/// // handler.add(Box::new(AnotherCommand));
///
/// handler.run(); // Run main logic
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
    pub fn set_description(&mut self, descr: &'a str) {
        self.description = Some(descr);
    }

    /// Get the program description
    pub fn get_description(&mut self) -> &'a str {
        match self.description {
            Some(descr) => descr,
            None => "",
        }
    }

    /// Override default args
    pub fn override_args(&mut self, args: Vec<String>) {
        self.args = args;
    }

    /// Register a new subcommand
    pub fn add(&mut self, command: Box<Command>) {
        self.commands.push(command);
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
    pub fn run(mut self) -> CmdResult {
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
        let mut handler = CmdHandler::new();
        let args: Vec<String> = vec!["bin".to_string(), "-h".to_string()];

        handler.override_args(args);

        match handler.run() {
            CmdResult::Help(msg) => {
                assert!(msg.get().contains("Usage"));
                assert!(msg.get().contains("Commands are:"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_bad_usage() {
        let mut handler = CmdHandler::new();
        let args: Vec<String> = vec!["bin".to_string(), "--unknow".to_string()];

        handler.override_args(args);

        match handler.run() {
            CmdResult::BadUsage(msg) => {
                assert!(msg.get().contains("Invalid argument"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_bad_command() {
        let mut handler = CmdHandler::new();
        let args: Vec<String> = vec!["bin".to_string(), "cmd-b".to_string()];

        handler.override_args(args);
        handler.add(Box::new(CmdA));
        handler.add(Box::new(AnotherCmd));

        match handler.run() {
            CmdResult::BadUsage(msg) => {
                assert!(msg.get().contains("cmd-a"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_unknow_cmd() {
        let mut handler = CmdHandler::new();
        let args: Vec<String> = vec!["bin".to_string(), "bbbbbbbbbbb".to_string()];

        handler.override_args(args);
        handler.add(Box::new(CmdA));
        handler.add(Box::new(CmdA));

        match handler.run() {
            CmdResult::UnknowCmd(msg) => assert!(msg.get().contains("No such subcommand")),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_cmd() {
        let mut handler = CmdHandler::new();
        let args: Vec<String> = vec!["bin".to_string(), "cmd-a".to_string()];

        handler.override_args(args);
        handler.add(Box::new(CmdA));
        handler.add(Box::new(CmdA));

        match handler.run() {
            CmdResult::Cmd(cmd) => assert_eq!(cmd.name(), "cmd-a"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_help_for_cmd() {
        let mut handler = CmdHandler::new();
        let args: Vec<String> = vec!["bin".to_string(), "help".to_string(), "cmd-a".to_string()];

        handler.override_args(args);
        handler.add(Box::new(CmdA));
        handler.add(Box::new(CmdA));

        match handler.run() {
            CmdResult::HelpForCmd(cmd) => assert_eq!(cmd.name(), "cmd-a"),
            _ => unreachable!(),
        }
    }

}
