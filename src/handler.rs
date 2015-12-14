use ::Command;

pub struct Handler<'a> {
    description: Option<&'a str>,
    subcmd: Vec<Box<Command>>,
}

impl<'a> Handler<'a> {

    pub fn new() -> Handler<'a> {
        Handler {
            description: None,
            subcmd: Vec::new(),
        }
    }

    pub fn set_description(&mut self, descr: &'a str) {
        self.description = Some(descr);
    }

    pub fn run(&self){

    }

}

