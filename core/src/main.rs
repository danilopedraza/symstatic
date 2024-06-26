use std::fs;
use std::process::ExitCode;

use symstatic::error::error_msg;
use symstatic::repl::{repl, MyCLI};
use symstatic::run::run;

fn run_smtc(args: &[String]) -> ExitCode {
    if args.len() == 1 {
        repl(&mut MyCLI::default());
        ExitCode::SUCCESS
    } else {
        let path = &args[1];
        let input_res = fs::read_to_string(path);

        match input_res {
            Ok(input) => {
                let res = run(&input);
                if let Err(err) = res {
                    error_msg(&err).emit(path, &input);
                    ExitCode::FAILURE
                } else {
                    ExitCode::SUCCESS
                }
            }
            Err(err) => {
                let msg = err.to_string();
                eprintln!("Error reading {path}: {msg}");
                ExitCode::FAILURE
            }
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    run_smtc(&args)
}
