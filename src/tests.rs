// Copyright © 2015 - Samuel Dolt <samuel@dolt.ch>
//
// Licensed under the MIT license. This file may not be copied, modified,
// or distributed except according to those terms.
//
// See the COPYRIGHT file at the top-level directory of this distribution.

use super::*;

struct CmdA;

impl Command for CmdA {
    fn name(&self) -> CmdName {
        CmdName::new("CmdA").unwrap()
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

#[test]
fn it_works() {

    let cmd1: Box<Command> = Box::new(CmdA);
    let mut handler = Handler::new();

    handler.add(cmd1);

    handler.run();
}
