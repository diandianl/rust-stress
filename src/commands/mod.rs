pub mod show;

use crate::context::Context;
use clap::{App, ArgMatches};

pub fn app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("stress")
        .subcommand(
            show::cli()
        );
    app
}

pub fn execute(ctx: &Context, cmd: &str, subcommand_args: &ArgMatches<'_>) {
    match cmd {
        "show" => show::exec(ctx, subcommand_args),
        _ => return
    }
}