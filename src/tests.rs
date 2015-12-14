// Copyright © 2015 - Samuel Dolt <samuel@dolt.ch>
//
// Licensed under the MIT license. This file may not be copied, modified,
// or distributed except according to those terms.
//
// See the COPYRIGHT file at the top-level directory of this distribution.

use super::*;

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

#[test]
fn it_works() {

    let mut handler = Handler::new();

    handler.add(Box::new(CmdA));

    let mut args: Vec<String> = Vec::new();
    args.push("my-bin".to_string());
    args.push("cmd-a".to_string());
    handler.run_with_args(&args);
}
