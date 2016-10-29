// Copyright © 2015-2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use Command;

/// This wrapper hold a command object and a arguments vectors.
pub struct CmdWrapper {
    cmd: Box<Command>,
    args: Vec<String>,
}

impl CmdWrapper {
    /// Create a new wrapper
    pub fn new(cmd: Box<Command>, args: Vec<String>) -> CmdWrapper {
        CmdWrapper {
            cmd: cmd,
            args: args,
        }
    }

    /// Get the name of the wrapped command
    pub fn name<'a>(&self) -> &'a str {
        self.cmd.name()
    }

    /// Get a string with help info
    pub fn help<'a>(&self) -> &'a str {
        self.cmd.help()
    }

    /// Print the help of the wrapper command
    pub fn print_help(&self) {
        println!("{}", self.cmd.help());
    }

    /// Run the command
    pub fn run(&self) {
        self.cmd.run(&self.args);
    }

    /// Return the embedded command
    pub fn unwrap(self) -> Box<Command> {
        self.cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeCmd;

    static mut FakeCmdRunCalled: bool = false;

    impl Command for FakeCmd {
        fn name<'a>(&self) -> &'a str {
            "fake"
        }
        fn help<'a>(&self) -> &'a str {
            "help for fake"
        }
        fn description<'a>(&self) -> &'a str {
            "descr. for fake"
        }
        fn run(&self, argv: &Vec<String>) {
            unsafe {
                FakeCmdRunCalled = true;
            }
            assert_eq!(argv[0], "test");
        }
    }

    #[test]
    fn test_cmd_wrapper() {
        let wrap = CmdWrapper::new(Box::new(FakeCmd), vec!["test".to_string()]);

        assert_eq!(wrap.name(), "fake");
        assert_eq!(wrap.help(), "help for fake");

        wrap.run();
        unsafe {
            assert_eq!(FakeCmdRunCalled, true);
        }

        let fake = wrap.unwrap();
        assert_eq!(fake.description(), "descr. for fake");
    }
}
