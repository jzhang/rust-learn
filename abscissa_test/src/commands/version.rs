//! `version` subcommand

#![allow(clippy::never_loop)]

use super::AbscissaTestCommand;
use abscissa_core::{Command, Options, Runnable};

/// `version` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct VersionCommand {}

impl Runnable for VersionCommand {
    /// Print version message
    fn run(&self) {
        println!(
            "{} {}",
            AbscissaTestCommand::name(),
            AbscissaTestCommand::version()
        );
    }
}
