extern crate clap;

use br4infuck::{compile, evaluate};
use std::fs;

use clap::{App, Arg};

fn main() {
    let matches = App::new("br4infuck")
        .version("0.1.5")
        .author("John Naylor <jonaylor89@gmail.com>")
        .about("brainfuck interpreter and compiler")
        .arg(
            Arg::with_name("file")
                .required(true)
                .takes_value(true)
                .help("brainfuck source code"),
        )
        .arg(
            Arg::with_name("compile")
                .short("c")
                .long("compile")
                .help("Compile brainfuck to x86_64 assembly"),
        )
        .get_matches();

    let path = matches.value_of("file").unwrap();

    let code_string = match fs::read_to_string(path) {
        Ok(x) => x,
        _ => {
            println!("[-] Unable to read in file");
            return;
        }
    };

   if matches.is_present("compile") {
        compile(code_string);
    } else {
        evaluate(code_string);
    }
}
