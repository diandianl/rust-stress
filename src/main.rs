use std::panic;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use stress::commands::{app, execute};
use stress::context::Context;

fn main() {

    let app = app();
    let ctx = Context::new();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    /*
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
     */
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.len() <= 0 {
                    continue;
                }
                println!("Input: {}", line);

                let mut line = line;
                line.insert_str(0, "stress ");

                let matches = match app.clone().get_matches_from_safe(line.split(' ')) {
                    Ok(args) => args,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue
                    }
                };
                let (cmd, subcommand_args) = matches.subcommand();
                // rl.add_history_entry(line.as_str());
                println!("Execute Command: {}", cmd);
                panic::catch_unwind(panic::AssertUnwindSafe(|| {
                    execute(&ctx, cmd, subcommand_args.unwrap());
                })).ok();
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // rl.save_history("history.txt").unwrap();
}

