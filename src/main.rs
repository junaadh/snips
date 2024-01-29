use std::{env, process};

mod lsp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = env::args().count();
    let inputs = &args[1..];
    let mut help = env::args();
    help.next();
    let help = help.next().unwrap_or_default();
    if help.contains("-h") || help.contains("--help") {
        lsp::help();
    }
    if argc <= 2 {
        process::exit(1);
    }
    lsp::run(inputs);
}
