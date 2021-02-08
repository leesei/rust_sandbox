use std::env;

mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    let input = env::args().nth(1).expect("please input module to run");
    match input.as_str() {
        "arrays" => arrays::run(),
        "cli" => cli::run(),
        "conditionals" => conditionals::run(),
        "enums" => enums::run(),
        "functions" => functions::run(),
        "loops" => loops::run(),
        "pointer_ref" => pointer_ref::run(),
        "print" => print::run(),
        "strings" => strings::run(),
        "structs" => structs::run(),
        "tuples" => tuples::run(),
        "types" => types::run(),
        "vars" => vars::run(),
        "vectors" => vectors::run(),
        _ => println!("unknown: {}", input),
    }
}
