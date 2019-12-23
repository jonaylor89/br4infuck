use std;
use std::io::{self, Read};

pub fn evaluate(program: String) {

    let tape_length = 100;

    let mut buffer = String::new();

    let mut tape: Vec<u8> = vec![0; tape_length];
    let mut ptr = 0;
    let mut is_looping = false;
    let mut loop_stack = vec![];
    let mut inner_loops = 0;

    let mut i = 0;
    
    while i < program.len() {

        let c = program.chars().nth(i).unwrap();

        if is_looping {
            if c == '[' { inner_loops += 1; }
            if c == ']' { 
                if inner_loops == 0 { is_looping = false; }
                else { inner_loops -= 1; }
            }

            i += 1;
            continue;
        }

        match c {
            '+' => tape[ptr] += 1,
            '-' => tape[ptr] -= 1,
            '>' => {
                ptr += 1;
            },
            '<' => {
                ptr -= 1;
            },
            '.' => print!("{}", tape[ptr] as char),
            ',' => {
                io::stdin().read_to_string(&mut buffer);
                tape[ptr] = buffer.chars().next().unwrap() as u8;
            },
            ']' => {
                if tape[ptr] != 0 {
                    i = loop_stack[loop_stack.len()-1]; 
                } else { 
                    loop_stack.pop(); 
                }
            },
            '[' => {
                if tape[ptr] == 0 {
                    is_looping = true;
                } else {
                    loop_stack.push(i);
                }
            },
            _ => {},
        }

        i += 1;
    }
}
