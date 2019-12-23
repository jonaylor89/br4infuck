
use std::{fs, env};
use br4infuck::evaluate;

fn main() {
    let path = match env::args().nth(1) {
        Some(x) => x,
        _ => {
            println!("[-] Must specify a path to a file containing brainfuck code");
            return
        }
    };

    let code_string = match fs::read_to_string(path) {
        Ok(x) => x,
        _ => {
            println!("[-] Unable to read in file");
            return
        }
    };

    evaluate(code_string);
}
