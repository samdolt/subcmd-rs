## Status

[![Build Status](https://img.shields.io/travis/samdolt/subcmd-rs/master.svg?label=Linux%20%2F%20OS%20X%20build)](https://travis-ci.org/samdolt/subcmd-rs)
[![App Veyor Status](https://img.shields.io/appveyor/ci/samdolt/subcmd-rs/master.svg?label=Windows%20build)](https://ci.appveyor.com/project/samdolt/subcmd-rs)
![Rust min version](https://img.shields.io/badge/Rust-%3E%3D%201.9-blue.svg)
[![Crates.io version](https://img.shields.io/crates/v/subcmd.svg)](https://crates.io/crates/subcmd/)
[![Clippy Linting Result](https://clippy.bashy.io/github/samdolt/subcmd-rs/master/badge.svg)](https://clippy.bashy.io/github/samdolt/subcmd-rs/master/log)

## Cargo style subcommand

This library help to build an app that use a similar command line interface
as Cargo or Git:

```bash
$ myproject build --with --some --option
$ myproject clean
$ myproject --help
```

i.e. Automaticaly pass argv to a corresponding subcommand (here build or clean).

## Feature

- Subcommand parser and runner
- Autogenerated help for `myproject --help` and `myproject -h`
- Hint when a command with a typo is typing
- Colored error message in Linux and OS X.

## Futur plans

- [x] Subcommand help with `myproject help subcommand`
- [ ] Search for `myproject-cmd-subcommand` in the $PATH if there is no built-in subcommand.
- [ ] Allow project wide option like `myproject --verbose clean` instead of `myproject clean --verbose`


