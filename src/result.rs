// Copyright © 2015-2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Message;
use CmdWrapper;

/// Result of a CmdHandler::run
///
/// # Usage
///
/// ```
/// use subcmd::CmdHandler;
/// use subcmd::CmdResult;
///
/// let handler = CmdHandler::new();
/// match handler.run() {
///     CmdResult::Help(msg)           => msg.print(),
///     CmdResult::HelpForCmd(cmd)     => cmd.print_help(),
///     CmdResult::BadUsage(msg)       => msg.print(),
///     CmdResult::UnknowCmd(msg)      => msg.print(),
///     CmdResult::Cmd(cmd)            => cmd.run(),
/// }
/// ```
pub enum CmdResult {
    /// Help has been requested with `-h` or `--help`
    Help(Message),

    /// Help for a command has been requested with `help cmd`
    HelpForCmd(CmdWrapper),

    /// A unknow option like `--unknow-option` has been requested
    BadUsage(Message),

    /// A unknow command like `unknow-command` has been requested
    UnknowCmd(Message),

    /// A know command has been requested
    Cmd(CmdWrapper),
}
