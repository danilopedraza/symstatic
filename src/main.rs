mod ast;
mod builtin;
mod env;
mod eval;
mod file;
mod lexer;
mod object;
mod parser;
mod semantic;

use builtin::standard_env;
use eval::exec;
use file::parse_file;
use lexer::build_lexer;
use parser::parser_from;
use semantic::postprocess;

use std::io::{stdin, stdout, BufRead, Write};

fn eval_line(line: &str) -> String {
    let lexer = build_lexer(line);
    let mut parser = parser_from(lexer.map(|res| res.unwrap()));
    match parser.next() {
        None => String::from(""),
        Some(Ok(node)) => exec(&postprocess(node), &mut standard_env())
            .unwrap()
            .to_string(),
        Some(Err(_)) => String::from("error"),
    }
}

fn repl() -> std::io::Result<()> {
    let mut handle = stdin().lock();
    loop {
        let mut line = String::new();
        print!(">>> ");
        stdout().flush().unwrap();
        handle.read_line(&mut line)?;
        let res = eval_line(&line);
        println!("{res}");
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl()?;
    } else {
        let nodes = parse_file(&args[1]);
        for node in nodes {
            exec(&node, &mut standard_env()).unwrap();
        }
    }

    Ok(())
}
