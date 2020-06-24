use clap::{App, ArgMatches, SubCommand};
use crate::context::Context;

pub mod accounts;

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .subcommand(accounts::cli())
}

pub fn exec(ctx: &Context, args: &ArgMatches) {
    let (cmd, subcommand_args) = match args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => return
    };

    let cmd = match cmd {
        "accounts" => accounts::exec,
        _ => return
    };
    cmd(ctx, subcommand_args);
}