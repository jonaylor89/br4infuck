
use std::{fs, env};
use br4infuck::evaluate;

fn main() {
    let path = match env::args().nth(1) {
        Some(x) => x,
        _ => return
    };

    let code_string = match fs::read_to_string(path) {
        Ok(x) => x,
        _ => return
    };

    evaluate(code_string);
}
