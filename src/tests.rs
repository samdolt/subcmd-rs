
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
    fn run(&self, argv: &Vec<String>){
        // DO NOTHING
    }
}

#[test]
fn it_works() {

    let handler = Handler::new();

    handler.run();
}
