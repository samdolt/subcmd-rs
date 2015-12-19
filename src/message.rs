// Copyright © 2015 - Samuel Dolt <samuel@dolt.ch>
//
// Licensed under the MIT license. This file may not be copied, modified,
// or distributed except according to those terms.
//
// See the COPYRIGHT file at the top-level directory of this distribution.

use ansi_term::Colour::Red;

/// A Message to be printed
///
/// ```
/// use subcmd::Message;
/// let mut msg = Message::new();
/// msg.add("Some text");
/// msg.print();
/// ```
pub struct Message {
    text: String,
    is_error: bool,
    formated: bool,
}

impl Message {
    /// Return a new Message
    pub fn new() -> Message {
        Message {
            text: String::with_capacity(160),
            is_error: false,
            formated: cfg!(target_os = "macos") || cfg!(target_os = "linux"),
        }
    }

    /// Return a copy of the internal string
    pub fn get(&self) -> String {
        self.text.clone()
    }

    /// Return a formated (colorized) copy of the internal string
    pub fn getf(&self) -> String {
        if self.is_error && self.formated {
            Red.paint(self.get()).to_string()
        } else {
            self.get()
        }
    }

    /// Print the formated string on stdout
    pub fn print(&self) {
        println!("{}", self.getf());
    }

    /// Add a string to the internal buffer
    pub fn add(&mut self, txt: &str) {
        self.text.push_str(txt);
    }

    /// Add a string to the internal buffer and a `\n` character
    pub fn add_line(&mut self, line: &str) {
        self.add(line);
        self.add("\n");
    }

    /// Return true if formated is enabled
    pub fn is_formated(&self) -> bool {
        self.formated
    }

    /// Return true if this is a error message
    pub fn is_error(&self) -> bool {
        self.is_error
    }

    /// Set or unset error flag
    pub fn set_error(&mut self, state: bool) {
        self.is_error = state;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_message_new() {
        // New message are by default not an error, and empty
        // Formated should be off on windows and enabled on Linux and Mac
        let msg = Message::new();

        assert_eq!(msg.get(), "");
        assert_eq!(msg.is_error(), false);

        if cfg!(target_os = "linux") {
            assert_eq!(msg.is_formated(), true);
        } else if cfg!(target_os = "macos") {
            assert_eq!(msg.is_formated(), true);
        } else if cfg!(target_os = "windows") {
            assert_eq!(msg.is_formated(), false);
        } else {
            assert_eq!(msg.is_formated(), false);
        }
    }

    #[test]
    fn test_message_add() {
        let mut msg = Message::new();
        msg.add("Some text");
        assert_eq!(msg.get(), "Some text");

        let mut msg = Message::new();
        msg.add_line("Some new line");
        assert_eq!(msg.get(), "Some new line\n");
    }

    #[test]
    fn test_message_set_error() {
        let mut msg = Message::new();

        msg.set_error(true);
        assert_eq!(msg.is_error(), true);

        msg.set_error(false);
        assert_eq!(msg.is_error(), false);
    }
}
